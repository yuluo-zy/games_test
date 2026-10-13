// PATCHED: modules en.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _garden_spawn_flowers_garden.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _119;

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
    float2 _204;
    float2 _228;
    float2 _152 = (((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx;
    uint _157 = ((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x) + 100u;
    uint _159 = _157 + (_157 << 10u);
    uint _161 = _159 ^ (_159 >> 6u);
    uint _163 = _161 + (_161 << 3u);
    uint _165 = _163 ^ (_163 >> 11u);
    float _171 = asfloat(((_165 + (_165 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _174 = _152 + lerp(-0.75f, 0.75f, _171).xx;
    uint _177 = uint((_171 * 100.0f) - 34.0f);
    uint _179 = _177 + (_177 << 10u);
    uint _181 = _179 ^ (_179 >> 6u);
    uint _183 = _181 + (_181 << 3u);
    uint _185 = _183 ^ (_183 >> 11u);
    float2 _194 = _152 + lerp(-0.75f, 0.75f, asfloat(((_185 + (_185 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    int2 _195 = int2(gl_GlobalInvocationID.xy);
    uint _602;
    uint _603;
    bool _604;
    do
    {
        float2 _201;
        bool _267;
        do
        {
            _201 = (_174 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            _204 = float2(path_mask_dims);
            uint2 _206 = uint2(_201 * _204);
            float4 _224 = terrain_heightmap.GatherRed(sampler_default, _201, int2(0, 0));
            _228 = 130.0f.xx / float2(terrain_heightmap_dims);
            float _229 = _224.z;
            float _230 = _224.y;
            float _232 = _224.w;
            float _233 = _224.x;
            bool _254;
            if (!(path_mask.Load(int3(_206, 0u)).x > 0.20000000298023223876953125f))
            {
                _254 = (int(rocky_terrain.Load(int3(_206, 0u)).x > 0.0f) & int(length(((float2((_229 + _230) - (_232 + _233), (_233 + _230) - (_232 + _229)) * 0.5f.xx) * 26.5f) / _228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _254 = true;
            }
            bool _259;
            if (!_254)
            {
                _259 = water_mask.Load(int3(_206, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _259 = true;
            }
            bool _264;
            if (!_259)
            {
                _264 = foliage_exclusion_mask.SampleLevel(sampler_llb, _201, 0.0f).x > 0.0f;
            }
            else
            {
                _264 = true;
            }
            if (_264)
            {
                _267 = true;
                break;
            }
            _267 = false;
            break;
        } while(false);
        if (_267)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _201, 0.0f).x > 0.5f)
        {
            _602 = _119;
            _603 = _119;
            _604 = false;
            break;
        }
        float4 _279 = garden_mask.SampleLevel(sampler_default, _201, 0.0f);
        float _280 = _279.y;
        float _294;
        do
        {
            float _284 = abs(_280 - 0.60000002384185791015625f);
            if (_284 > 0.20000000298023223876953125f)
            {
                _294 = 0.0f;
                break;
            }
            float _288 = _284 * 5.0f;
            _294 = 1.0f - ((_288 * _288) * (3.0f - (_284 * 10.0f)));
            break;
        } while(false);
        float _298 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, _201, 0.0f).y);
        float2 _301 = _174 * 0.5f;
        int2 _303 = int2(floor(_301));
        float2 _304 = frac(_301);
        uint2 _305 = uint2(_303);
        uint _307 = _305.y;
        uint _309 = _307 + (_307 << 10u);
        uint _311 = _309 ^ (_309 >> 6u);
        uint _313 = _311 + (_311 << 3u);
        uint _315 = _313 ^ (_313 >> 11u);
        uint _321 = (((_305.x * 1664525u) + (_315 + (_315 << 15u))) + 1013904223u) * 1664525u;
        uint _323 = _321 ^ (_321 >> 11u);
        uint _326 = _323 ^ ((_323 << 7u) & 2636928640u);
        uint _329 = _326 ^ ((_326 << 15u) & 4022730752u);
        uint2 _337 = uint2(_303 + int2(1, 0));
        uint _339 = _337.y;
        uint _341 = _339 + (_339 << 10u);
        uint _343 = _341 ^ (_341 >> 6u);
        uint _345 = _343 + (_343 << 3u);
        uint _347 = _345 ^ (_345 >> 11u);
        uint _353 = (((_337.x * 1664525u) + (_347 + (_347 << 15u))) + 1013904223u) * 1664525u;
        uint _355 = _353 ^ (_353 >> 11u);
        uint _358 = _355 ^ ((_355 << 7u) & 2636928640u);
        uint _361 = _358 ^ ((_358 << 15u) & 4022730752u);
        uint2 _369 = uint2(_303 + int2(0, 1));
        uint _371 = _369.y;
        uint _373 = _371 + (_371 << 10u);
        uint _375 = _373 ^ (_373 >> 6u);
        uint _377 = _375 + (_375 << 3u);
        uint _379 = _377 ^ (_377 >> 11u);
        uint _385 = (((_369.x * 1664525u) + (_379 + (_379 << 15u))) + 1013904223u) * 1664525u;
        uint _387 = _385 ^ (_385 >> 11u);
        uint _390 = _387 ^ ((_387 << 7u) & 2636928640u);
        uint _393 = _390 ^ ((_390 << 15u) & 4022730752u);
        uint2 _401 = uint2(_303 + int2(1, 1));
        uint _403 = _401.y;
        uint _405 = _403 + (_403 << 10u);
        uint _407 = _405 ^ (_405 >> 6u);
        uint _409 = _407 + (_407 << 3u);
        uint _411 = _409 ^ (_409 >> 11u);
        uint _417 = (((_401.x * 1664525u) + (_411 + (_411 << 15u))) + 1013904223u) * 1664525u;
        uint _419 = _417 ^ (_417 >> 11u);
        uint _422 = _419 ^ ((_419 << 7u) & 2636928640u);
        uint _425 = _422 ^ ((_422 << 15u) & 4022730752u);
        float2 _435 = (_304 * _304) * (3.0f.xx - (_304 * 2.0f));
        float _436 = _435.x;
        float _437 = 1.0f - _436;
        float _439 = _435.y;
        if ((int((((((1.0f - _439) * (((asfloat(((_329 ^ (_329 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _437) + ((asfloat(((_361 ^ (_361 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _436))) + (((asfloat(((_393 ^ (_393 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _437) * _439)) + (((asfloat(((_425 ^ (_425 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _436) * _439)) * 2.0f) > 0.60000002384185791015625f) & int(_280 > 0.425000011920928955078125f)) != 0)
        {
            float3 _459 = _174.xxy;
            _459.y = 0.5f;
            int _461 = _195.x;
            int _462 = _195.y;
            uint _464 = uint(_461 + _462);
            uint _466 = _464 + (_464 << 10u);
            uint _468 = _466 ^ (_466 >> 6u);
            uint _470 = _468 + (_468 << 3u);
            uint _472 = _470 ^ (_470 >> 11u);
            float _479 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _482 = _479.xxx * lerp(0.5f, 1.0f, _294 + (_298 * 1.5f));
            float3 _491 = float3(min(_482.x, 0.75f), (_482.y * 1.2000000476837158203125f) * _479, min(_482.z, 0.75f)) * 1.5f;
            bool _492 = _298 > 0.0f;
            float3 _496;
            if (_492)
            {
                float3 _495 = _459;
                _495.y = 1.10000002384185791015625f;
                _496 = _495;
            }
            else
            {
                _496 = _459;
            }
            uint _515 = uint((_461 * _462) + _461);
            uint _517 = _515 + (_515 << 10u);
            uint _519 = _517 ^ (_517 >> 6u);
            uint _521 = _519 + (_519 << 3u);
            uint _523 = _521 ^ (_521 >> 11u);
            float _530 = 3.1400001049041748046875f * (asfloat(((_523 + (_523 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _531 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _532 = sin(_530);
            float _533 = cos(_530);
            float _534 = 1.0f - _533;
            float _535 = _531.x;
            float _536 = _534 * _535;
            float _539 = _531.y;
            float _540 = _536 * _539;
            float _541 = _531.z;
            float _542 = _541 * _532;
            float _544 = _534 * _541;
            float _545 = _544 * _535;
            float _546 = _539 * _532;
            float _549 = _534 * _539;
            float _552 = _549 * _541;
            float _553 = _535 * _532;
            float4x4 _564 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _496.x), float4(0.0f, 1.0f, 0.0f, _496.y), float4(0.0f, 0.0f, 1.0f, _496.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_491.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _491.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _491.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_536 * _535) + _533, _540 - _542, _545 + _546, 0.0f), float4(_540 + _542, (_549 * _539) + _533, _552 - _553, 0.0f), float4(_545 - _546, _552 + _553, (_544 * _541) + _533, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            bool3 _592 = _492.xxx;
            uint _595;
            __alias_draw_commands.InterlockedAdd(224, 1u, _595);
            uint _597 = uint(int(_595));
            float _599[12] = { _564[0].x, _564[1].x, _564[2].x, _564[0].y, _564[1].y, _564[2].y, _564[0].z, _564[1].z, _564[2].z, _564[0].w, _564[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_564[0].x, _564[1].x, _564[2].x), float3(_564[0].y, _564[1].y, _564[2].y), float3(_564[0].z, _564[1].z, _564[2].z), float3(_564[0].w, _564[1].w, _564[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _564[2].w };
            Affine3Packed _600 = { _599 };
            FlowerXform _601 = { _600, float3(_592.x ? 1.0f.xxx.x : (-1.0f).xxx.x, _592.y ? 1.0f.xxx.y : (-1.0f).xxx.y, _592.z ? 1.0f.xxx.z : (-1.0f).xxx.z), 0u };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_flower_buffer.Store<float>(_4ident * 4 + _597 * 64 + 0, _601.transform.data[_4ident]);
            }
            __alias_flower_buffer.Store<float3>(_597 * 64 + 48, _601.color);
            __alias_flower_buffer.Store<uint>(_597 * 64 + 60, _601.flower_type);
        }
        _602 = 1013904223u;
        _603 = 1664525u;
        _604 = true;
        break;
    } while(false);
    do
    {
        float2 _610;
        bool _662;
        do
        {
            _610 = (_194 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            uint2 _612 = uint2(_610 * _204);
            float4 _623 = terrain_heightmap.GatherRed(sampler_default, _610, int2(0, 0));
            float _624 = _623.z;
            float _625 = _623.y;
            float _627 = _623.w;
            float _628 = _623.x;
            bool _649;
            if (!(path_mask.Load(int3(_612, 0u)).x > 0.20000000298023223876953125f))
            {
                _649 = (int(rocky_terrain.Load(int3(_612, 0u)).x > 0.0f) & int(length(((float2((_624 + _625) - (_627 + _628), (_628 + _625) - (_627 + _624)) * 0.5f.xx) * 26.5f) / _228) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _649 = true;
            }
            bool _654;
            if (!_649)
            {
                _654 = water_mask.Load(int3(_612, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _654 = true;
            }
            bool _659;
            if (!_654)
            {
                _659 = foliage_exclusion_mask.SampleLevel(sampler_llb, _610, 0.0f).x > 0.0f;
            }
            else
            {
                _659 = true;
            }
            if (_659)
            {
                _662 = true;
                break;
            }
            _662 = false;
            break;
        } while(false);
        if (_662)
        {
            break;
        }
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _610, 0.0f).x > 0.5f)
        {
            break;
        }
        float4 _674 = garden_mask.SampleLevel(sampler_default, _610, 0.0f);
        float _675 = _674.y;
        float _689;
        do
        {
            float _679 = abs(_675 - 0.650000035762786865234375f);
            if (_679 > 0.300000011920928955078125f)
            {
                _689 = 0.0f;
                break;
            }
            float _683 = _679 * 3.3333332538604736328125f;
            _689 = 1.0f - ((_683 * _683) * (3.0f - (_679 * 6.666666507720947265625f)));
            break;
        } while(false);
        float _693 = smoothstep(0.89999997615814208984375f, 1.0f, garden_mask.SampleLevel(sampler_default, _610, 0.0f).y);
        float2 _697 = (_194 * 0.5f) + 100.0f.xx;
        int2 _699 = int2(floor(_697));
        float2 _700 = frac(_697);
        uint2 _701 = uint2(_699);
        uint _703 = _701.y;
        uint _705 = _703 + (_703 << 10u);
        uint _707 = _705 ^ (_705 >> 6u);
        uint _709 = _707 + (_707 << 3u);
        uint _711 = _709 ^ (_709 >> 11u);
        bool _714 = _604 ? _604 : true;
        uint _715 = _604 ? _603 : 1664525u;
        uint _716 = _604 ? _602 : 1013904223u;
        uint _720 = (((_701.x * _715) + (_711 + (_711 << 15u))) + _716) * _715;
        uint _722 = _720 ^ (_720 >> 11u);
        uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
        uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
        uint2 _736 = uint2(_699 + int2(1, 0));
        uint _738 = _736.y;
        uint _740 = _738 + (_738 << 10u);
        uint _742 = _740 ^ (_740 >> 6u);
        uint _744 = _742 + (_742 << 3u);
        uint _746 = _744 ^ (_744 >> 11u);
        bool _749 = _714 ? _714 : true;
        uint _750 = _714 ? _715 : 1664525u;
        uint _751 = _714 ? _716 : 1013904223u;
        uint _755 = (((_736.x * _750) + (_746 + (_746 << 15u))) + _751) * _750;
        uint _757 = _755 ^ (_755 >> 11u);
        uint _760 = _757 ^ ((_757 << 7u) & 2636928640u);
        uint _763 = _760 ^ ((_760 << 15u) & 4022730752u);
        uint2 _771 = uint2(_699 + int2(0, 1));
        uint _773 = _771.y;
        uint _775 = _773 + (_773 << 10u);
        uint _777 = _775 ^ (_775 >> 6u);
        uint _779 = _777 + (_777 << 3u);
        uint _781 = _779 ^ (_779 >> 11u);
        bool _784 = _749 ? _749 : true;
        uint _785 = _749 ? _750 : 1664525u;
        uint _786 = _749 ? _751 : 1013904223u;
        uint _790 = (((_771.x * _785) + (_781 + (_781 << 15u))) + _786) * _785;
        uint _792 = _790 ^ (_790 >> 11u);
        uint _795 = _792 ^ ((_792 << 7u) & 2636928640u);
        uint _798 = _795 ^ ((_795 << 15u) & 4022730752u);
        uint2 _806 = uint2(_699 + int2(1, 1));
        uint _808 = _806.y;
        uint _810 = _808 + (_808 << 10u);
        uint _812 = _810 ^ (_810 >> 6u);
        uint _814 = _812 + (_812 << 3u);
        uint _816 = _814 ^ (_814 >> 11u);
        uint _819 = _784 ? _785 : 1664525u;
        uint _824 = (((_806.x * _819) + (_816 + (_816 << 15u))) + (_784 ? _786 : 1013904223u)) * _819;
        uint _826 = _824 ^ (_824 >> 11u);
        uint _829 = _826 ^ ((_826 << 7u) & 2636928640u);
        uint _832 = _829 ^ ((_829 << 15u) & 4022730752u);
        float2 _842 = (_700 * _700) * (3.0f.xx - (_700 * 2.0f));
        float _843 = _842.x;
        float _844 = 1.0f - _843;
        float _846 = _842.y;
        if ((int((((((1.0f - _846) * (((asfloat(((_728 ^ (_728 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _844) + ((asfloat(((_763 ^ (_763 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _843))) + (((asfloat(((_798 ^ (_798 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _844) * _846)) + (((asfloat(((_832 ^ (_832 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _843) * _846)) * 2.0f) > 0.60000002384185791015625f) & int(_675 > 0.425000011920928955078125f)) != 0)
        {
            float3 _866 = _194.xxy;
            _866.y = 0.5f;
            int _868 = _195.x;
            int _869 = _195.y;
            uint _871 = uint(_868 + _869);
            uint _873 = _871 + (_871 << 10u);
            uint _875 = _873 ^ (_873 >> 6u);
            uint _877 = _875 + (_875 << 3u);
            uint _879 = _877 ^ (_877 >> 11u);
            float _886 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_879 + (_879 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _889 = _886.xxx * lerp(0.5f, 1.0f, _689 + (_693 * 1.5f));
            float3 _898 = float3(min(_889.x, 0.75f), (_889.y * 1.2000000476837158203125f) * _886, min(_889.z, 0.75f)) * 1.5f;
            float3 _903;
            if (_693 > 0.0f)
            {
                float3 _902 = _866;
                _902.y = 1.10000002384185791015625f;
                _903 = _902;
            }
            else
            {
                _903 = _866;
            }
            uint _922 = uint((_868 * _869) + _868);
            uint _924 = _922 + (_922 << 10u);
            uint _926 = _924 ^ (_924 >> 6u);
            uint _928 = _926 + (_926 << 3u);
            uint _930 = _928 ^ (_928 >> 11u);
            float _937 = 3.1400001049041748046875f * (asfloat(((_930 + (_930 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _938 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _939 = sin(_937);
            float _940 = cos(_937);
            float _941 = 1.0f - _940;
            float _942 = _938.x;
            float _943 = _941 * _942;
            float _946 = _938.y;
            float _947 = _943 * _946;
            float _948 = _938.z;
            float _949 = _948 * _939;
            float _951 = _941 * _948;
            float _952 = _951 * _942;
            float _953 = _946 * _939;
            float _956 = _941 * _946;
            float _959 = _956 * _948;
            float _960 = _942 * _939;
            float4x4 _971 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _903.x), float4(0.0f, 1.0f, 0.0f, _903.y), float4(0.0f, 0.0f, 1.0f, _903.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_898.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _898.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _898.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_943 * _942) + _940, _947 - _949, _952 + _953, 0.0f), float4(_947 + _949, (_956 * _946) + _940, _959 - _960, 0.0f), float4(_952 - _953, _959 + _960, (_951 * _948) + _940, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            uint _1000;
            __alias_draw_commands.InterlockedAdd(224, 1u, _1000);
            uint _1002 = uint(int(_1000));
            float _1004[12] = { _971[0].x, _971[1].x, _971[2].x, _971[0].y, _971[1].y, _971[2].y, _971[0].z, _971[1].z, _971[2].z, _971[0].w, _971[1].w + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_971[0].x, _971[1].x, _971[2].x), float3(_971[0].y, _971[1].y, _971[2].y), float3(_971[0].z, _971[1].z, _971[2].z), float3(_971[0].w, _971[1].w, _971[2].w))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _971[2].w };
            Affine3Packed _1005 = { _1004 };
            FlowerXform _1006 = { _1005, (-1.0f).xxx, 1u };
            [unroll]
            for (int _5ident = 0; _5ident < 12; _5ident++)
            {
                __alias_flower_buffer.Store<float>(_5ident * 4 + _1002 * 64 + 0, _1006.transform.data[_5ident]);
            }
            __alias_flower_buffer.Store<float3>(_1002 * 64 + 48, _1006.color);
            __alias_flower_buffer.Store<uint>(_1002 * 64 + 60, _1006.flower_type);
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
