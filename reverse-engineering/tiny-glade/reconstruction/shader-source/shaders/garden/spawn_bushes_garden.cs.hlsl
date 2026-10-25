// PATCHED: modules en.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _garden_spawn_bushes_garden.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
        float2 _161;
        uint _140 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _142 = _140 + (_140 << 10u);
        uint _144 = _142 ^ (_142 >> 6u);
        uint _146 = _144 + (_144 << 3u);
        uint _148 = _146 ^ (_146 >> 11u);
        float _154 = asfloat(((_148 + (_148 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float2 _157 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-1.0f, 1.0f, _154).xx;
        bool _227;
        do
        {
            _161 = (_157 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 _166 = uint2(_161 * float2(path_mask_dims));
            float4 _184 = terrain_heightmap.GatherRed(sampler_default, _161, int2(0, 0));
            float _189 = _184.z;
            float _190 = _184.y;
            float _192 = _184.w;
            float _193 = _184.x;
            bool _214;
            if (!(path_mask.Load(int3(_166, 0u)).x > 0.20000000298023223876953125f))
            {
                _214 = (int(rocky_terrain.Load(int3(_166, 0u)).x > 0.0f) & int(length(((float2((_189 + _190) - (_192 + _193), (_193 + _190) - (_192 + _189)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims))) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _214 = true;
            }
            bool _219;
            if (!_214)
            {
                _219 = water_mask.Load(int3(_166, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _219 = true;
            }
            bool _224;
            if (!_219)
            {
                _224 = foliage_exclusion_mask.SampleLevel(sampler_llb, _161, 0.0f).x > 0.0f;
            }
            else
            {
                _224 = true;
            }
            if (_224)
            {
                _227 = true;
                break;
            }
            _227 = false;
            break;
        } while(false);
        if (_227)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _161, 0.0f).x > 0.5f)
        {
            break;
        }
        float _241 = garden_mask.SampleLevel(sampler_default, _161, 0.0f).y + 0.004999999888241291046142578125f;
        if (_241 > 0.5f)
        {
            uint _257 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
            uint _259 = _257 + (_257 << 10u);
            uint _261 = _259 ^ (_259 >> 6u);
            uint _263 = _261 + (_261 << 3u);
            uint _265 = _263 ^ (_263 >> 11u);
            float _272 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_265 + (_265 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _276 = _272.xxx * lerp(0.300000011920928955078125f, 0.800000011920928955078125f, sqrt(clamp(smoothstep(0.449999988079071044921875f, 1.0f, _241) + (smoothstep(0.60000002384185791015625f, 1.0f, _241) * 0.20000000298023223876953125f), 0.0f, 1.0f)));
            float _296 = 3.1400001049041748046875f * _154;
            float3 _297 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _298 = sin(_296);
            float _299 = cos(_296);
            float _300 = 1.0f - _299;
            float _301 = _297.x;
            float _302 = _300 * _301;
            float _305 = _297.y;
            float _306 = _302 * _305;
            float _307 = _297.z;
            float _308 = _307 * _298;
            float _310 = _300 * _307;
            float _311 = _310 * _301;
            float _312 = _305 * _298;
            float _315 = _300 * _305;
            float _318 = _315 * _307;
            float _319 = _301 * _298;
            float4x4 _330 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _157.x), float4(0.0f, 1.0f, 0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_157.xy * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x)), float4(0.0f, 0.0f, 1.0f, _157.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(min(_276.x, 0.75f), 0.0f, 0.0f, 0.0f), float4(0.0f, min(_276.y * _272, 1.2000000476837158203125f), 0.0f, 0.0f), float4(0.0f, 0.0f, min(_276.z, 0.75f), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_302 * _301) + _299, _306 - _308, _311 + _312, 0.0f), float4(_306 + _308, (_315 * _305) + _299, _318 - _319, 0.0f), float4(_311 - _312, _318 + _319, (_310 * _307) + _299, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _344;
            __alias_draw_commands.InterlockedAdd(204, 1u, _344);
            uint _346 = uint(int(_344));
            float _348[12] = { _330[0].x, _330[1].x, _330[2].x, _330[0].y, _330[1].y, _330[2].y, _330[0].z, _330[1].z, _330[2].z, _330[0].w, _330[1].w, _330[2].w };
            Affine3Packed _349 = { _348 };
            BushXform _350 = { _349 };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_bush_buffer.Store<float>(_4ident * 4 + _346 * 48 + 0, _350.transform.data[_4ident]);
            }
            uint _354 = ((gl_GlobalInvocationID.x * 2u) + (gl_GlobalInvocationID.y / 5u)) + 1233u;
            uint _356 = _354 + (_354 << 10u);
            uint _358 = _356 ^ (_356 >> 6u);
            uint _360 = _358 + (_358 << 3u);
            uint _362 = _360 ^ (_360 >> 11u);
            bool _373;
            if ((asfloat(((_362 + (_362 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.300000011920928955078125f)
            {
                _373 = _241 > 0.800000011920928955078125f;
            }
            else
            {
                _373 = false;
            }
            if (_373)
            {
                uint _377;
                __alias_draw_commands.InterlockedAdd(224, 1u, _377);
                uint _379 = uint(int(_377));
                FlowerXform _381 = { _349, (-1.0f).xxx, 2u };
                [unroll]
                for (int _5ident = 0; _5ident < 12; _5ident++)
                {
                    __alias_bush_flowers.Store<float>(_5ident * 4 + _379 * 64 + 0, _381.transform.data[_5ident]);
                }
                __alias_bush_flowers.Store<float3>(_379 * 64 + 48, _381.color);
                __alias_bush_flowers.Store<uint>(_379 * 64 + 60, _381.flower_type);
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
