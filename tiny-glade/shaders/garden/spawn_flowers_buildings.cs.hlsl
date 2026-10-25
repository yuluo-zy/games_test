// PATCHED: modules gs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/garden/spawn_flowers_buildings.cs.hlsl
// 花朵生成(建筑区):两个抖动放置点;排除测试;garden_mask.y>0.3 跳过;
// buildings_mask.y 落在 0.55 带(0.3/0.4 宽度);buildings_non_blurred>0.99999 跳过;
// perlin(p*0.5)>0.6 且 buildings.x>0 时放置 FlowerXform(命令 11,type 0/1)。
// 依据:_garden_spawn_flowers_buildings.cs.hlsl_cs_main_0.bin 的 spirv-cross。
// 状态:O (算法取自 SPIR-V);I (阈值)

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
    // 主/次两个放置点:抖动世界坐标,过排除与建筑遮罩带后放置花朵。
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _garden_spawn_flowers_buildings.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,编号变量已统一清理)
// 计算着色器:建筑区花朵生成(逻辑同重建段,算术为 oracle 逐字)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct Affine3Packed
{
    float data[12];
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

static uint v123;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 buildings_mask_dims : packoffset(c2);
    uint2 buildings_mask_non_blurred_dims : packoffset(c2.z);
    int2 grid_dims : packoffset(c3);
    uint2 path_mask_dims : packoffset(c3.z);
    uint2 rocky_terrain_dims : packoffset(c4);
    uint2 water_mask_dims : packoffset(c4.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c5);
};

