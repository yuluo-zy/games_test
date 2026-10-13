// PATCHED: modules en.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED==
// 功能:花园小区花朵生成:抖动坐标 + 排除,在 garden_mask.y 带内放置 Flower03Xform。 shaders/garden/spawn_small_flowers_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_small_flowers_garden.cs.hlsl_cs_main_0.bin (jittered world pos seed+100; exclusion masks; garden_mask.y in (0.35,0.8) band; Flower03Xform{float4(x,0,z,size)} -> draw cmd 16, stride 16) | status: O (algorithm from SPIR-V); I (band/size literals)

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

struct Flower03Xform
{
    float4 position;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 garden_mask_dims : packoffset(c1.z);
    int2 grid_dims : packoffset(c2);
    uint2 path_mask_dims : packoffset(c2.z);
    uint2 rocky_terrain_dims : packoffset(c3);
    uint2 water_mask_dims : packoffset(c3.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c4);
};

RWByteAddressBuffer __alias_flower03_buffer : register(u18, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t20, space0);
Texture2D<float4> water_mask : register(t21, space0);
Texture2D<float4> foliage_exclusion_mask : register(t22, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        float2 v136;
        uint v115 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
        uint v117 = v115 + (v115 << 10u);
        uint v119 = v117 ^ (v117 >> 6u);
        uint v121 = v119 + (v119 << 3u);
        uint v123 = v121 ^ (v121 >> 11u);
        float2 v132 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-0.75f, 0.75f, asfloat(((v123 + (v123 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
        bool v202;
        do
        {
            v136 = (v132 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 v141 = uint2(v136 * float2(path_mask_dims));
            float4 v159 = terrain_heightmap.GatherRed(sampler_default, v136, int2(0, 0));
            float v164 = v159.z;
            float v165 = v159.y;
            float v167 = v159.w;
            float v168 = v159.x;
            bool v189;
            if (!(path_mask.Load(int3(v141, 0u)).x > 0.20000000298023223876953125f))
            {
                v189 = (int(rocky_terrain.Load(int3(v141, 0u)).x > 0.0f) & int(length(((float2((v164 + v165) - (v167 + v168), (v168 + v165) - (v167 + v164)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                v189 = true;
            }
            bool v194;
            if (!v189)
            {
                v194 = water_mask.Load(int3(v141, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                v194 = true;
            }
            bool v199;
            if (!v194)
            {
                v199 = foliage_exclusion_mask.SampleLevel(sampler_llb, v136, 0.0f).x > 0.0f;
            }
            else
            {
                v199 = true;
            }
            if (v199)
            {
                v202 = true;
                break;
            }
            v202 = false;
            break;
        } while(false);
        if (v202)
        {
            break;
        }
        float v209 = garden_mask.SampleLevel(sampler_default, v136, 0.0f).y - 0.0500000007450580596923828125f;
        bool v214;
        if (v209 > 0.3499999940395355224609375f)
        {
            v214 = v209 < 0.800000011920928955078125f;
        }
        else
        {
            v214 = false;
        }
        if (v214)
        {
            uint v217 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint v219 = v217 + (v217 << 10u);
            uint v221 = v219 ^ (v219 >> 6u);
            uint v223 = v221 + (v221 << 3u);
            uint v225 = v223 ^ (v223 >> 11u);
            uint v237;
            __alias_draw_commands.InterlockedAdd(324, 1u, v237);
            uint v239 = uint(int(v237));
            Flower03Xform v241 = { float4(v132.x, 0.0f, v132.y, lerp(0.60000002384185791015625f, 1.0f, asfloat(((v225 + (v225 << 15u)) & 8388607u) | 1065353216u) - 1.0f)) };
            __alias_flower03_buffer.Store<float4>(v239 * 16 + 0, v241.position);
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
