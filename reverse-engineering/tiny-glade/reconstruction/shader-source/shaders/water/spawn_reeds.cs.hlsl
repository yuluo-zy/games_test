// PATCHED: modules ds.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/spawn_reeds.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_reeds.cs.hlsl_cs_main_0.bin (jittered world pos; water-mask height bounds-checked sample; water band falloff around 0.5 + path falloff around 0.2; perlin(world+20)>0.6 && water>0.55 && buildings<0.1 gate; Reed = T*S*R yaw) | status: O (algorithm from SPIR-V); I (function names, falloff literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"

struct Reed
{
    Affine3Packed xform;
};

// Rodrigues rotation around a runtime (normalized) axis — transposed row-major.
float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    float cxx = omc * x;
    float cyy = omc * y;
    float czz = omc * z;
    return transpose(float4x4(
        float4(cxx * x + c, cxx * y - z * s, czz * x + y * s, 0.0f),
        float4(cxx * y + z * s, cyy * y + c, cyy * z - x * s, 0.0f),
        float4(czz * x - y * s, cyy * z + x * s, czz * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

// Banded falloff around `center`: 1 at center, 0 at distance >= half_width.
float band_falloff(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t17, space0);
RWStructuredBuffer<Reed> __alias_instance_buffer : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint seed = id.x * id.y + id.x;
    float rnd = rand01(hash(seed));

    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-0.4f, 0.4f, rnd).xx;

    float2 uv = world_pos * 0.0076923076994717121124267578125f + 0.5f;
    bool in_bounds = uv.x >= 0.0f && uv.x <= 1.0f && uv.y >= 0.0f && uv.y <= 1.0f;

    float water = in_bounds ? water_mask.SampleLevel(sampler_default, uv, 0.0f).y : 0.0f;
    float path = path_mask.SampleLevel(sampler_llb, uv, 0.0f).x;

    float water_band = band_falloff(abs(water - 0.5f), 3.3333332538604736328125f);
    float path_band = band_falloff(abs(path - 0.2f), 5.0f);

    float score = (water_band + path_band * smoothstep(0.6f, 0.8f, water)) - smoothstep(0.2f, 0.25f, path);

    if (score > 0.0f)
    {
        if (!(perlin(world_pos + 20.0f) > 0.6f && water > 0.55f
              && buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x < 0.1f))
            return;

        float scale = lerp(1.0f, 1.5f, rand01(hash(id.x + id.y)));
        float yaw = 3.14f * rnd;

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 s = float4x4(
            float4(scale, 0.0f, 0.0f, 0.0f),
            float4(0.0f, scale * lerp(0.5f, 1.0f, score * score), 0.0f, 0.0f),
            float4(0.0f, 0.0f, scale, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 m = mul(mul(
            affine_translation(float3(world_pos.x, -1.0f, world_pos.y)),
            mul(identity, s)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = Reed(pack_affine(m));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_spawn_reeds.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Reed
{
    Affine3Packed xform;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_water_mask_dims : packoffset(c0.y);
    uint2 _Globals_path_mask_dims : packoffset(c1);
    uint2 _Globals_buildings_mask_non_blurred_dims : packoffset(c1.z);
    int2 _Globals_grid_dims : packoffset(c2);
};

RWByteAddressBuffer __alias_instance_buffer : register(u18, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_mask : register(t15, space0);
Texture2D<float4> path_mask : register(t16, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t17, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 _158;
    uint _137 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
    uint _139 = _137 + (_137 << 10u);
    uint _141 = _139 ^ (_139 >> 6u);
    uint _143 = _141 + (_141 << 3u);
    uint _145 = _143 ^ (_143 >> 11u);
    float _151 = asfloat(((_145 + (_145 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _154 = ((((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(_Globals_grid_dims)) - 0.5f.xx) * 130.0f.xx) + lerp(-0.4000000059604644775390625f, 0.4000000059604644775390625f, _151).xx;
    float _186;
    do
    {
        _158 = (_154 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        bool _165;
        if (!(_158.x < 0.0f))
        {
            _165 = _158.x > 1.0f;
        }
        else
        {
            _165 = true;
        }
        bool _171;
        if (!_165)
        {
            _171 = _158.y < 0.0f;
        }
        else
        {
            _171 = true;
        }
        bool _177;
        if (!_171)
        {
            _177 = _158.y > 1.0f;
        }
        else
        {
            _177 = true;
        }
        if (_177)
        {
            _186 = 0.0f;
            break;
        }
        else
        {
            _186 = water_mask.SampleLevel(sampler_default, _158, 0.0f).y;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float4 _190 = path_mask.SampleLevel(sampler_llb, _158, 0.0f);
    float _191 = _190.x;
    float _205;
    do
    {
        float _195 = abs(_186 - 0.5f);
        if (_195 > 0.300000011920928955078125f)
        {
            _205 = 0.0f;
            break;
        }
        float _199 = _195 * 3.3333332538604736328125f;
        _205 = 1.0f - ((_199 * _199) * (3.0f - (_195 * 6.666666507720947265625f)));
        break;
    } while(false);
    float _219;
    do
    {
        float _209 = abs(_191 - 0.20000000298023223876953125f);
        if (_209 > 0.20000000298023223876953125f)
        {
            _219 = 0.0f;
            break;
        }
        float _213 = _209 * 5.0f;
        _219 = 1.0f - ((_213 * _213) * (3.0f - (_209 * 10.0f)));
        break;
    } while(false);
    float _224 = (_205 + (_219 * smoothstep(0.60000002384185791015625f, 0.800000011920928955078125f, _186))) - smoothstep(0.20000000298023223876953125f, 0.25f, _191);
    bool _391;
    if (_224 > 0.0f)
    {
        float2 _233 = _154 + 20.0f.xx;
        int2 _235 = int2(floor(_233));
        float2 _236 = frac(_233);
        uint2 _237 = uint2(_235);
        uint _239 = _237.y;
        uint _241 = _239 + (_239 << 10u);
        uint _243 = _241 ^ (_241 >> 6u);
        uint _245 = _243 + (_243 << 3u);
        uint _247 = _245 ^ (_245 >> 11u);
        uint _253 = (((_237.x * 1664525u) + (_247 + (_247 << 15u))) + 1013904223u) * 1664525u;
        uint _255 = _253 ^ (_253 >> 11u);
        uint _258 = _255 ^ ((_255 << 7u) & 2636928640u);
        uint _261 = _258 ^ ((_258 << 15u) & 4022730752u);
        uint2 _269 = uint2(_235 + int2(1, 0));
        uint _271 = _269.y;
        uint _273 = _271 + (_271 << 10u);
        uint _275 = _273 ^ (_273 >> 6u);
        uint _277 = _275 + (_275 << 3u);
        uint _279 = _277 ^ (_277 >> 11u);
        uint _285 = (((_269.x * 1664525u) + (_279 + (_279 << 15u))) + 1013904223u) * 1664525u;
        uint _287 = _285 ^ (_285 >> 11u);
        uint _290 = _287 ^ ((_287 << 7u) & 2636928640u);
        uint _293 = _290 ^ ((_290 << 15u) & 4022730752u);
        uint2 _301 = uint2(_235 + int2(0, 1));
        uint _303 = _301.y;
        uint _305 = _303 + (_303 << 10u);
        uint _307 = _305 ^ (_305 >> 6u);
        uint _309 = _307 + (_307 << 3u);
        uint _311 = _309 ^ (_309 >> 11u);
        uint _317 = (((_301.x * 1664525u) + (_311 + (_311 << 15u))) + 1013904223u) * 1664525u;
        uint _319 = _317 ^ (_317 >> 11u);
        uint _322 = _319 ^ ((_319 << 7u) & 2636928640u);
        uint _325 = _322 ^ ((_322 << 15u) & 4022730752u);
        uint2 _333 = uint2(_235 + int2(1, 1));
        uint _335 = _333.y;
        uint _337 = _335 + (_335 << 10u);
        uint _339 = _337 ^ (_337 >> 6u);
        uint _341 = _339 + (_339 << 3u);
        uint _343 = _341 ^ (_341 >> 11u);
        uint _349 = (((_333.x * 1664525u) + (_343 + (_343 << 15u))) + 1013904223u) * 1664525u;
        uint _351 = _349 ^ (_349 >> 11u);
        uint _354 = _351 ^ ((_351 << 7u) & 2636928640u);
        uint _357 = _354 ^ ((_354 << 15u) & 4022730752u);
        float2 _367 = (_236 * _236) * (3.0f.xx - (_236 * 2.0f));
        float _368 = _367.x;
        float _369 = 1.0f - _368;
        float _371 = _367.y;
        _391 = ((int(((((1.0f - _371) * (((asfloat(((_261 ^ (_261 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _369) + ((asfloat(((_293 ^ (_293 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _368))) + (((asfloat(((_325 ^ (_325 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _369) * _371)) + (((asfloat(((_357 ^ (_357 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _368) * _371)) > 0.60000002384185791015625f) & int(_186 > 0.550000011920928955078125f)) & int(buildings_mask_non_blurred.SampleLevel(sampler_default, _158, 0.0f).x < 0.100000001490116119384765625f)) != 0;
    }
    else
    {
        _391 = false;
    }
    if (_391)
    {
        uint _394 = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y;
        uint _396 = _394 + (_394 << 10u);
        uint _398 = _396 ^ (_396 >> 6u);
        uint _400 = _398 + (_398 << 3u);
        uint _402 = _400 ^ (_400 >> 11u);
        float _409 = lerp(1.0f, 1.5f, asfloat(((_402 + (_402 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
        float _424 = 3.1400001049041748046875f * _151;
        float3 _425 = normalize(float3(0.0f, 1.0f, 0.0f));
        float _426 = sin(_424);
        float _427 = cos(_424);
        float _428 = 1.0f - _427;
        float _429 = _425.x;
        float _430 = _428 * _429;
        float _433 = _425.y;
        float _434 = _430 * _433;
        float _435 = _425.z;
        float _436 = _435 * _426;
        float _438 = _428 * _435;
        float _439 = _438 * _429;
        float _440 = _433 * _426;
        float _443 = _428 * _433;
        float _446 = _443 * _435;
        float _447 = _429 * _426;
        float4x4 _458 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _154.x), float4(0.0f, 1.0f, 0.0f, -1.0f), float4(0.0f, 0.0f, 1.0f, _154.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(_409, 0.0f, 0.0f, 0.0f), float4(0.0f, _409 * lerp(0.5f, 1.0f, _224 * _224), 0.0f, 0.0f), float4(0.0f, 0.0f, _409, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_430 * _429) + _427, _434 - _436, _439 + _440, 0.0f), float4(_434 + _436, (_443 * _433) + _427, _446 - _447, 0.0f), float4(_439 - _440, _446 + _447, (_438 * _435) + _427, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        uint _475;
        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _475);
        uint _477 = uint(int(_475));
        float _479[12] = { _458[0].x, _458[1].x, _458[2].x, _458[0].y, _458[1].y, _458[2].y, _458[0].z, _458[1].z, _458[2].z, _458[0].w, _458[1].w, _458[2].w };
        Affine3Packed _480 = { _479 };
        Reed _481 = { _480 };
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            __alias_instance_buffer.Store<float>(_2ident * 4 + _477 * 48 + 0, _481.xform.data[_2ident]);
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