RWByteAddressBuffer __alias_flower_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t21, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> foliage_exclusion_mask : register(t24, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 v208;
    float2 v232;
    float2 v154 = (((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx;
    uint v161 = (((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) / 2u) + (gl_GlobalInvocationID.x * 20u)) + 100u;
    uint v163 = v161 + (v161 << 10u);
    uint v165 = v163 ^ (v163 >> 6u);
    uint v167 = v165 + (v165 << 3u);
    uint v169 = v167 ^ (v167 >> 11u);
    float v175 = asfloat(((v169 + (v169 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 v178 = v154 + lerp(-0.75f, 0.75f, v175).xx;
    uint v181 = uint((v175 * 100.0f) - 341.0f);
    uint v183 = v181 + (v181 << 10u);
    uint v185 = v183 ^ (v183 >> 6u);
    uint v187 = v185 + (v185 << 3u);
    uint v189 = v187 ^ (v187 >> 11u);
    float2 v198 = v154 + lerp(-0.75f, 0.75f, asfloat(((v189 + (v189 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    int2 v199 = int2(gl_GlobalInvocationID.xy);
    uint v598;
    uint v599;
    bool v600;
    do
    {
        float2 v205;
        uint2 v210;
        bool v271;
        do
        {
            v205 = (v178 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            v208 = float2(path_mask_dims);
            v210 = uint2(v205 * v208);
            float4 v228 = terrain_heightmap.GatherRed(sampler_default, v205, int2(0, 0));
            v232 = 130.0f.xx / float2(terrain_heightmap_dims);
            float v233 = v228.z;
            float v234 = v228.y;
            float v236 = v228.w;
            float v237 = v228.x;
            bool v258;
            if (!(path_mask.Load(int3(v210, 0u)).x > 0.20000000298023223876953125f))
            {
                v258 = (int(rocky_terrain.Load(int3(v210, 0u)).x > 0.0f) & int(length(((float2((v233 + v234) - (v236 + v237), (v237 + v234) - (v236 + v233)) * 0.5f.xx) * 26.5f) / v232) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v258 = true;
            }
            bool v263;
            if (!v258)
            {
                v263 = water_mask.Load(int3(v210, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v263 = true;
            }
            bool v268;
            if (!v263)
            {
                v268 = foliage_exclusion_mask.SampleLevel(sampler_llb, v205, 0.0f).x > 0.0f;
            }
            else
            {
                v268 = true;
            }
            if (v268)
            {
                v271 = true;
                break;
            }
            v271 = false;
            break;
        } while(false);
        if (v271)
        {
            v598 = v123;
            v599 = v123;
            v600 = false;
            break;
        }
        if (garden_mask.Load(int3(v210, 0u)).y > 0.300000011920928955078125f)
        {
            v598 = v123;
            v599 = v123;
            v600 = false;
            break;
        }
        float4 v282 = buildings_mask.SampleLevel(sampler_default, v205, 0.0f);
        float v298;
        do
        {
            float v288 = abs(v282.y - 0.550000011920928955078125f);
            if (v288 > 0.300000011920928955078125f)
            {
                v298 = 0.0f;
                break;
            }
            float v292 = v288 * 3.3333332538604736328125f;
            v298 = 1.0f - ((v292 * v292) * (3.0f - (v288 * 6.666666507720947265625f)));
            break;
        } while(false);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, v205, 0.0f).x > 0.999989986419677734375f)
        {
            v598 = v123;
            v599 = v123;
            v600 = false;
            break;
        }
        float2 v306 = v178 * 0.5f;
        int2 v308 = int2(floor(v306));
        float2 v309 = frac(v306);
        uint2 v310 = uint2(v308);
        uint v312 = v310.y;
        uint v314 = v312 + (v312 << 10u);
        uint v316 = v314 ^ (v314 >> 6u);
        uint v318 = v316 + (v316 << 3u);
        uint v320 = v318 ^ (v318 >> 11u);
        uint v326 = (((v310.x * 1664525u) + (v320 + (v320 << 15u))) + 1013904223u) * 1664525u;
        uint v328 = v326 ^ (v326 >> 11u);
        uint v331 = v328 ^ ((v328 << 7u) & 2636928640u);
        uint v334 = v331 ^ ((v331 << 15u) & 4022730752u);
        uint2 v342 = uint2(v308 + int2(1, 0));
        uint v344 = v342.y;
        uint v346 = v344 + (v344 << 10u);
        uint v348 = v346 ^ (v346 >> 6u);
        uint v350 = v348 + (v348 << 3u);
        uint v352 = v350 ^ (v350 >> 11u);
        uint v358 = (((v342.x * 1664525u) + (v352 + (v352 << 15u))) + 1013904223u) * 1664525u;
        uint v360 = v358 ^ (v358 >> 11u);
        uint v363 = v360 ^ ((v360 << 7u) & 2636928640u);
        uint v366 = v363 ^ ((v363 << 15u) & 4022730752u);
        uint2 v374 = uint2(v308 + int2(0, 1));
        uint v376 = v374.y;
        uint v378 = v376 + (v376 << 10u);
        uint v380 = v378 ^ (v378 >> 6u);
        uint v382 = v380 + (v380 << 3u);
        uint v384 = v382 ^ (v382 >> 11u);
        uint v390 = (((v374.x * 1664525u) + (v384 + (v384 << 15u))) + 1013904223u) * 1664525u;
        uint v392 = v390 ^ (v390 >> 11u);
        uint v395 = v392 ^ ((v392 << 7u) & 2636928640u);
        uint v398 = v395 ^ ((v395 << 15u) & 4022730752u);
        uint2 v406 = uint2(v308 + int2(1, 1));
        uint v408 = v406.y;
        uint v410 = v408 + (v408 << 10u);
        uint v412 = v410 ^ (v410 >> 6u);
        uint v414 = v412 + (v412 << 3u);
        uint v416 = v414 ^ (v414 >> 11u);
        uint v422 = (((v406.x * 1664525u) + (v416 + (v416 << 15u))) + 1013904223u) * 1664525u;
        uint v424 = v422 ^ (v422 >> 11u);
        uint v427 = v424 ^ ((v424 << 7u) & 2636928640u);
        uint v430 = v427 ^ ((v427 << 15u) & 4022730752u);
        float2 v440 = (v309 * v309) * (3.0f.xx - (v309 * 2.0f));
        float v441 = v440.x;
        float v442 = 1.0f - v441;
        float v444 = v440.y;
        if ((int(((((1.0f - v444) * (((asfloat(((v334 ^ (v334 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v442) + ((asfloat(((v366 ^ (v366 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v441))) + (((asfloat(((v398 ^ (v398 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v442) * v444)) + (((asfloat(((v430 ^ (v430 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v441) * v444)) > 0.60000002384185791015625f) & int(v282.x > 0.0f)) != 0)
        {
            int v463 = v199.x;
            int v464 = v199.y;
            uint v466 = uint(v463 + v464);
            uint v468 = v466 + (v466 << 10u);
            uint v470 = v468 ^ (v468 >> 6u);
            uint v472 = v470 + (v470 << 3u);
            uint v474 = v472 ^ (v472 >> 11u);
            float v481 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((v474 + (v474 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v484 = v481.xxx * lerp(0.20000000298023223876953125f, 1.0f, v298);
            float v489 = min(v484.x, 0.75f);
            if (v489 < 0.4000000059604644775390625f)
            {
                v598 = 1013904223u;
                v599 = 1664525u;
                v600 = true;
                break;
            }
            float3 v496 = float3(v489, (v484.y * 1.2000000476837158203125f) * v481, min(v484.z, 0.75f)) * 2.0f;
            uint v513 = uint((v463 * v464) + v463);
            uint v515 = v513 + (v513 << 10u);
            uint v517 = v515 ^ (v515 >> 6u);
            uint v519 = v517 + (v517 << 3u);
            uint v521 = v519 ^ (v519 >> 11u);
            float v528 = 3.1400001049041748046875f * (asfloat(((v521 + (v521 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v529 = normalize(float3(0.0f, 1.0f, 0.0f));
            float v530 = sin(v528);
            float v531 = cos(v528);
            float v532 = 1.0f - v531;
            float v533 = v529.x;
            float v534 = v532 * v533;
            float v537 = v529.y;
            float v538 = v534 * v537;
            float v539 = v529.z;
            float v540 = v539 * v530;
            float v542 = v532 * v539;
            float v543 = v542 * v533;
            float v544 = v537 * v530;
            float v547 = v532 * v537;
            float v550 = v547 * v539;
            float v551 = v533 * v530;
            float4x4 v562 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, v178.x), float4(0.0f, 1.0f, 0.0f, 0.5f), float4(0.0f, 0.0f, 1.0f, v178.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(v496.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v496.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v496.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((v534 * v533) + v531, v538 - v540, v543 + v544, 0.0f), float4(v538 + v540, (v547 * v537) + v531, v550 - v551, 0.0f), float4(v543 - v544, v550 + v551, (v542 * v539) + v531, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            float v563 = v562[0].x;
            float v564 = v562[1].x;
            float v565 = v562[2].x;
            float v566 = v562[0].y;
            float v567 = v562[1].y;
            float v568 = v562[2].y;
            float v569 = v562[0].z;
            float v570 = v562[1].z;
            float v571 = v562[2].z;
            float v572 = v562[0].w;
            float v573 = v562[1].w;
            float v574 = v562[2].w;
            uint v591;
            __alias_draw_commands.InterlockedAdd(224, 1u, v591);
            uint v593 = uint(int(v591));
            float v595[12] = { v563, v564, v565, v566, v567, v568, v569, v570, v571, v572, v573 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(v563, v564, v565), float3(v566, v567, v568), float3(v569, v570, v571), float3(v572, v573, v574))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), v574 };
            Affine3Packed v596 = { v595 };
            FlowerXform v597 = { v596, (-1.0f).xxx, 0u };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_flower_buffer.Store<float>(_4ident * 4 + v593 * 64 + 0, v597.transform.data[_4ident]);
            }
            __alias_flower_buffer.Store<float3>(v593 * 64 + 48, v597.color);
            __alias_flower_buffer.Store<uint>(v593 * 64 + 60, v597.flower_type);
        }
        v598 = 1013904223u;
        v599 = 1664525u;
        v600 = true;
        break;
    } while(false);
    do
    {
        float2 v606;
        uint2 v608;
        bool v658;
        do
        {
            v606 = (v198 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            v608 = uint2(v606 * v208);
            float4 v619 = terrain_heightmap.GatherRed(sampler_default, v606, int2(0, 0));
            float v620 = v619.z;
            float v621 = v619.y;
            float v623 = v619.w;
            float v624 = v619.x;
            bool v645;
            if (!(path_mask.Load(int3(v608, 0u)).x > 0.20000000298023223876953125f))
            {
                v645 = (int(rocky_terrain.Load(int3(v608, 0u)).x > 0.0f) & int(length(((float2((v620 + v621) - (v623 + v624), (v624 + v621) - (v623 + v620)) * 0.5f.xx) * 26.5f) / v232) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v645 = true;
            }
            bool v650;
            if (!v645)
            {
                v650 = water_mask.Load(int3(v608, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v650 = true;
            }
            bool v655;
            if (!v650)
            {
                v655 = foliage_exclusion_mask.SampleLevel(sampler_llb, v606, 0.0f).x > 0.0f;
            }
            else
            {
                v655 = true;
            }
            if (v655)
            {
                v658 = true;
                break;
            }
            v658 = false;
            break;
        } while(false);
        if (v658)
        {
            break;
        }
        if (garden_mask.Load(int3(v608, 0u)).y > 0.300000011920928955078125f)
        {
            break;
        }
        float4 v669 = buildings_mask.SampleLevel(sampler_default, v606, 0.0f);
        float v685;
        do
        {
            float v675 = abs(v669.y - 0.550000011920928955078125f);
            if (v675 > 0.4000000059604644775390625f)
            {
                v685 = 0.0f;
                break;
            }
            float v679 = v675 * 2.5f;
            v685 = 1.0f - ((v679 * v679) * (3.0f - (v675 * 5.0f)));
            break;
        } while(false);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, v606, 0.0f).x > 0.999989986419677734375f)
        {
            break;
        }
        float2 v694 = (v198 * 0.5f) + 100.0f.xx;
        int2 v696 = int2(floor(v694));
        float2 v697 = frac(v694);
        uint2 v698 = uint2(v696);
        uint v700 = v698.y;
        uint v702 = v700 + (v700 << 10u);
        uint v704 = v702 ^ (v702 >> 6u);
        uint v706 = v704 + (v704 << 3u);
        uint v708 = v706 ^ (v706 >> 11u);
        bool v711 = v600 ? v600 : true;
        uint v712 = v600 ? v599 : 1664525u;
        uint v713 = v600 ? v598 : 1013904223u;
        uint v717 = (((v698.x * v712) + (v708 + (v708 << 15u))) + v713) * v712;
        uint v719 = v717 ^ (v717 >> 11u);
        uint v722 = v719 ^ ((v719 << 7u) & 2636928640u);
        uint v725 = v722 ^ ((v722 << 15u) & 4022730752u);
        uint2 v733 = uint2(v696 + int2(1, 0));
        uint v735 = v733.y;
        uint v737 = v735 + (v735 << 10u);
        uint v739 = v737 ^ (v737 >> 6u);
        uint v741 = v739 + (v739 << 3u);
        uint v743 = v741 ^ (v741 >> 11u);
        bool v746 = v711 ? v711 : true;
        uint v747 = v711 ? v712 : 1664525u;
        uint v748 = v711 ? v713 : 1013904223u;
        uint v752 = (((v733.x * v747) + (v743 + (v743 << 15u))) + v748) * v747;
        uint v754 = v752 ^ (v752 >> 11u);
        uint v757 = v754 ^ ((v754 << 7u) & 2636928640u);
        uint v760 = v757 ^ ((v757 << 15u) & 4022730752u);
        uint2 v768 = uint2(v696 + int2(0, 1));
        uint v770 = v768.y;
        uint v772 = v770 + (v770 << 10u);
        uint v774 = v772 ^ (v772 >> 6u);
        uint v776 = v774 + (v774 << 3u);
        uint v778 = v776 ^ (v776 >> 11u);
        bool v781 = v746 ? v746 : true;
        uint v782 = v746 ? v747 : 1664525u;
        uint v783 = v746 ? v748 : 1013904223u;
        uint v787 = (((v768.x * v782) + (v778 + (v778 << 15u))) + v783) * v782;
        uint v789 = v787 ^ (v787 >> 11u);
        uint v792 = v789 ^ ((v789 << 7u) & 2636928640u);
        uint v795 = v792 ^ ((v792 << 15u) & 4022730752u);
        uint2 v803 = uint2(v696 + int2(1, 1));
        uint v805 = v803.y;
        uint v807 = v805 + (v805 << 10u);
        uint v809 = v807 ^ (v807 >> 6u);
        uint v811 = v809 + (v809 << 3u);
        uint v813 = v811 ^ (v811 >> 11u);
        uint v816 = v781 ? v782 : 1664525u;
        uint v821 = (((v803.x * v816) + (v813 + (v813 << 15u))) + (v781 ? v783 : 1013904223u)) * v816;
        uint v823 = v821 ^ (v821 >> 11u);
        uint v826 = v823 ^ ((v823 << 7u) & 2636928640u);
        uint v829 = v826 ^ ((v826 << 15u) & 4022730752u);
        float2 v839 = (v697 * v697) * (3.0f.xx - (v697 * 2.0f));
        float v840 = v839.x;
        float v841 = 1.0f - v840;
        float v843 = v839.y;
        if ((int(((((1.0f - v843) * (((asfloat(((v725 ^ (v725 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v841) + ((asfloat(((v760 ^ (v760 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v840))) + (((asfloat(((v795 ^ (v795 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v841) * v843)) + (((asfloat(((v829 ^ (v829 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v840) * v843)) > 0.60000002384185791015625f) & int(v669.x > 0.0f)) != 0)
        {
            int v862 = v199.x;
            int v863 = v199.y;
            uint v865 = uint(v862 + v863);
            uint v867 = v865 + (v865 << 10u);
            uint v869 = v867 ^ (v867 >> 6u);
            uint v871 = v869 + (v869 << 3u);
            uint v873 = v871 ^ (v871 >> 11u);
            float v880 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((v873 + (v873 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v883 = v880.xxx * lerp(0.20000000298023223876953125f, 1.0f, v685);
            float v888 = min(v883.x, 0.75f);
            if (v888 < 0.4000000059604644775390625f)
            {
                break;
            }
            float3 v895 = float3(v888, (v883.y * 1.2000000476837158203125f) * v880, min(v883.z, 0.75f)) * 2.0f;
            uint v912 = uint((v862 * v863) + v862);
            uint v914 = v912 + (v912 << 10u);
            uint v916 = v914 ^ (v914 >> 6u);
            uint v918 = v916 + (v916 << 3u);
            uint v920 = v918 ^ (v918 >> 11u);
            float v927 = 3.1400001049041748046875f * (asfloat(((v920 + (v920 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v928 = normalize(float3(0.0f, 1.0f, 0.0f));
            float v929 = sin(v927);
            float v930 = cos(v927);
            float v931 = 1.0f - v930;
            float v932 = v928.x;
            float v933 = v931 * v932;
            float v936 = v928.y;
            float v937 = v933 * v936;
            float v938 = v928.z;
            float v939 = v938 * v929;
            float v941 = v931 * v938;
            float v942 = v941 * v932;
            float v943 = v936 * v929;
            float v946 = v931 * v936;
            float v949 = v946 * v938;
            float v950 = v932 * v929;
            float4x4 v961 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, v198.x), float4(0.0f, 1.0f, 0.0f, 0.5f), float4(0.0f, 0.0f, 1.0f, v198.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(v895.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v895.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v895.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((v933 * v932) + v930, v937 - v939, v942 + v943, 0.0f), float4(v937 + v939, (v946 * v936) + v930, v949 - v950, 0.0f), float4(v942 - v943, v949 + v950, (v941 * v938) + v930, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            float v962 = v961[0].x;
            float v963 = v961[1].x;
            float v964 = v961[2].x;
            float v965 = v961[0].y;
            float v966 = v961[1].y;
            float v967 = v961[2].y;
            float v968 = v961[0].z;
            float v969 = v961[1].z;
            float v970 = v961[2].z;
            float v971 = v961[0].w;
            float v972 = v961[1].w;
            float v973 = v961[2].w;
            uint v990;
            __alias_draw_commands.InterlockedAdd(224, 1u, v990);
            uint v992 = uint(int(v990));
            float v994[12] = { v962, v963, v964, v965, v966, v967, v968, v969, v970, v971, v972 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(v962, v963, v964), float3(v965, v966, v967), float3(v968, v969, v970), float3(v971, v972, v973))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), v973 };
            Affine3Packed v995 = { v994 };
            FlowerXform v996 = { v995, (-1.0f).xxx, 1u };
            [unroll]
            for (int _5ident = 0; _5ident < 12; _5ident++)
            {
                __alias_flower_buffer.Store<float>(_5ident * 4 + v992 * 64 + 0, v996.transform.data[_5ident]);
            }
            __alias_flower_buffer.Store<float3>(v992 * 64 + 48, v996.color);
            __alias_flower_buffer.Store<uint>(v992 * 64 + 60, v996.flower_type);
        }
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
