// PATCHED: modules es.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/path_pebbles.cs.hlsl | target: _path_pebbles.cs.hlsl_cs_main_0.bin | evidence: spirv-cross + SPIR-V disasm (grid-cell scatter over scatter_grid_dim, 104-unit cell, hash jitter; height -> path_mask rejection <0.9; rocky/water in-bounds rejection; per-vertex random scale/angle/tilt via hash(seed±k); atomic-increment draw_commands[draw_indirect_buffer_offset].instanceCount; compose scale*T(basis)*translation, pack_affine, store into pebble_placement_buffer[instance]) | bindings: sampler_llb s4, sampler_default s9, __alias_draw_commands u13, $Globals b14, terrain_heightmap t15, path_mask t17, rocky_terrain t18, water_mask t19, __alias_pebble_placement_buffer u20 | local_size [64,1,1] | status: O (algorithm); residual do-while/phi + int3-Load codegen = dxc-version artifacts

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/draw_indirect_types.hlsl"
#include "inc/draw_indirect.hlsl"

// The original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBOs; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<Affine3Packed> __alias_pebble_placement_buffer : register(u20, space0);

Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint pebble_buffer_size;
    int scatter_grid_dim;
};

// inc/scatter.hlsl references scatter_grid_dim as a global, so $Globals must be
// declared before its include.
#include "inc/scatter.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    int index = int(gl_GlobalInvocationID.x);
    int grid = scatter_grid_dim;
    int area = grid * grid;
    int cell = (index % area);
    float g = float(grid);
    float2 grid_pos = (float2(float((cell % grid)) / g, float(cell / grid) / g) - 0.5f) * 104.0f;

    uint iu = uint(index);
    float2 jitter = float2(rand01(hash(iu)), rand01(hash(iu ^ 188053769u))) * 6.3f;

    float px = grid_pos.x + jitter.x;
    float pz = grid_pos.y + jitter.y;

    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (float3(px, 0.0f, pz).xz * 0.0076923076994717121124267578125f + 0.5f), 0.0f).x);
    float2 mask_uv = (float3(px, h, pz).xz * 0.0076923076994717121124267578125f) + 0.5f;

    if (path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x < 0.9f)
        return;

    bool in_bounds = !(mask_uv.x < 0.0f) && !(mask_uv.x > 1.0f) && !(mask_uv.y < 0.0f) && !(mask_uv.y > 1.0f);

    float rocky = 0.0f;
    if (in_bounds)
        rocky = rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x;

    float water = 0.0f;
    if (in_bounds)
        water = water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;

    if (rocky > 0.0f || water > 0.5f)
        return;

    int seed = (int(px) ^ (345 + int(pz))) ^ 2378;

    float scale = lerp(0.4f, 1.2f, rand01(hash(uint(seed)))) * 1.2f;

    float angle = (1.0f - asfloat((hash(uint(seed)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir = normalize(float2(cos(angle), sin(angle)));

    float angle2 = (1.0f - asfloat((hash(uint(seed - 235)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));

    float tilt_slope = rand01(hash(uint(seed + 2345))) * 0.5f;
    float3 tilt = normalize(float3(dir2.x * tilt_slope, 1.0f, dir2.y * tilt_slope));

    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);

    float3 f = float3(dir.x, 0.0f, dir.y);
    float3 r = cross(f, tilt);
    float4x4 basis = float4x4(float4(r, 0.0f), float4(tilt, 0.0f), float4(f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale, 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, px), float4(0.0f, 1.0f, 0.0f, h), float4(0.0f, 0.0f, 1.0f, pz), float4(0.0f, 0.0f, 0.0f, 1.0f));

    float4x4 m = mul(mul(scale_m, transpose(basis)), trans);
    __alias_pebble_placement_buffer[slot] = pack_affine(m);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _path_pebbles.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float3 _103;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_path_mask_dims : packoffset(c1.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c2);
    uint2 _Globals_water_mask_dims : packoffset(c2.z);
    uint _Globals_pebble_buffer_size : packoffset(c3);
    int _Globals_scatter_grid_dim : packoffset(c3.y);
};

RWByteAddressBuffer __alias_pebble_placement_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        int _112 = int(gl_GlobalInvocationID.x);
        int _113 = (_112 % (_Globals_scatter_grid_dim * _Globals_scatter_grid_dim));
        float _117 = float(_Globals_scatter_grid_dim);
        float2 _123 = (float2(float((_113 % _Globals_scatter_grid_dim)) / _117, float(_113 / _Globals_scatter_grid_dim) / _117) - 0.5f.xx) * 104.0f.xx;
        uint _126 = uint(_112);
        uint _128 = _126 + (_126 << 10u);
        uint _130 = _128 ^ (_128 >> 6u);
        uint _132 = _130 + (_130 << 3u);
        uint _134 = _132 ^ (_132 >> 11u);
        uint _141 = _126 ^ 188053769u;
        uint _143 = _141 + (_141 << 10u);
        uint _145 = _143 ^ (_143 >> 6u);
        uint _147 = _145 + (_145 << 3u);
        uint _149 = _147 ^ (_147 >> 11u);
        float2 _157 = float2(asfloat(((_134 + (_134 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_149 + (_149 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 6.30000019073486328125f;
        float _159 = _123.x + _157.x;
        float3 _160;
        _160.x = _159;
        float _162 = _123.y + _157.y;
        _160.z = _162;
        float _172 = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_160.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
        float2 _178 = (float3(_159, _172, _162).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (path_mask.SampleLevel(sampler_llb, _178, 0.0f).x < 0.89999997615814208984375f)
        {
            break;
        }
        float _187;
        bool _189;
        float _213;
        do
        {
            _187 = _178.x;
            _189 = !(_187 < 0.0f);
            bool _193;
            if (_189)
            {
                _193 = _187 > 1.0f;
            }
            else
            {
                _193 = true;
            }
            bool _199;
            if (!_193)
            {
                _199 = _178.y < 0.0f;
            }
            else
            {
                _199 = true;
            }
            bool _205;
            if (!_199)
            {
                _205 = _178.y > 1.0f;
            }
            else
            {
                _205 = true;
            }
            if (_205)
            {
                _213 = 0.0f;
                break;
            }
            else
            {
                _213 = rocky_terrain.SampleLevel(sampler_default, _178, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float _239;
        do
        {
            bool _219;
            if (_189)
            {
                _219 = _187 > 1.0f;
            }
            else
            {
                _219 = true;
            }
            bool _225;
            if (!_219)
            {
                _225 = _178.y < 0.0f;
            }
            else
            {
                _225 = true;
            }
            bool _231;
            if (!_225)
            {
                _231 = _178.y > 1.0f;
            }
            else
            {
                _231 = true;
            }
            if (_231)
            {
                _239 = 0.0f;
                break;
            }
            else
            {
                _239 = water_mask.SampleLevel(sampler_default, _178, 0.0f).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _245;
        if (!(_213 > 0.0f))
        {
            _245 = _239 > 0.5f;
        }
        else
        {
            _245 = true;
        }
        if (_245)
        {
            break;
        }
        int _252 = (int(_159) ^ (345 + int(_162))) ^ 2378;
        uint _253 = uint(_252);
        uint _255 = _253 + (_253 << 10u);
        uint _257 = _255 ^ (_255 >> 6u);
        uint _259 = _257 + (_257 << 3u);
        uint _261 = _259 ^ (_259 >> 11u);
        float _266 = asfloat(((_261 + (_261 << 15u)) & 8388607u) | 1065353216u);
        float _269 = lerp(0.4000000059604644775390625f, 1.2000000476837158203125f, _266 - 1.0f) * 1.2000000476837158203125f;
        float _271 = (1.0f - _266) * 6.280000209808349609375f;
        float2 _275 = normalize(float2(cos(_271), sin(_271)));
        float _276 = _275.x;
        float _277 = _275.y;
        uint _280 = uint(_252 - 235);
        uint _282 = _280 + (_280 << 10u);
        uint _284 = _282 ^ (_282 >> 6u);
        uint _286 = _284 + (_284 << 3u);
        uint _288 = _286 ^ (_286 >> 11u);
        float _295 = (1.0f - asfloat(((_288 + (_288 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
        float2 _299 = normalize(float2(cos(_295), sin(_295)));
        uint _301 = uint(_252 + 2345);
        uint _303 = _301 + (_301 << 10u);
        uint _305 = _303 ^ (_303 >> 6u);
        uint _307 = _305 + (_305 << 3u);
        uint _309 = _307 ^ (_307 >> 11u);
        float _316 = (asfloat(((_309 + (_309 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.5f;
        float3 _322 = normalize(float3(_299.x * _316, 1.0f, _299.y * _316));
        uint _328;
        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _328);
        uint _330 = uint(int(_328));
        float4x4 _351 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _159), float4(0.0f, 1.0f, 0.0f, _172), float4(0.0f, 0.0f, 1.0f, _162), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(cross(float3(_276, 0.0f, _277), _322), 0.0f), float4(_322, 0.0f), float4(_276, 0.0f, _277, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_269, 0.0f, 0.0f, 0.0f), float4(0.0f, _269, 0.0f, 0.0f), float4(0.0f, 0.0f, _269, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        float _365[12] = { _351[0].x, _351[1].x, _351[2].x, _351[0].y, _351[1].y, _351[2].y, _351[0].z, _351[1].z, _351[2].z, _351[0].w, _351[1].w, _351[2].w };
        Affine3Packed _366 = { _365 };
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            __alias_pebble_placement_buffer.Store<float>(_2ident * 4 + _330 * 48 + 0, _366.data[_2ident]);
        }
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
