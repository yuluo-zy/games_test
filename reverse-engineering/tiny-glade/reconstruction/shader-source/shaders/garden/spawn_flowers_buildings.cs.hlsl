// PATCHED: modules gs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _garden_spawn_flowers_buildings.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _123;

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
    float2 _208;
    float2 _232;
    float2 _154 = (((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims)) - 0.5f.xx) * 130.0f.xx;
    uint _161 = (((gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) / 2u) + (gl_GlobalInvocationID.x * 20u)) + 100u;
    uint _163 = _161 + (_161 << 10u);
    uint _165 = _163 ^ (_163 >> 6u);
    uint _167 = _165 + (_165 << 3u);
    uint _169 = _167 ^ (_167 >> 11u);
    float _175 = asfloat(((_169 + (_169 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _178 = _154 + lerp(-0.75f, 0.75f, _175).xx;
    uint _181 = uint((_175 * 100.0f) - 341.0f);
    uint _183 = _181 + (_181 << 10u);
    uint _185 = _183 ^ (_183 >> 6u);
    uint _187 = _185 + (_185 << 3u);
    uint _189 = _187 ^ (_187 >> 11u);
    float2 _198 = _154 + lerp(-0.75f, 0.75f, asfloat(((_189 + (_189 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx;
    int2 _199 = int2(gl_GlobalInvocationID.xy);
    uint _598;
    uint _599;
    bool _600;
    do
    {
        float2 _205;
        uint2 _210;
        bool _271;
        do
        {
            _205 = (_178 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            _208 = float2(path_mask_dims);
            _210 = uint2(_205 * _208);
            float4 _228 = terrain_heightmap.GatherRed(sampler_default, _205, int2(0, 0));
            _232 = 130.0f.xx / float2(terrain_heightmap_dims);
            float _233 = _228.z;
            float _234 = _228.y;
            float _236 = _228.w;
            float _237 = _228.x;
            bool _258;
            if (!(path_mask.Load(int3(_210, 0u)).x > 0.20000000298023223876953125f))
            {
                _258 = (int(rocky_terrain.Load(int3(_210, 0u)).x > 0.0f) & int(length(((float2((_233 + _234) - (_236 + _237), (_237 + _234) - (_236 + _233)) * 0.5f.xx) * 26.5f) / _232) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _258 = true;
            }
            bool _263;
            if (!_258)
            {
                _263 = water_mask.Load(int3(_210, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _263 = true;
            }
            bool _268;
            if (!_263)
            {
                _268 = foliage_exclusion_mask.SampleLevel(sampler_llb, _205, 0.0f).x > 0.0f;
            }
            else
            {
                _268 = true;
            }
            if (_268)
            {
                _271 = true;
                break;
            }
            _271 = false;
            break;
        } while(false);
        if (_271)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        if (garden_mask.Load(int3(_210, 0u)).y > 0.300000011920928955078125f)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        float4 _282 = buildings_mask.SampleLevel(sampler_default, _205, 0.0f);
        float _298;
        do
        {
            float _288 = abs(_282.y - 0.550000011920928955078125f);
            if (_288 > 0.300000011920928955078125f)
            {
                _298 = 0.0f;
                break;
            }
            float _292 = _288 * 3.3333332538604736328125f;
            _298 = 1.0f - ((_292 * _292) * (3.0f - (_288 * 6.666666507720947265625f)));
            break;
        } while(false);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _205, 0.0f).x > 0.999989986419677734375f)
        {
            _598 = _123;
            _599 = _123;
            _600 = false;
            break;
        }
        float2 _306 = _178 * 0.5f;
        int2 _308 = int2(floor(_306));
        float2 _309 = frac(_306);
        uint2 _310 = uint2(_308);
        uint _312 = _310.y;
        uint _314 = _312 + (_312 << 10u);
        uint _316 = _314 ^ (_314 >> 6u);
        uint _318 = _316 + (_316 << 3u);
        uint _320 = _318 ^ (_318 >> 11u);
        uint _326 = (((_310.x * 1664525u) + (_320 + (_320 << 15u))) + 1013904223u) * 1664525u;
        uint _328 = _326 ^ (_326 >> 11u);
        uint _331 = _328 ^ ((_328 << 7u) & 2636928640u);
        uint _334 = _331 ^ ((_331 << 15u) & 4022730752u);
        uint2 _342 = uint2(_308 + int2(1, 0));
        uint _344 = _342.y;
        uint _346 = _344 + (_344 << 10u);
        uint _348 = _346 ^ (_346 >> 6u);
        uint _350 = _348 + (_348 << 3u);
        uint _352 = _350 ^ (_350 >> 11u);
        uint _358 = (((_342.x * 1664525u) + (_352 + (_352 << 15u))) + 1013904223u) * 1664525u;
        uint _360 = _358 ^ (_358 >> 11u);
        uint _363 = _360 ^ ((_360 << 7u) & 2636928640u);
        uint _366 = _363 ^ ((_363 << 15u) & 4022730752u);
        uint2 _374 = uint2(_308 + int2(0, 1));
        uint _376 = _374.y;
        uint _378 = _376 + (_376 << 10u);
        uint _380 = _378 ^ (_378 >> 6u);
        uint _382 = _380 + (_380 << 3u);
        uint _384 = _382 ^ (_382 >> 11u);
        uint _390 = (((_374.x * 1664525u) + (_384 + (_384 << 15u))) + 1013904223u) * 1664525u;
        uint _392 = _390 ^ (_390 >> 11u);
        uint _395 = _392 ^ ((_392 << 7u) & 2636928640u);
        uint _398 = _395 ^ ((_395 << 15u) & 4022730752u);
        uint2 _406 = uint2(_308 + int2(1, 1));
        uint _408 = _406.y;
        uint _410 = _408 + (_408 << 10u);
        uint _412 = _410 ^ (_410 >> 6u);
        uint _414 = _412 + (_412 << 3u);
        uint _416 = _414 ^ (_414 >> 11u);
        uint _422 = (((_406.x * 1664525u) + (_416 + (_416 << 15u))) + 1013904223u) * 1664525u;
        uint _424 = _422 ^ (_422 >> 11u);
        uint _427 = _424 ^ ((_424 << 7u) & 2636928640u);
        uint _430 = _427 ^ ((_427 << 15u) & 4022730752u);
        float2 _440 = (_309 * _309) * (3.0f.xx - (_309 * 2.0f));
        float _441 = _440.x;
        float _442 = 1.0f - _441;
        float _444 = _440.y;
        if ((int(((((1.0f - _444) * (((asfloat(((_334 ^ (_334 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _442) + ((asfloat(((_366 ^ (_366 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _441))) + (((asfloat(((_398 ^ (_398 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _442) * _444)) + (((asfloat(((_430 ^ (_430 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _441) * _444)) > 0.60000002384185791015625f) & int(_282.x > 0.0f)) != 0)
        {
            int _463 = _199.x;
            int _464 = _199.y;
            uint _466 = uint(_463 + _464);
            uint _468 = _466 + (_466 << 10u);
            uint _470 = _468 ^ (_468 >> 6u);
            uint _472 = _470 + (_470 << 3u);
            uint _474 = _472 ^ (_472 >> 11u);
            float _481 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_474 + (_474 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _484 = _481.xxx * lerp(0.20000000298023223876953125f, 1.0f, _298);
            float _489 = min(_484.x, 0.75f);
            if (_489 < 0.4000000059604644775390625f)
            {
                _598 = 1013904223u;
                _599 = 1664525u;
                _600 = true;
                break;
            }
            float3 _496 = float3(_489, (_484.y * 1.2000000476837158203125f) * _481, min(_484.z, 0.75f)) * 2.0f;
            uint _513 = uint((_463 * _464) + _463);
            uint _515 = _513 + (_513 << 10u);
            uint _517 = _515 ^ (_515 >> 6u);
            uint _519 = _517 + (_517 << 3u);
            uint _521 = _519 ^ (_519 >> 11u);
            float _528 = 3.1400001049041748046875f * (asfloat(((_521 + (_521 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _529 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _530 = sin(_528);
            float _531 = cos(_528);
            float _532 = 1.0f - _531;
            float _533 = _529.x;
            float _534 = _532 * _533;
            float _537 = _529.y;
            float _538 = _534 * _537;
            float _539 = _529.z;
            float _540 = _539 * _530;
            float _542 = _532 * _539;
            float _543 = _542 * _533;
            float _544 = _537 * _530;
            float _547 = _532 * _537;
            float _550 = _547 * _539;
            float _551 = _533 * _530;
            float4x4 _562 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _178.x), float4(0.0f, 1.0f, 0.0f, 0.5f), float4(0.0f, 0.0f, 1.0f, _178.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_496.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _496.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _496.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_534 * _533) + _531, _538 - _540, _543 + _544, 0.0f), float4(_538 + _540, (_547 * _537) + _531, _550 - _551, 0.0f), float4(_543 - _544, _550 + _551, (_542 * _539) + _531, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            float _563 = _562[0].x;
            float _564 = _562[1].x;
            float _565 = _562[2].x;
            float _566 = _562[0].y;
            float _567 = _562[1].y;
            float _568 = _562[2].y;
            float _569 = _562[0].z;
            float _570 = _562[1].z;
            float _571 = _562[2].z;
            float _572 = _562[0].w;
            float _573 = _562[1].w;
            float _574 = _562[2].w;
            uint _591;
            __alias_draw_commands.InterlockedAdd(224, 1u, _591);
            uint _593 = uint(int(_591));
            float _595[12] = { _563, _564, _565, _566, _567, _568, _569, _570, _571, _572, _573 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_563, _564, _565), float3(_566, _567, _568), float3(_569, _570, _571), float3(_572, _573, _574))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _574 };
            Affine3Packed _596 = { _595 };
            FlowerXform _597 = { _596, (-1.0f).xxx, 0u };
            [unroll]
            for (int _4ident = 0; _4ident < 12; _4ident++)
            {
                __alias_flower_buffer.Store<float>(_4ident * 4 + _593 * 64 + 0, _597.transform.data[_4ident]);
            }
            __alias_flower_buffer.Store<float3>(_593 * 64 + 48, _597.color);
            __alias_flower_buffer.Store<uint>(_593 * 64 + 60, _597.flower_type);
        }
        _598 = 1013904223u;
        _599 = 1664525u;
        _600 = true;
        break;
    } while(false);
    do
    {
        float2 _606;
        uint2 _608;
        bool _658;
        do
        {
            _606 = (_198 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
            _608 = uint2(_606 * _208);
            float4 _619 = terrain_heightmap.GatherRed(sampler_default, _606, int2(0, 0));
            float _620 = _619.z;
            float _621 = _619.y;
            float _623 = _619.w;
            float _624 = _619.x;
            bool _645;
            if (!(path_mask.Load(int3(_608, 0u)).x > 0.20000000298023223876953125f))
            {
                _645 = (int(rocky_terrain.Load(int3(_608, 0u)).x > 0.0f) & int(length(((float2((_620 + _621) - (_623 + _624), (_624 + _621) - (_623 + _620)) * 0.5f.xx) * 26.5f) / _232) > 0.89999997615814208984375f)) != 0;
            }
            else
            {
                _645 = true;
            }
            bool _650;
            if (!_645)
            {
                _650 = water_mask.Load(int3(_608, 0u)).y > 0.300000011920928955078125f;
            }
            else
            {
                _650 = true;
            }
            bool _655;
            if (!_650)
            {
                _655 = foliage_exclusion_mask.SampleLevel(sampler_llb, _606, 0.0f).x > 0.0f;
            }
            else
            {
                _655 = true;
            }
            if (_655)
            {
                _658 = true;
                break;
            }
            _658 = false;
            break;
        } while(false);
        if (_658)
        {
            break;
        }
        if (garden_mask.Load(int3(_608, 0u)).y > 0.300000011920928955078125f)
        {
            break;
        }
        float4 _669 = buildings_mask.SampleLevel(sampler_default, _606, 0.0f);
        float _685;
        do
        {
            float _675 = abs(_669.y - 0.550000011920928955078125f);
            if (_675 > 0.4000000059604644775390625f)
            {
                _685 = 0.0f;
                break;
            }
            float _679 = _675 * 2.5f;
            _685 = 1.0f - ((_679 * _679) * (3.0f - (_675 * 5.0f)));
            break;
        } while(false);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _606, 0.0f).x > 0.999989986419677734375f)
        {
            break;
        }
        float2 _694 = (_198 * 0.5f) + 100.0f.xx;
        int2 _696 = int2(floor(_694));
        float2 _697 = frac(_694);
        uint2 _698 = uint2(_696);
        uint _700 = _698.y;
        uint _702 = _700 + (_700 << 10u);
        uint _704 = _702 ^ (_702 >> 6u);
        uint _706 = _704 + (_704 << 3u);
        uint _708 = _706 ^ (_706 >> 11u);
        bool _711 = _600 ? _600 : true;
        uint _712 = _600 ? _599 : 1664525u;
        uint _713 = _600 ? _598 : 1013904223u;
        uint _717 = (((_698.x * _712) + (_708 + (_708 << 15u))) + _713) * _712;
        uint _719 = _717 ^ (_717 >> 11u);
        uint _722 = _719 ^ ((_719 << 7u) & 2636928640u);
        uint _725 = _722 ^ ((_722 << 15u) & 4022730752u);
        uint2 _733 = uint2(_696 + int2(1, 0));
        uint _735 = _733.y;
        uint _737 = _735 + (_735 << 10u);
        uint _739 = _737 ^ (_737 >> 6u);
        uint _741 = _739 + (_739 << 3u);
        uint _743 = _741 ^ (_741 >> 11u);
        bool _746 = _711 ? _711 : true;
        uint _747 = _711 ? _712 : 1664525u;
        uint _748 = _711 ? _713 : 1013904223u;
        uint _752 = (((_733.x * _747) + (_743 + (_743 << 15u))) + _748) * _747;
        uint _754 = _752 ^ (_752 >> 11u);
        uint _757 = _754 ^ ((_754 << 7u) & 2636928640u);
        uint _760 = _757 ^ ((_757 << 15u) & 4022730752u);
        uint2 _768 = uint2(_696 + int2(0, 1));
        uint _770 = _768.y;
        uint _772 = _770 + (_770 << 10u);
        uint _774 = _772 ^ (_772 >> 6u);
        uint _776 = _774 + (_774 << 3u);
        uint _778 = _776 ^ (_776 >> 11u);
        bool _781 = _746 ? _746 : true;
        uint _782 = _746 ? _747 : 1664525u;
        uint _783 = _746 ? _748 : 1013904223u;
        uint _787 = (((_768.x * _782) + (_778 + (_778 << 15u))) + _783) * _782;
        uint _789 = _787 ^ (_787 >> 11u);
        uint _792 = _789 ^ ((_789 << 7u) & 2636928640u);
        uint _795 = _792 ^ ((_792 << 15u) & 4022730752u);
        uint2 _803 = uint2(_696 + int2(1, 1));
        uint _805 = _803.y;
        uint _807 = _805 + (_805 << 10u);
        uint _809 = _807 ^ (_807 >> 6u);
        uint _811 = _809 + (_809 << 3u);
        uint _813 = _811 ^ (_811 >> 11u);
        uint _816 = _781 ? _782 : 1664525u;
        uint _821 = (((_803.x * _816) + (_813 + (_813 << 15u))) + (_781 ? _783 : 1013904223u)) * _816;
        uint _823 = _821 ^ (_821 >> 11u);
        uint _826 = _823 ^ ((_823 << 7u) & 2636928640u);
        uint _829 = _826 ^ ((_826 << 15u) & 4022730752u);
        float2 _839 = (_697 * _697) * (3.0f.xx - (_697 * 2.0f));
        float _840 = _839.x;
        float _841 = 1.0f - _840;
        float _843 = _839.y;
        if ((int(((((1.0f - _843) * (((asfloat(((_725 ^ (_725 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _841) + ((asfloat(((_760 ^ (_760 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _840))) + (((asfloat(((_795 ^ (_795 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _841) * _843)) + (((asfloat(((_829 ^ (_829 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _840) * _843)) > 0.60000002384185791015625f) & int(_669.x > 0.0f)) != 0)
        {
            int _862 = _199.x;
            int _863 = _199.y;
            uint _865 = uint(_862 + _863);
            uint _867 = _865 + (_865 << 10u);
            uint _869 = _867 ^ (_867 >> 6u);
            uint _871 = _869 + (_869 << 3u);
            uint _873 = _871 ^ (_871 >> 11u);
            float _880 = lerp(0.800000011920928955078125f, 1.2000000476837158203125f, asfloat(((_873 + (_873 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _883 = _880.xxx * lerp(0.20000000298023223876953125f, 1.0f, _685);
            float _888 = min(_883.x, 0.75f);
            if (_888 < 0.4000000059604644775390625f)
            {
                break;
            }
            float3 _895 = float3(_888, (_883.y * 1.2000000476837158203125f) * _880, min(_883.z, 0.75f)) * 2.0f;
            uint _912 = uint((_862 * _863) + _862);
            uint _914 = _912 + (_912 << 10u);
            uint _916 = _914 ^ (_914 >> 6u);
            uint _918 = _916 + (_916 << 3u);
            uint _920 = _918 ^ (_918 >> 11u);
            float _927 = 3.1400001049041748046875f * (asfloat(((_920 + (_920 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
            float3 _928 = normalize(float3(0.0f, 1.0f, 0.0f));
            float _929 = sin(_927);
            float _930 = cos(_927);
            float _931 = 1.0f - _930;
            float _932 = _928.x;
            float _933 = _931 * _932;
            float _936 = _928.y;
            float _937 = _933 * _936;
            float _938 = _928.z;
            float _939 = _938 * _929;
            float _941 = _931 * _938;
            float _942 = _941 * _932;
            float _943 = _936 * _929;
            float _946 = _931 * _936;
            float _949 = _946 * _938;
            float _950 = _932 * _929;
            float4x4 _961 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _198.x), float4(0.0f, 1.0f, 0.0f, 0.5f), float4(0.0f, 0.0f, 1.0f, _198.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_895.x, 0.0f, 0.0f, 0.0f), float4(0.0f, _895.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _895.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_933 * _932) + _930, _937 - _939, _942 + _943, 0.0f), float4(_937 + _939, (_946 * _936) + _930, _949 - _950, 0.0f), float4(_942 - _943, _949 + _950, (_941 * _938) + _930, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
            float _962 = _961[0].x;
            float _963 = _961[1].x;
            float _964 = _961[2].x;
            float _965 = _961[0].y;
            float _966 = _961[1].y;
            float _967 = _961[2].y;
            float _968 = _961[0].z;
            float _969 = _961[1].z;
            float _970 = _961[2].z;
            float _971 = _961[0].w;
            float _972 = _961[1].w;
            float _973 = _961[2].w;
            uint _990;
            __alias_draw_commands.InterlockedAdd(224, 1u, _990);
            uint _992 = uint(int(_990));
            float _994[12] = { _962, _963, _964, _965, _966, _967, _968, _969, _970, _971, _972 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (mul(transpose(float4x3(float3(_962, _963, _964), float3(_965, _966, _967), float3(_968, _969, _970), float3(_971, _972, _973))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), _973 };
            Affine3Packed _995 = { _994 };
            FlowerXform _996 = { _995, (-1.0f).xxx, 1u };
            [unroll]
            for (int _5ident = 0; _5ident < 12; _5ident++)
            {
                __alias_flower_buffer.Store<float>(_5ident * 4 + _992 * 64 + 0, _996.transform.data[_5ident]);
            }
            __alias_flower_buffer.Store<float3>(_992 * 64 + 48, _996.color);
            __alias_flower_buffer.Store<uint>(_992 * 64 + 60, _996.flower_type);
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
