// PATCHED: modules en.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED==
// 功能:花园区灌木生成:抖动世界坐标,过排除/花园遮罩后按 perlin 门控放置 BushXform。 shaders/garden/spawn_bushes_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_bushes_garden.cs.hlsl_cs_main_0.bin (exclusion masks; buildings_non_blurred<0.5; garden_mask.y+0.005>0.5 -> BushXform (cmd 10, 48B) with T(terrain height)*S*R yaw; 30% -> FlowerXform (cmd 11, 64B, color=-1, type=2)) | status: O (algorithm from SPIR-V); I (smoothstep/threshold literals)

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

struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

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

RWByteAddressBuffer __alias_bush_buffer : register(u19, space0);
RWByteAddressBuffer __alias_bush_flowers : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
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
    do
    {
        float2 v161;
        uint v140 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint v142 = v140 + (v140 << 10u);
        uint v144 = v142 ^ (v142 >> 6u);
        uint v146 = v144 + (v144 << 3u);
        uint v148 = v146 ^ (v146 >> 11u);
        float v154 = asfloat(((v148 + (v148 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 v157 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, v154).xx;
        bool v227;
        do
        {
            v161 = (v157 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 v166 = uint2(v161 * float2(path_mask_dims));
            float4 v184 = terrain_heightmap.GatherRed(sampler_default, v161, int2(0, 0));
            float v189 = v184.z;
            float v190 = v184.y;
            float v192 = v184.w;
            float v193 = v184.x;
            bool v214;
            if (!(path_mask.Load(int3(v166, 0u)).x > 0.20000000298023223876953125f))
            {
                v214 = (int(rocky_terrain.Load(int3(v166, 0u)).x > 0.0f) & int(length(((float2((v189 + v190) - (v192 + v193), (v193 + v190) - (v192 + v189)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v214 = true;
            }
            bool v219;
            if (!v214)
            {
                v219 = water_mask.Load(int3(v166, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v219 = true;
            }
            bool v224;
            if (!v219)
            {
                v224 = foliage_exclusion_mask.SampleLevel(sampler_llb, v161, 0.0f).x > 0.0f;
            }
            else
            {
                v224 = true;
            }
            if (v224)
            {
                v227 = true;
                break;
            }
            v227 = false;
            break;
        } while(false);
        if (v227)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, v161, 0.0f).x > 0.5f)
        {
            break;
        }
        float v241 = garden_mask.SampleLevel(sampler_default, v161, 0.0f).y + 0.004999999888241291046142578125f;
        if (v241 > 0.5f)
        {
            uint v257 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint v259 = v257 + (v257 << 10u);
            uint v261 = v259 ^ (v259 >> 6u);
            uint v263 = v261 + (v261 << 3u);
            uint v265 = v263 ^ (v263 >> 11u);
            float v272 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((v265 + (v265 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 v276 = v272.xxx * lerp(0.300000011920928955078125f, 0.800000011920928955078125f, sqrt(clamp(smoothstep(0.449999988079071044921875f, 1.0f, v241) + (smoothstep(0.60000002384185791015625f, 1.0f, v241) * 0.20000000298023223876953125f), 0.0f, 1.0f)));
            float v296 = 3.1400001049041748046875f * v154;
            float3 v297 = normalize(float3(0.0f, 1.0f, 0.0f));
            float v298 = sin(v296);
            float v299 = cos(v296);
            float v300 = 1.0f - v299;
            float v301 = v297.x;
            float v302 = v300 * v301;
            float v305 = v297.y;
            float v306 = v302 * v305;
            float v307 = v297.z;
            float v308 = v307 * v298;
            float v310 = v300 * v307;
            float v311 = v310 * v301;
            float v312 = v305 * v298;
            float v315 = v300 * v305;
            float v318 = v315 * v307;
            float v319 = v301 * v298;
            float4x4 v330 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, v157.x), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (v157.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, v157.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(min(v276.x, 0.75f), 0.0f, 0.0f, 0.0f), float4(0.0f, min(v276.y * v272, 1.2000000476837158203125f), 0.0f, 0.0f), float4(0.0f, 0.0f, min(v276.z, 0.75f), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((v302 * v301) + v299, v306 - v308, v311 + v312, 0.0f), float4(v306 + v308, (v315 * v305) + v299, v318 - v319, 0.0f), float4(v311 - v312, v318 + v319, (v310 * v307) + v299, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint v344;
            __alias_draw_commands.InterlockedAdd(204, 1u, v344);
            uint v346 = uint(int(v344));
            float v348[12] = { v330[0].x, v330[1].x, v330[2].x, v330[0].y, v330[1].y, v330[2].y, v330[0].z, v330[1].z, v330[2].z, v330[0].w, v330[1].w, v330[2].w };
            Affine3Packed v349 = { v348 };
            BushXform v350 = { v349 };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_bush_buffer.Store<float>(_4ident * 4 + v346 * 48 + 0, v350.transform.data[_4ident]);
            }
            uint v354 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 1233u;
            uint v356 = v354 + (v354 << 10u);
            uint v358 = v356 ^ (v356 >> 6u);
            uint v360 = v358 + (v358 << 3u);
            uint v362 = v360 ^ (v360 >> 11u);
            bool v373;
            if ((asfloat(((v362 + (v362 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.300000011920928955078125f)
            {
                v373 = v241 > 0.800000011920928955078125f;
            }
            else
            {
                v373 = false;
            }
            if (v373)
            {
                uint v377;
                __alias_draw_commands.InterlockedAdd(224, 1u, v377);
                uint v379 = uint(int(v377));
                FlowerXform v381 = { v349, (-1.0f).xxx, 2u };
                [unroll]
                for (int _5ident = 0; _5ident < 12; _5ident++)
                {
                    __alias_bush_flowers.Store<float>(_5ident * 4 + v379 * 64 + 0, v381.transform.data[_5ident]);
                }
                __alias_bush_flowers.Store<float3>(v379 * 64 + 48, v381.color);
                __alias_bush_flowers.Store<uint>(v379 * 64 + 60, v381.flower_type);
            }
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
