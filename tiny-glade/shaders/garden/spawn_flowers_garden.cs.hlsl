// PATCHED: modules en.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED==
// 功能:花园区花朵生成:抖动世界坐标,过排除/花园遮罩带后放置 FlowerXform。 shaders/garden/spawn_flowers_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_flowers_garden.cs.hlsl_cs_main_0.bin (two jittered candidate sites; exclusion; buildings<0.5; garden_mask.y band around 0.6; perlin(world*0.5) gate; FlowerXform (cmd 11, 64B, color=(-1|1), type 0/1)) | status: O (algorithm from SPIR-V); I (thresholds/scale literals)

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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 oracle bin(spirv-cross;行为精确,编号变量已统一清理)
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

static uint v119;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    uint2 buildings_mask_non_blurred_dims : packoffset(c2);
    int2 grid_dims : packoffset(c2.z);
    uint2 path_mask_dims : packoffset(c3);
    uint2 rocky_terrain_dims : packoffset(c3.z);
    uint2 water_mask_dims : packoffset(c4);
    uint2 foliage_exclusion_mask_dims : packoffset(c4.z);
};

RWByteAddressBuffer __alias_flower_buffer : register(u19, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
Texture2D<float4> path_mask : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 v204;
    float2 v228;
    float2 v152 = (((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx;
    uint v157 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
    uint v159 = v157 + (v157 << 10u);
    uint v161 = v159 ^ (v159 >> 6u);
    uint v163 = v161 + (v161 << 3u);
    uint v165 = v163 ^ (v163 >> 11u);
    float v171 = asfloat(((v165 + (v165 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 v174 = v152 + lerp(-0.75f, 0.75f, v171).xx;
    uint v177 = uint((v171 * 100.0f) - 34.0f);
    uint v179 = v177 + (v177 << 10u);
    uint v181 = v179 ^ (v179 >> 6u);
    uint v183 = v181 + (v181 << 3u);
    uint v185 = v183 ^ (v183 >> 11u);
    float2 v194 = v152 + lerp(-0.75f, 0.75f, asfloat(((v185 + (v185 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    int2 v195 = int2(gl_GlobalInvocationID.xy);
    uint v602;
    uint v603;
    bool v604;
    do
    {
        float2 v201;
        bool v267;
        do
        {
            v201 = (v174 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            v204 = float2(path_mask_dims);
            uint2 v206 = uint2(v201 * v204);
            float4 v224 = terrain_heightmap.GatherRed(sampler_default, v201, int2(0, 0));
            v228 = 130.0f.xx / float2(terrain_heightmap_dims);
            float v229 = v224.z;
            float v230 = v224.y;
            float v232 = v224.w;
            float v233 = v224.x;
            bool v254;
            if (!(path_mask.Load(int3(v206, 0u)).x > 0.20000000298023223876953125f))
            {
                v254 = (int(rocky_terrain.Load(int3(v206, 0u)).x > 0.0f) & int(length(((float2((v229 + v230) - (v232 + v233), (v233 + v230) - (v232 + v229)) * 0.5f.xx) * 26.5f) / v228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v254 = true;
            }
            bool v259;
            if (!v254)
            {
                v259 = water_mask.Load(int3(v206, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v259 = true;
            }
            bool v264;
            if (!v259)
            {
                v264 = foliage_exclusion_mask.SampleLevel(sampler_llb, v201, 0.0f).x > 0.0f;
            }
            else
            {
                v264 = true;
            }
            if (v264)
            {
                v267 = true;
                break;
            }
            v267 = false;
            break;
        } while(false);
        if (v267)
        {
            v602 = v119;
            v603 = v119;
            v604 = false;
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, v201, 0.0f).x > 0.5f)
        {
            v602 = v119;
            v603 = v119;
            v604 = false;
            break;
        }
        float4 v279 = garden_mask.SampleLevel(sampler_default, v201, 0.0f);
        float v280 = v279.y;
        float v294;
        do
        {
            float v284 = abs(v280 - 0.60000002384185791015625f);
            if (v284 > 0.20000000298023223876953125f)
            {
                v294 = 0.0f;
                break;
            }
            float v288 = v284 * 5.0f;
            v294 = 1.0f - ((v288 * v288) * (3.0f - (v284 * 10.0f)));
            break;
        } while(false);
        float v298 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, v201, 0.0f).y);
        float2 v301 = v174 * 0.5f;
        int2 v303 = int2(floor(v301));
        float2 v304 = frac(v301);
        uint2 v305 = uint2(v303);
        uint v307 = v305.y;
        uint v309 = v307 + (v307 << 10u);
        uint v311 = v309 ^ (v309 >> 6u);
        uint v313 = v311 + (v311 << 3u);
        uint v315 = v313 ^ (v313 >> 11u);
        uint v321 = (((v305.x * 1664525u) + (v315 + (v315 << 15u))) + 1013904223u) * 1664525u;
        uint v323 = v321 ^ (v321 >> 11u);
        uint v326 = v323 ^ ((v323 << 7u) & 2636928640u);
        uint v329 = v326 ^ ((v326 << 15u) & 4022730752u);
        uint2 v337 = uint2(v303 + int2(1, 0));
        uint v339 = v337.y;
        uint v341 = v339 + (v339 << 10u);
        uint v343 = v341 ^ (v341 >> 6u);
        uint v345 = v343 + (v343 << 3u);
        uint v347 = v345 ^ (v345 >> 11u);
        uint v353 = (((v337.x * 1664525u) + (v347 + (v347 << 15u))) + 1013904223u) * 1664525u;
        uint v355 = v353 ^ (v353 >> 11u);
        uint v358 = v355 ^ ((v355 << 7u) & 2636928640u);
        uint v361 = v358 ^ ((v358 << 15u) & 4022730752u);
        uint2 v369 = uint2(v303 + int2(0, 1));
        uint v371 = v369.y;
        uint v373 = v371 + (v371 << 10u);
        uint v375 = v373 ^ (v373 >> 6u);
        uint v377 = v375 + (v375 << 3u);
        uint v379 = v377 ^ (v377 >> 11u);
        uint v385 = (((v369.x * 1664525u) + (v379 + (v379 << 15u))) + 1013904223u) * 1664525u;
        uint v387 = v385 ^ (v385 >> 11u);
        uint v390 = v387 ^ ((v387 << 7u) & 2636928640u);
        uint v393 = v390 ^ ((v390 << 15u) & 4022730752u);
        uint2 v401 = uint2(v303 + int2(1, 1));
        uint v403 = v401.y;
        uint v405 = v403 + (v403 << 10u);
        uint v407 = v405 ^ (v405 >> 6u);
        uint v409 = v407 + (v407 << 3u);
        uint v411 = v409 ^ (v409 >> 11u);
        uint v417 = (((v401.x * 1664525u) + (v411 + (v411 << 15u))) + 1013904223u) * 1664525u;
        uint v419 = v417 ^ (v417 >> 11u);
        uint v422 = v419 ^ ((v419 << 7u) & 2636928640u);
        uint v425 = v422 ^ ((v422 << 15u) & 4022730752u);
        float2 v435 = (v304 * v304) * (3.0f.xx - (v304 * 2.0f));
        float v436 = v435.x;
        float v437 = 1.0f - v436;
        float v439 = v435.y;
        if ((int((((((1.0f - v439) * (((asfloat(((v329 ^ (v329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v437) + ((asfloat(((v361 ^ (v361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v436))) + (((asfloat(((v393 ^ (v393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v437) * v439)) + (((asfloat(((v425 ^ (v425 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v436) * v439)) * 2.0f) > 0.60000002384185791015625f) & int(v280 > 0.425000011920928955078125f)) != 0)
        {
            float3 v459 = v174.xxy;
            v459.y = 0.5f;
            int v461 = v195.x;
            int v462 = v195.y;
            uint v464 = uint(v461 + v462);
            uint v466 = v464 + (v464 << 10u);
            uint v468 = v466 ^ (v466 >> 6u);
            uint v470 = v468 + (v468 << 3u);
            uint v472 = v470 ^ (v470 >> 11u);
            float v479 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((v472 + (v472 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v482 = v479.xxx * lerp(0.5f, 1.0f, v294 + (v298 * 1.5f));
            float3 v491 = float3(min(v482.x, 0.75f), (v482.y * 1.2000000476837158203125f) * v479, min(v482.z, 0.75f)) * 1.5f;
            bool v492 = v298 > 0.0f;
            float3 v496;
            if (v492)
            {
                float3 v495 = v459;
                v495.y = 1.10000002384185791015625f;
                v496 = v495;
            }
            else
            {
                v496 = v459;
            }
            uint v515 = uint((v461 * v462) + v461);
            uint v517 = v515 + (v515 << 10u);
            uint v519 = v517 ^ (v517 >> 6u);
            uint v521 = v519 + (v519 << 3u);
            uint v523 = v521 ^ (v521 >> 11u);
            float v530 = 3.1400001049041748046875f * (asfloat(((v523 + (v523 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v531 = normalize(float3(0.0f, 1.0f, 0.0f));
            float v532 = sin(v530);
            float v533 = cos(v530);
            float v534 = 1.0f - v533;
            float v535 = v531.x;
            float v536 = v534 * v535;
            float v539 = v531.y;
            float v540 = v536 * v539;
            float v541 = v531.z;
            float v542 = v541 * v532;
            float v544 = v534 * v541;
            float v545 = v544 * v535;
            float v546 = v539 * v532;
            float v549 = v534 * v539;
            float v552 = v549 * v541;
            float v553 = v535 * v532;
            float4x4 v564 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, v496.x), float4(0.0f, 1.0f, 0.0f, v496.y), float4(0.0f, 0.0f, 1.0f, v496.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(v491.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v491.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v491.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((v536 * v535) + v533, v540 - v542, v545 + v546, 0.0f), float4(v540 + v542, (v549 * v539) + v533, v552 - v553, 0.0f), float4(v545 - v546, v552 + v553, (v544 * v541) + v533, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            bool3 v592 = v492.xxx;
            uint v595;
            __alias_draw_commands.InterlockedAdd(224, 1u, v595);
            uint v597 = uint(int(v595));
            float v599[12] = { v564[0].x, v564[1].x, v564[2].x, v564[0].y, v564[1].y, v564[2].y, v564[0].z, v564[1].z, v564[2].z, v564[0].w, v564[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(v564[0].x, v564[1].x, v564[2].x), float3(v564[0].y, v564[1].y, v564[2].y), float3(v564[0].z, v564[1].z, v564[2].z), float3(v564[0].w, v564[1].w, v564[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), v564[2].w };
            Affine3Packed v600 = { v599 };
            FlowerXform v601 = { v600, float3(v592.x ? 1.0f.xxx.x : (-1.0f).xxx.x, v592.y ? 1.0f.xxx.y : (-1.0f).xxx.y, v592.z ? 1.0f.xxx.z : (-1.0f).xxx.z), 0u };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_flower_buffer.Store<float>(_4ident * 4 + v597 * 64 + 0, v601.transform.data[_4ident]);
            }
            __alias_flower_buffer.Store<float3>(v597 * 64 + 48, v601.color);
            __alias_flower_buffer.Store<uint>(v597 * 64 + 60, v601.flower_type);
        }
        v602 = 1013904223u;
        v603 = 1664525u;
        v604 = true;
        break;
    } while(false);
    do
    {
        float2 v610;
        bool v662;
        do
        {
            v610 = (v194 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 v612 = uint2(v610 * v204);
            float4 v623 = terrain_heightmap.GatherRed(sampler_default, v610, int2(0, 0));
            float v624 = v623.z;
            float v625 = v623.y;
            float v627 = v623.w;
            float v628 = v623.x;
            bool v649;
            if (!(path_mask.Load(int3(v612, 0u)).x > 0.20000000298023223876953125f))
            {
                v649 = (int(rocky_terrain.Load(int3(v612, 0u)).x > 0.0f) & int(length(((float2((v624 + v625) - (v627 + v628), (v628 + v625) - (v627 + v624)) * 0.5f.xx) * 26.5f) / v228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v649 = true;
            }
            bool v654;
            if (!v649)
            {
                v654 = water_mask.Load(int3(v612, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v654 = true;
            }
            bool v659;
            if (!v654)
            {
                v659 = foliage_exclusion_mask.SampleLevel(sampler_llb, v610, 0.0f).x > 0.0f;
            }
            else
            {
                v659 = true;
            }
            if (v659)
            {
                v662 = true;
                break;
            }
            v662 = false;
            break;
        } while(false);
        if (v662)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, v610, 0.0f).x > 0.5f)
        {
            break;
        }
        float4 v674 = garden_mask.SampleLevel(sampler_default, v610, 0.0f);
        float v675 = v674.y;
        float v689;
        do
        {
            float v679 = abs(v675 - 0.650000035762786865234375f);
            if (v679 > 0.300000011920928955078125f)
            {
                v689 = 0.0f;
                break;
            }
            float v683 = v679 * 3.3333332538604736328125f;
            v689 = 1.0f - ((v683 * v683) * (3.0f - (v679 * 6.666666507720947265625f)));
            break;
        } while(false);
        float v693 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, v610, 0.0f).y);
        float2 v697 = (v194 * 0.5f) + 100.0f.xx;
        int2 v699 = int2(floor(v697));
        float2 v700 = frac(v697);
        uint2 v701 = uint2(v699);
        uint v703 = v701.y;
        uint v705 = v703 + (v703 << 10u);
        uint v707 = v705 ^ (v705 >> 6u);
        uint v709 = v707 + (v707 << 3u);
        uint v711 = v709 ^ (v709 >> 11u);
        bool v714 = v604 ? v604 : true;
        uint v715 = v604 ? v603 : 1664525u;
        uint v716 = v604 ? v602 : 1013904223u;
        uint v720 = (((v701.x * v715) + (v711 + (v711 << 15u))) + v716) * v715;
        uint v722 = v720 ^ (v720 >> 11u);
        uint v725 = v722 ^ ((v722 << 7u) & 2636928640u);
        uint v728 = v725 ^ ((v725 << 15u) & 4022730752u);
        uint2 v736 = uint2(v699 + int2(1, 0));
        uint v738 = v736.y;
        uint v740 = v738 + (v738 << 10u);
        uint v742 = v740 ^ (v740 >> 6u);
        uint v744 = v742 + (v742 << 3u);
        uint v746 = v744 ^ (v744 >> 11u);
        bool v749 = v714 ? v714 : true;
        uint v750 = v714 ? v715 : 1664525u;
        uint v751 = v714 ? v716 : 1013904223u;
        uint v755 = (((v736.x * v750) + (v746 + (v746 << 15u))) + v751) * v750;
        uint v757 = v755 ^ (v755 >> 11u);
        uint v760 = v757 ^ ((v757 << 7u) & 2636928640u);
        uint v763 = v760 ^ ((v760 << 15u) & 4022730752u);
        uint2 v771 = uint2(v699 + int2(0, 1));
        uint v773 = v771.y;
        uint v775 = v773 + (v773 << 10u);
        uint v777 = v775 ^ (v775 >> 6u);
        uint v779 = v777 + (v777 << 3u);
        uint v781 = v779 ^ (v779 >> 11u);
        bool v784 = v749 ? v749 : true;
        uint v785 = v749 ? v750 : 1664525u;
        uint v786 = v749 ? v751 : 1013904223u;
        uint v790 = (((v771.x * v785) + (v781 + (v781 << 15u))) + v786) * v785;
        uint v792 = v790 ^ (v790 >> 11u);
        uint v795 = v792 ^ ((v792 << 7u) & 2636928640u);
        uint v798 = v795 ^ ((v795 << 15u) & 4022730752u);
        uint2 v806 = uint2(v699 + int2(1, 1));
        uint v808 = v806.y;
        uint v810 = v808 + (v808 << 10u);
        uint v812 = v810 ^ (v810 >> 6u);
        uint v814 = v812 + (v812 << 3u);
        uint v816 = v814 ^ (v814 >> 11u);
        uint v819 = v784 ? v785 : 1664525u;
        uint v824 = (((v806.x * v819) + (v816 + (v816 << 15u))) + (v784 ? v786 : 1013904223u)) * v819;
        uint v826 = v824 ^ (v824 >> 11u);
        uint v829 = v826 ^ ((v826 << 7u) & 2636928640u);
        uint v832 = v829 ^ ((v829 << 15u) & 4022730752u);
        float2 v842 = (v700 * v700) * (3.0f.xx - (v700 * 2.0f));
        float v843 = v842.x;
        float v844 = 1.0f - v843;
        float v846 = v842.y;
        if ((int((((((1.0f - v846) * (((asfloat(((v728 ^ (v728 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v844) + ((asfloat(((v763 ^ (v763 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v843))) + (((asfloat(((v798 ^ (v798 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v844) * v846)) + (((asfloat(((v832 ^ (v832 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v843) * v846)) * 2.0f) > 0.60000002384185791015625f) & int(v675 > 0.425000011920928955078125f)) != 0)
        {
            float3 v866 = v194.xxy;
            v866.y = 0.5f;
            int v868 = v195.x;
            int v869 = v195.y;
            uint v871 = uint(v868 + v869);
            uint v873 = v871 + (v871 << 10u);
            uint v875 = v873 ^ (v873 >> 6u);
            uint v877 = v875 + (v875 << 3u);
            uint v879 = v877 ^ (v877 >> 11u);
            float v886 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((v879 + (v879 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v889 = v886.xxx * lerp(0.5f, 1.0f, v689 + (v693 * 1.5f));
            float3 v898 = float3(min(v889.x, 0.75f), (v889.y * 1.2000000476837158203125f) * v886, min(v889.z, 0.75f)) * 1.5f;
            float3 v903;
            if (v693 > 0.0f)
            {
                float3 v902 = v866;
                v902.y = 1.10000002384185791015625f;
                v903 = v902;
            }
            else
            {
                v903 = v866;
            }
            uint v922 = uint((v868 * v869) + v868);
            uint v924 = v922 + (v922 << 10u);
            uint v926 = v924 ^ (v924 >> 6u);
            uint v928 = v926 + (v926 << 3u);
            uint v930 = v928 ^ (v928 >> 11u);
            float v937 = 3.1400001049041748046875f * (asfloat(((v930 + (v930 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v938 = normalize(float3(0.0f, 1.0f, 0.0f));
            float v939 = sin(v937);
            float v940 = cos(v937);
            float v941 = 1.0f - v940;
            float v942 = v938.x;
            float v943 = v941 * v942;
            float v946 = v938.y;
            float v947 = v943 * v946;
            float v948 = v938.z;
            float v949 = v948 * v939;
            float v951 = v941 * v948;
            float v952 = v951 * v942;
            float v953 = v946 * v939;
            float v956 = v941 * v946;
            float v959 = v956 * v948;
            float v960 = v942 * v939;
            float4x4 v971 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, v903.x), float4(0.0f, 1.0f, 0.0f, v903.y), float4(0.0f, 0.0f, 1.0f, v903.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(v898.x, 0.0f, 0.0f, 0.0f), float4(0.0f, v898.y, 0.0f, 0.0f), float4(0.0f, 0.0f, v898.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((v943 * v942) + v940, v947 - v949, v952 + v953, 0.0f), float4(v947 + v949, (v956 * v946) + v940, v959 - v960, 0.0f), float4(v952 - v953, v959 + v960, (v951 * v948) + v940, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint v1000;
            __alias_draw_commands.InterlockedAdd(224, 1u, v1000);
            uint v1002 = uint(int(v1000));
            float v1004[12] = { v971[0].x, v971[1].x, v971[2].x, v971[0].y, v971[1].y, v971[2].y, v971[0].z, v971[1].z, v971[2].z, v971[0].w, v971[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(v971[0].x, v971[1].x, v971[2].x), float3(v971[0].y, v971[1].y, v971[2].y), float3(v971[0].z, v971[1].z, v971[2].z), float3(v971[0].w, v971[1].w, v971[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), v971[2].w };
            Affine3Packed v1005 = { v1004 };
            FlowerXform v1006 = { v1005, (-1.0f).xxx, 1u };
            [unroll]
            for (int _5ident = 0; _5ident < 12; _5ident++)
            {
                __alias_flower_buffer.Store<float>(_5ident * 4 + v1002 * 64 + 0, v1006.transform.data[_5ident]);
            }
            __alias_flower_buffer.Store<float3>(v1002 * 64 + 48, v1006.color);
            __alias_flower_buffer.Store<uint>(v1002 * 64 + 60, v1006.flower_type);
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
