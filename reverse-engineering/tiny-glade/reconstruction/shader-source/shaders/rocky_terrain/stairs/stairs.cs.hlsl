// PATCHED: modules rs.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/stairs.cs.hlsl
// target: _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   per-cell marching-squares contouring over the terrain height field. A contouring-grid cell is
//   kept only when at least one corner sits on rock (rocky_terrain.x clamp>0.2), near a path
//   (smoothstep(path_mask.x)>0.99) and off water (water_mask.y<0.5). For each stair iso-level
//   (lerp(0.8,10,level/16)) the 4-bit corner-state mask selects the crossing edges (marching_squares.inc),
//   find_root (inc/find_root.hlsl, via place_stairs.inc) bisects each edge to the target height, and
//   place_stairs.inc writes a Stair{Affine3Packed} into __alias_stairs_buffer and bumps
//   __alias_draw_commands[draw_indirect_buffer_offset].instanceCount; pebbles reuse the buffer at
//   pebble_buffer_offset and the pebble_indirect_draw_idx command. Binding/entry/LocalSize verbatim;
//   the two SSBOs are plain RWStructuredBuffer (target module emits only OpCapability Shader, no
//   SPV_EXT_descriptor_indexing, so NO [[vk::ext_capability(5302)]] here — verified from the bin).
// bindings: sampler_llb s4, sampler_default s9, __alias_draw_commands u13 (SSBO), $Globals b14 (UBO),
//   terrain_heightmap t15, rocky_terrain t17, path_mask t18, water_mask t19, __alias_stairs_buffer u20 (SSBO)
// entry: cs_main | local_size [8,8,1] | gid uint3 (SV_DispatchThreadID)
// status: O (algorithm); residual do-while/phi/switch + int3-Load codegen = dxc-version artifacts;
//   OpSource/OpString/OpLine count drift = reconstructed inc-layer extra include edges (inc/* read-only)

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/draw_indirect_types.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

struct Stair
{
    Affine3Packed transform;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
RWStructuredBuffer<Stair> __alias_stairs_buffer : register(u20, space0);

Texture2D<float4> rocky_terrain : register(t17, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 contouring_grid_dims;
    uint pebble_indirect_draw_idx;
    uint pebble_buffer_offset;
};

// terrain_height(float2) is now provided by inc/terrain_heightmap.inc.hlsl
// (adjudicated inc-fix #3); it samples the global terrain_heightmap texture
// declared below at register t15.

// Bounds-checked rocky-terrain sample; outside the [0,1]^2 uv domain it reads 0.
float4 sample_rocky(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
}

// Bounds-checked water-mask sample; outside the [0,1]^2 uv domain it reads 0.
float water_mask_value(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return water_mask.SampleLevel(sampler_default, uv, 0.0f).y;
}

// The three stair include files (this shader's contouring logic), dependency
// chain: isolines.inc -> marching_squares.inc -> place_stairs.inc -> inc/find_root.hlsl.
#include "isolines.inc.hlsl"

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x > contouring_grid_dims.x ||
        gl_GlobalInvocationID.y > contouring_grid_dims.y)
        return;

    float2 grid_dims = float2(contouring_grid_dims);
    float2 center = (float2(gl_GlobalInvocationID.x + 0.5f, gl_GlobalInvocationID.y + 0.5f) / grid_dims - 0.5f) * 130.0f;
    float2 half = 65.0f / grid_dims;

    float2 corners[4];
    corners[0] = center + float2(-half.x, -half.y);
    corners[1] = center + float2(half.x, -half.y);
    corners[2] = center + float2(half.x, half.y);
    corners[3] = center + float2(-half.x, half.y);

    int eligible = 0;
    for (int i = 0; i < 4; i++)
    {
        float rocky = clamp(sample_rocky(corners[i]).x, 0.0f, 1.0f);
        bool near_path = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, corners[i] * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) > 0.99f;
        bool off_water = water_mask_value(corners[i]) < 0.5f;
        if (rocky > 0.2f && near_path && off_water)
            eligible++;
    }
    if (eligible == 0)
        return;

    float4 corner_heights;
    for (int i = 0; i < 4; i++)
        corner_heights[i] = terrain_height(corners[i]);

    place_stair_isolines(corners, corner_heights);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Stair
{
    Affine3Packed transform;
};

static uint _132;

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int _Globals_draw_indirect_buffer_offset : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_rocky_terrain_dims : packoffset(c1.z);
    uint2 _Globals_path_mask_dims : packoffset(c2);
    uint2 _Globals_water_mask_dims : packoffset(c2.z);
    uint2 _Globals_contouring_grid_dims : packoffset(c3);
    uint _Globals_pebble_indirect_draw_idx : packoffset(c3.z);
    uint _Globals_pebble_buffer_offset : packoffset(c3.w);
};

RWByteAddressBuffer __alias_stairs_buffer : register(u20, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> rocky_terrain : register(t17, space0);
Texture2D<float4> path_mask : register(t18, space0);
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
        bool _165;
        if (!(gl_GlobalInvocationID.x > _Globals_contouring_grid_dims.x))
        {
            _165 = gl_GlobalInvocationID.y > _Globals_contouring_grid_dims.y;
        }
        else
        {
            _165 = true;
        }
        if (_165)
        {
            break;
        }
        float _170 = float(_Globals_contouring_grid_dims.x);
        float _176 = float(_Globals_contouring_grid_dims.y);
        float2 _181 = (float2((float(gl_GlobalInvocationID.x) + 0.5f) / _170, (float(gl_GlobalInvocationID.y) + 0.5f) / _176) - 0.5f.xx) * 130.0f.xx;
        float2 _182 = 65.0f.xx / float2(_170, _176);
        float _183 = _182.x;
        float _184 = -_183;
        float _185 = _182.y;
        float _186 = -_185;
        float2 _149[4];
        _149[0] = _181 + float2(_184, _186);
        _149[1] = _181 + float2(_183, _186);
        _149[2] = _181 + _182;
        _149[3] = _181 + float2(_184, _185);
        int _199;
        _199 = 0;
        int _200;
        for (int _202 = 0; _202 < 4; _199 = _200, _202++)
        {
            float _240;
            do
            {
                float2 _212 = (_149[_202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                float _213 = _212.x;
                bool _219;
                if (!(_213 < 0.0f))
                {
                    _219 = _213 > 1.0f;
                }
                else
                {
                    _219 = true;
                }
                bool _225;
                if (!_219)
                {
                    _225 = _212.y < 0.0f;
                }
                else
                {
                    _225 = true;
                }
                bool _231;
                if (!_225)
                {
                    _231 = _212.y > 1.0f;
                }
                else
                {
                    _231 = true;
                }
                if (_231)
                {
                    _240 = 0.0f;
                    break;
                }
                else
                {
                    _240 = rocky_terrain.SampleLevel(sampler_default, _212, 0.0f).x;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            float _282;
            do
            {
                float2 _254 = (_149[_202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                float _255 = _254.x;
                bool _261;
                if (!(_255 < 0.0f))
                {
                    _261 = _255 > 1.0f;
                }
                else
                {
                    _261 = true;
                }
                bool _267;
                if (!_261)
                {
                    _267 = _254.y < 0.0f;
                }
                else
                {
                    _267 = true;
                }
                bool _273;
                if (!_267)
                {
                    _273 = _254.y > 1.0f;
                }
                else
                {
                    _273 = true;
                }
                if (_273)
                {
                    _282 = 0.0f;
                    break;
                }
                else
                {
                    _282 = water_mask.SampleLevel(sampler_default, _254, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            if (((int(clamp(_240, 0.0f, 1.0f) > 0.20000000298023223876953125f) & int(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_149[_202] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) > 0.9900000095367431640625f)) & int(_282 < 0.5f)) != 0)
            {
                _200 = _199 + 1;
            }
            else
            {
                _200 = _199;
            }
        }
        if (_199 == 0)
        {
            break;
        }
        float4 _150 = 0.0f.xxxx;
        for (int _300 = 0; _300 < 4; )
        {
            _150[uint(_300)] = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[_300] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
            _300++;
            continue;
        }
        int _339 = min(int(ceil((min(min(_150.x, _150.y), min(_150.z, _150.w)) - 0.900000035762786865234375f) * 1.84782612323760986328125f)), 16);
        int _344 = max(int(floor((max(max(_150.x, _150.y), max(_150.z, _150.w)) + (-0.699999988079071044921875f)) * 1.84782612323760986328125f)), 0);
        uint _347;
        uint _350;
        bool _352;
        bool _354;
        uint _346 = 0u;
        uint _349 = 0u;
        bool _351 = false;
        bool _353 = false;
        int _355 = _339;
        for (; _355 <= _344; _346 = _347, _349 = _350, _351 = _352, _353 = _354, _355++)
        {
            float _362 = lerp(0.800000011920928955078125f, 10.0f, float(_355) * 0.0625f);
            uint _364;
            _364 = 0u;
            uint _365;
            for (int _367 = 0; _367 < 4; _364 = _365, _367++)
            {
                if (lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[_367] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) >= _362)
                {
                    _365 = _364 | (8u >> (uint(_367) & 31u));
                }
                else
                {
                    _365 = _364;
                }
            }
            int _389 = int(_364);
            bool _396;
            if (!(_389 == 0))
            {
                _396 = _389 == 15;
            }
            else
            {
                _396 = true;
            }
            if (_396)
            {
                _347 = _346;
                _350 = _349;
                _352 = _351;
                _354 = _353;
            }
            else
            {
                bool _405;
                if (!(_389 == 1))
                {
                    _405 = _389 == 14;
                }
                else
                {
                    _405 = true;
                }
                uint _4587;
                uint _4588;
                bool _4589;
                bool _4590;
                if (_405)
                {
                    uint _4583;
                    uint _4584;
                    bool _4585;
                    bool _4586;
                    do
                    {
                        float _4180 = -_362;
                        float2 _4191 = (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                        float _4198;
                        float _4201;
                        float2 _4203;
                        float2 _4205;
                        _4198 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                        _4201 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _4191, 0.0f).x);
                        _4203 = _149[0];
                        _4205 = _149[3];
                        for (uint _4207 = 0u; _4207 < 2u; )
                        {
                            float2 _4211 = _4203 + _4205;
                            float2 _4212 = _4211 * 0.5f;
                            float _4219 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_4212 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            float _4220 = _4219 * _4198;
                            bool _4221 = _4220 < 0.0f;
                            bool2 _4222 = _4221.xx;
                            _4198 = _4221 ? _4198 : _4219;
                            _4201 = _4221 ? _4219 : _4201;
                            _4203 = float2(_4222.x ? _4203.x : _4212.x, _4222.y ? _4203.y : _4212.y);
                            _4205 = float2(_4222.x ? _4212.x : _4205.x, _4222.y ? _4212.y : _4205.y);
                            _4207++;
                            continue;
                        }
                        float2 _4227 = lerp(_4205, _4203, clamp(_4201 / (_4201 - _4198), 0.0f, 1.0f).xx);
                        float _4228 = _4227.x;
                        float3 _4230 = float3(_4228, _362, _4227.y);
                        float _4245;
                        float _4248;
                        float2 _4250;
                        float2 _4252;
                        _4245 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _4191, 0.0f).x);
                        _4248 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                        _4250 = _149[3];
                        _4252 = _149[2];
                        for (uint _4254 = 0u; _4254 < 2u; )
                        {
                            float2 _4258 = _4250 + _4252;
                            float2 _4259 = _4258 * 0.5f;
                            float _4266 = _4180 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_4259 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            float _4267 = _4266 * _4245;
                            bool _4268 = _4267 < 0.0f;
                            bool2 _4269 = _4268.xx;
                            _4245 = _4268 ? _4245 : _4266;
                            _4248 = _4268 ? _4266 : _4248;
                            _4250 = float2(_4269.x ? _4250.x : _4259.x, _4269.y ? _4250.y : _4259.y);
                            _4252 = float2(_4269.x ? _4259.x : _4252.x, _4269.y ? _4259.y : _4252.y);
                            _4254++;
                            continue;
                        }
                        float2 _4283;
                        float _4284;
                        bool _4286;
                        float2 _4274 = lerp(_4252, _4250, clamp(_4248 / (_4248 - _4245), 0.0f, 1.0f).xx);
                        float _4276 = _4274.y;
                        float3 _4277 = float3(_4274.x, _362, _4276);
                        float3 _4278 = lerp(_4230, _4277, 0.5f.xxx);
                        float _4310;
                        do
                        {
                            _4283 = (_4278.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            _4284 = _4283.x;
                            _4286 = !(_4284 < 0.0f);
                            bool _4290;
                            if (_4286)
                            {
                                _4290 = _4284 > 1.0f;
                            }
                            else
                            {
                                _4290 = true;
                            }
                            bool _4296;
                            if (!_4290)
                            {
                                _4296 = _4283.y < 0.0f;
                            }
                            else
                            {
                                _4296 = true;
                            }
                            bool _4302;
                            if (!_4296)
                            {
                                _4302 = _4283.y > 1.0f;
                            }
                            else
                            {
                                _4302 = true;
                            }
                            if (_4302)
                            {
                                _4310 = 0.0f;
                                break;
                            }
                            else
                            {
                                _4310 = rocky_terrain.SampleLevel(sampler_default, _4283, 0.0f).x;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (_4310 < 0.00999999977648258209228515625f)
                        {
                            _4583 = _346;
                            _4584 = _349;
                            _4585 = _351;
                            _4586 = _353;
                            break;
                        }
                        bool _4334;
                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_4230.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                        {
                            _4334 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_4277.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                        }
                        else
                        {
                            _4334 = false;
                        }
                        if (_4334)
                        {
                            _4583 = _346;
                            _4584 = _349;
                            _4585 = _351;
                            _4586 = _353;
                            break;
                        }
                        float _4337 = distance(_4230, _4277);
                        if (_4337 < 0.20000000298023223876953125f)
                        {
                            _4583 = _346;
                            _4584 = _349;
                            _4585 = _351;
                            _4586 = _353;
                            break;
                        }
                        float4 _4342 = terrain_heightmap.GatherRed(sampler_default, _4283, int2(0, 0));
                        float _4347 = _4342.z;
                        float _4348 = _4342.y;
                        float _4350 = _4342.w;
                        float _4351 = _4342.x;
                        float2 _4360 = ((float2((_4347 + _4348) - (_4350 + _4351), (_4351 + _4348) - (_4350 + _4347)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                        float _4393;
                        do
                        {
                            bool _4373;
                            if (_4286)
                            {
                                _4373 = _4284 > 1.0f;
                            }
                            else
                            {
                                _4373 = true;
                            }
                            bool _4379;
                            if (!_4373)
                            {
                                _4379 = _4283.y < 0.0f;
                            }
                            else
                            {
                                _4379 = true;
                            }
                            bool _4385;
                            if (!_4379)
                            {
                                _4385 = _4283.y > 1.0f;
                            }
                            else
                            {
                                _4385 = true;
                            }
                            if (_4385)
                            {
                                _4393 = 0.0f;
                                break;
                            }
                            else
                            {
                                _4393 = rocky_terrain.SampleLevel(sampler_default, _4283, 0.0f).x;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        float _4419;
                        do
                        {
                            bool _4399;
                            if (_4286)
                            {
                                _4399 = _4284 > 1.0f;
                            }
                            else
                            {
                                _4399 = true;
                            }
                            bool _4405;
                            if (!_4399)
                            {
                                _4405 = _4283.y < 0.0f;
                            }
                            else
                            {
                                _4405 = true;
                            }
                            bool _4411;
                            if (!_4405)
                            {
                                _4411 = _4283.y > 1.0f;
                            }
                            else
                            {
                                _4411 = true;
                            }
                            if (_4411)
                            {
                                _4419 = 0.0f;
                                break;
                            }
                            else
                            {
                                _4419 = rocky_terrain.SampleLevel(sampler_default, _4283, 0.0f).z;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        float3 _4423 = cross(normalize(float3(0.0f, _4360.y, 1.0f)), normalize(float3(1.0f, _4360.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_4393 + _4419, 0.0f, 1.0f));
                        float3 _4424 = _4278 - _4423;
                        float3 _4428 = normalize(_4277 - _4230);
                        uint _4433 = uint(((_4228 + _362) + _4276) * 100.0f);
                        float4x4 _4451 = transpose(float4x4(float4(_4428, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_4428, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                        float4x4 _4453 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _4424.x), float4(0.0f, 1.0f, 0.0f, _4424.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _4424.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_4451, float4x4(float4(_4337, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                        uint _4458;
                        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _4458);
                        uint _4472 = uint(int(_4458));
                        float _4474[12] = { _4453[0].x, _4453[1].x, _4453[2].x, _4453[0].y, _4453[1].y, _4453[2].y, _4453[0].z, _4453[1].z, _4453[2].z, _4453[0].w, _4453[1].w, _4453[2].w };
                        Affine3Packed _4475 = { _4474 };
                        Stair _4476 = { _4475 };
                        [unroll]
                        for (int _20ident = 0; _20ident < 12; _20ident++)
                        {
                            __alias_stairs_buffer.Store<float>(_20ident * 4 + _4472 * 48 + 0, _4476.transform.data[_20ident]);
                        }
                        uint _4477 = _4433 ^ 23784u;
                        uint _4479 = _4477 + (_4477 << 10u);
                        uint _4481 = _4479 ^ (_4479 >> 6u);
                        uint _4483 = _4481 + (_4481 << 3u);
                        uint _4485 = _4483 ^ (_4483 >> 11u);
                        bool _4488 = _353 ? _353 : true;
                        uint _4489 = _353 ? _349 : 8388607u;
                        bool _4490 = _351 ? _351 : true;
                        uint _4491 = _351 ? _346 : 1065353216u;
                        uint _4579;
                        uint _4580;
                        bool _4581;
                        bool _4582;
                        if ((asfloat(((_4485 + (_4485 << 15u)) & _4489) | _4491) - 1.0f) > 0.85000002384185791015625f)
                        {
                            uint _4499 = _4433 ^ 90455u;
                            uint _4501 = _4499 + (_4499 << 10u);
                            uint _4503 = _4501 ^ (_4501 >> 6u);
                            uint _4505 = _4503 + (_4503 << 3u);
                            uint _4507 = _4505 ^ (_4505 >> 11u);
                            bool _4510 = _4488 ? _4488 : true;
                            uint _4511 = _4488 ? _4489 : 8388607u;
                            bool _4512 = _4490 ? _4490 : true;
                            uint _4513 = _4490 ? _4491 : 1065353216u;
                            float _4518 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_4507 + (_4507 << 15u)) & _4511) | _4513) - 1.0f);
                            uint _4519 = _4433 ^ 3845u;
                            uint _4521 = _4519 + (_4519 << 10u);
                            uint _4523 = _4521 ^ (_4521 >> 6u);
                            uint _4525 = _4523 + (_4523 << 3u);
                            uint _4527 = _4525 ^ (_4525 >> 11u);
                            uint _4531 = _4510 ? _4511 : 8388607u;
                            uint _4533 = _4512 ? _4513 : 1065353216u;
                            float3 _4540 = lerp(_4230, _4277, (asfloat(((_4527 + (_4527 << 15u)) & _4531) | _4533) - 1.0f).xxx) - _4423;
                            uint _4544;
                            __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _4544);
                            uint _4549 = _Globals_pebble_buffer_offset + uint(int(_4544));
                            float4x4 _4562 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _4540.x), float4(0.0f, 1.0f, 0.0f, _4540.y), float4(0.0f, 0.0f, 1.0f, _4540.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_4451, float4x4(float4(_4518, 0.0f, 0.0f, 0.0f), float4(0.0f, _4518, 0.0f, 0.0f), float4(0.0f, 0.0f, _4518, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            float _4576[12] = { _4562[0].x, _4562[1].x, _4562[2].x, _4562[0].y, _4562[1].y, _4562[2].y, _4562[0].z, _4562[1].z, _4562[2].z, _4562[0].w, _4562[1].w, _4562[2].w };
                            Affine3Packed _4577 = { _4576 };
                            Stair _4578 = { _4577 };
                            [unroll]
                            for (int _21ident = 0; _21ident < 12; _21ident++)
                            {
                                __alias_stairs_buffer.Store<float>(_21ident * 4 + _4549 * 48 + 0, _4578.transform.data[_21ident]);
                            }
                            _4579 = _4533;
                            _4580 = _4531;
                            _4581 = _4512 ? _4512 : true;
                            _4582 = _4510 ? _4510 : true;
                        }
                        else
                        {
                            _4579 = _4491;
                            _4580 = _4489;
                            _4581 = _4490;
                            _4582 = _4488;
                        }
                        _4583 = _4579;
                        _4584 = _4580;
                        _4585 = _4581;
                        _4586 = _4582;
                        break;
                    } while(false);
                    _4587 = _4583;
                    _4588 = _4584;
                    _4589 = _4585;
                    _4590 = _4586;
                }
                else
                {
                    bool _414;
                    if (!(_389 == 2))
                    {
                        _414 = _389 == 13;
                    }
                    else
                    {
                        _414 = true;
                    }
                    uint _4172;
                    uint _4173;
                    bool _4174;
                    bool _4175;
                    if (_414)
                    {
                        uint _4168;
                        uint _4169;
                        bool _4170;
                        bool _4171;
                        do
                        {
                            float _3765 = -_362;
                            float2 _3776 = (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            float _3783;
                            float _3786;
                            float2 _3788;
                            float2 _3790;
                            _3783 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            _3786 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _3776, 0.0f).x);
                            _3788 = _149[1];
                            _3790 = _149[2];
                            for (uint _3792 = 0u; _3792 < 2u; )
                            {
                                float2 _3796 = _3788 + _3790;
                                float2 _3797 = _3796 * 0.5f;
                                float _3804 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3797 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                float _3805 = _3804 * _3783;
                                bool _3806 = _3805 < 0.0f;
                                bool2 _3807 = _3806.xx;
                                _3783 = _3806 ? _3783 : _3804;
                                _3786 = _3806 ? _3804 : _3786;
                                _3788 = float2(_3807.x ? _3788.x : _3797.x, _3807.y ? _3788.y : _3797.y);
                                _3790 = float2(_3807.x ? _3797.x : _3790.x, _3807.y ? _3797.y : _3790.y);
                                _3792++;
                                continue;
                            }
                            float2 _3812 = lerp(_3790, _3788, clamp(_3786 / (_3786 - _3783), 0.0f, 1.0f).xx);
                            float _3813 = _3812.x;
                            float3 _3815 = float3(_3813, _362, _3812.y);
                            float _3830;
                            float _3833;
                            float2 _3835;
                            float2 _3837;
                            _3830 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            _3833 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _3776, 0.0f).x);
                            _3835 = _149[3];
                            _3837 = _149[2];
                            for (uint _3839 = 0u; _3839 < 2u; )
                            {
                                float2 _3843 = _3835 + _3837;
                                float2 _3844 = _3843 * 0.5f;
                                float _3851 = _3765 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3844 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                float _3852 = _3851 * _3830;
                                bool _3853 = _3852 < 0.0f;
                                bool2 _3854 = _3853.xx;
                                _3830 = _3853 ? _3830 : _3851;
                                _3833 = _3853 ? _3851 : _3833;
                                _3835 = float2(_3854.x ? _3835.x : _3844.x, _3854.y ? _3835.y : _3844.y);
                                _3837 = float2(_3854.x ? _3844.x : _3837.x, _3854.y ? _3844.y : _3837.y);
                                _3839++;
                                continue;
                            }
                            float2 _3868;
                            float _3869;
                            bool _3871;
                            float2 _3859 = lerp(_3837, _3835, clamp(_3833 / (_3833 - _3830), 0.0f, 1.0f).xx);
                            float _3861 = _3859.y;
                            float3 _3862 = float3(_3859.x, _362, _3861);
                            float3 _3863 = lerp(_3815, _3862, 0.5f.xxx);
                            float _3895;
                            do
                            {
                                _3868 = (_3863.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                _3869 = _3868.x;
                                _3871 = !(_3869 < 0.0f);
                                bool _3875;
                                if (_3871)
                                {
                                    _3875 = _3869 > 1.0f;
                                }
                                else
                                {
                                    _3875 = true;
                                }
                                bool _3881;
                                if (!_3875)
                                {
                                    _3881 = _3868.y < 0.0f;
                                }
                                else
                                {
                                    _3881 = true;
                                }
                                bool _3887;
                                if (!_3881)
                                {
                                    _3887 = _3868.y > 1.0f;
                                }
                                else
                                {
                                    _3887 = true;
                                }
                                if (_3887)
                                {
                                    _3895 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    _3895 = rocky_terrain.SampleLevel(sampler_default, _3868, 0.0f).x;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            if (_3895 < 0.00999999977648258209228515625f)
                            {
                                _4168 = _346;
                                _4169 = _349;
                                _4170 = _351;
                                _4171 = _353;
                                break;
                            }
                            bool _3919;
                            if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_3815.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                            {
                                _3919 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_3862.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                            }
                            else
                            {
                                _3919 = false;
                            }
                            if (_3919)
                            {
                                _4168 = _346;
                                _4169 = _349;
                                _4170 = _351;
                                _4171 = _353;
                                break;
                            }
                            float _3922 = distance(_3815, _3862);
                            if (_3922 < 0.20000000298023223876953125f)
                            {
                                _4168 = _346;
                                _4169 = _349;
                                _4170 = _351;
                                _4171 = _353;
                                break;
                            }
                            float4 _3927 = terrain_heightmap.GatherRed(sampler_default, _3868, int2(0, 0));
                            float _3932 = _3927.z;
                            float _3933 = _3927.y;
                            float _3935 = _3927.w;
                            float _3936 = _3927.x;
                            float2 _3945 = ((float2((_3932 + _3933) - (_3935 + _3936), (_3936 + _3933) - (_3935 + _3932)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                            float _3978;
                            do
                            {
                                bool _3958;
                                if (_3871)
                                {
                                    _3958 = _3869 > 1.0f;
                                }
                                else
                                {
                                    _3958 = true;
                                }
                                bool _3964;
                                if (!_3958)
                                {
                                    _3964 = _3868.y < 0.0f;
                                }
                                else
                                {
                                    _3964 = true;
                                }
                                bool _3970;
                                if (!_3964)
                                {
                                    _3970 = _3868.y > 1.0f;
                                }
                                else
                                {
                                    _3970 = true;
                                }
                                if (_3970)
                                {
                                    _3978 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    _3978 = rocky_terrain.SampleLevel(sampler_default, _3868, 0.0f).x;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            float _4004;
                            do
                            {
                                bool _3984;
                                if (_3871)
                                {
                                    _3984 = _3869 > 1.0f;
                                }
                                else
                                {
                                    _3984 = true;
                                }
                                bool _3990;
                                if (!_3984)
                                {
                                    _3990 = _3868.y < 0.0f;
                                }
                                else
                                {
                                    _3990 = true;
                                }
                                bool _3996;
                                if (!_3990)
                                {
                                    _3996 = _3868.y > 1.0f;
                                }
                                else
                                {
                                    _3996 = true;
                                }
                                if (_3996)
                                {
                                    _4004 = 0.0f;
                                    break;
                                }
                                else
                                {
                                    _4004 = rocky_terrain.SampleLevel(sampler_default, _3868, 0.0f).z;
                                    break;
                                }
                                break; // unreachable workaround
                            } while(false);
                            float3 _4008 = cross(normalize(float3(0.0f, _3945.y, 1.0f)), normalize(float3(1.0f, _3945.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_3978 + _4004, 0.0f, 1.0f));
                            float3 _4009 = _3863 - _4008;
                            float3 _4013 = normalize(_3862 - _3815);
                            uint _4018 = uint(((_3813 + _362) + _3861) * 100.0f);
                            float4x4 _4036 = transpose(float4x4(float4(_4013, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_4013, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                            float4x4 _4038 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _4009.x), float4(0.0f, 1.0f, 0.0f, _4009.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _4009.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_4036, float4x4(float4(_3922, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                            uint _4043;
                            __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _4043);
                            uint _4057 = uint(int(_4043));
                            float _4059[12] = { _4038[0].x, _4038[1].x, _4038[2].x, _4038[0].y, _4038[1].y, _4038[2].y, _4038[0].z, _4038[1].z, _4038[2].z, _4038[0].w, _4038[1].w, _4038[2].w };
                            Affine3Packed _4060 = { _4059 };
                            Stair _4061 = { _4060 };
                            [unroll]
                            for (int _22ident = 0; _22ident < 12; _22ident++)
                            {
                                __alias_stairs_buffer.Store<float>(_22ident * 4 + _4057 * 48 + 0, _4061.transform.data[_22ident]);
                            }
                            uint _4062 = _4018 ^ 23784u;
                            uint _4064 = _4062 + (_4062 << 10u);
                            uint _4066 = _4064 ^ (_4064 >> 6u);
                            uint _4068 = _4066 + (_4066 << 3u);
                            uint _4070 = _4068 ^ (_4068 >> 11u);
                            bool _4073 = _353 ? _353 : true;
                            uint _4074 = _353 ? _349 : 8388607u;
                            bool _4075 = _351 ? _351 : true;
                            uint _4076 = _351 ? _346 : 1065353216u;
                            uint _4164;
                            uint _4165;
                            bool _4166;
                            bool _4167;
                            if ((asfloat(((_4070 + (_4070 << 15u)) & _4074) | _4076) - 1.0f) > 0.85000002384185791015625f)
                            {
                                uint _4084 = _4018 ^ 90455u;
                                uint _4086 = _4084 + (_4084 << 10u);
                                uint _4088 = _4086 ^ (_4086 >> 6u);
                                uint _4090 = _4088 + (_4088 << 3u);
                                uint _4092 = _4090 ^ (_4090 >> 11u);
                                bool _4095 = _4073 ? _4073 : true;
                                uint _4096 = _4073 ? _4074 : 8388607u;
                                bool _4097 = _4075 ? _4075 : true;
                                uint _4098 = _4075 ? _4076 : 1065353216u;
                                float _4103 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_4092 + (_4092 << 15u)) & _4096) | _4098) - 1.0f);
                                uint _4104 = _4018 ^ 3845u;
                                uint _4106 = _4104 + (_4104 << 10u);
                                uint _4108 = _4106 ^ (_4106 >> 6u);
                                uint _4110 = _4108 + (_4108 << 3u);
                                uint _4112 = _4110 ^ (_4110 >> 11u);
                                uint _4116 = _4095 ? _4096 : 8388607u;
                                uint _4118 = _4097 ? _4098 : 1065353216u;
                                float3 _4125 = lerp(_3815, _3862, (asfloat(((_4112 + (_4112 << 15u)) & _4116) | _4118) - 1.0f).xxx) - _4008;
                                uint _4129;
                                __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _4129);
                                uint _4134 = _Globals_pebble_buffer_offset + uint(int(_4129));
                                float4x4 _4147 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _4125.x), float4(0.0f, 1.0f, 0.0f, _4125.y), float4(0.0f, 0.0f, 1.0f, _4125.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_4036, float4x4(float4(_4103, 0.0f, 0.0f, 0.0f), float4(0.0f, _4103, 0.0f, 0.0f), float4(0.0f, 0.0f, _4103, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                float _4161[12] = { _4147[0].x, _4147[1].x, _4147[2].x, _4147[0].y, _4147[1].y, _4147[2].y, _4147[0].z, _4147[1].z, _4147[2].z, _4147[0].w, _4147[1].w, _4147[2].w };
                                Affine3Packed _4162 = { _4161 };
                                Stair _4163 = { _4162 };
                                [unroll]
                                for (int _23ident = 0; _23ident < 12; _23ident++)
                                {
                                    __alias_stairs_buffer.Store<float>(_23ident * 4 + _4134 * 48 + 0, _4163.transform.data[_23ident]);
                                }
                                _4164 = _4118;
                                _4165 = _4116;
                                _4166 = _4097 ? _4097 : true;
                                _4167 = _4095 ? _4095 : true;
                            }
                            else
                            {
                                _4164 = _4076;
                                _4165 = _4074;
                                _4166 = _4075;
                                _4167 = _4073;
                            }
                            _4168 = _4164;
                            _4169 = _4165;
                            _4170 = _4166;
                            _4171 = _4167;
                            break;
                        } while(false);
                        _4172 = _4168;
                        _4173 = _4169;
                        _4174 = _4170;
                        _4175 = _4171;
                    }
                    else
                    {
                        bool _423;
                        if (!(_389 == 3))
                        {
                            _423 = _389 == 12;
                        }
                        else
                        {
                            _423 = true;
                        }
                        uint _3757;
                        uint _3758;
                        bool _3759;
                        bool _3760;
                        if (_423)
                        {
                            uint _3753;
                            uint _3754;
                            bool _3755;
                            bool _3756;
                            do
                            {
                                float _3347 = -_362;
                                float _3365;
                                float _3368;
                                float2 _3370;
                                float2 _3372;
                                _3365 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                _3368 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                _3370 = _149[0];
                                _3372 = _149[3];
                                for (uint _3374 = 0u; _3374 < 2u; )
                                {
                                    float2 _3378 = _3370 + _3372;
                                    float2 _3379 = _3378 * 0.5f;
                                    float _3386 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3379 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    float _3387 = _3386 * _3365;
                                    bool _3388 = _3387 < 0.0f;
                                    bool2 _3389 = _3388.xx;
                                    _3365 = _3388 ? _3365 : _3386;
                                    _3368 = _3388 ? _3386 : _3368;
                                    _3370 = float2(_3389.x ? _3370.x : _3379.x, _3389.y ? _3370.y : _3379.y);
                                    _3372 = float2(_3389.x ? _3379.x : _3372.x, _3389.y ? _3379.y : _3372.y);
                                    _3374++;
                                    continue;
                                }
                                float2 _3394 = lerp(_3372, _3370, clamp(_3368 / (_3368 - _3365), 0.0f, 1.0f).xx);
                                float _3395 = _3394.x;
                                float3 _3397 = float3(_3395, _362, _3394.y);
                                float _3415;
                                float _3418;
                                float2 _3420;
                                float2 _3422;
                                _3415 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                _3418 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                _3420 = _149[1];
                                _3422 = _149[2];
                                for (uint _3424 = 0u; _3424 < 2u; )
                                {
                                    float2 _3428 = _3420 + _3422;
                                    float2 _3429 = _3428 * 0.5f;
                                    float _3436 = _3347 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3429 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    float _3437 = _3436 * _3415;
                                    bool _3438 = _3437 < 0.0f;
                                    bool2 _3439 = _3438.xx;
                                    _3415 = _3438 ? _3415 : _3436;
                                    _3418 = _3438 ? _3436 : _3418;
                                    _3420 = float2(_3439.x ? _3420.x : _3429.x, _3439.y ? _3420.y : _3429.y);
                                    _3422 = float2(_3439.x ? _3429.x : _3422.x, _3439.y ? _3429.y : _3422.y);
                                    _3424++;
                                    continue;
                                }
                                float2 _3453;
                                float _3454;
                                bool _3456;
                                float2 _3444 = lerp(_3422, _3420, clamp(_3418 / (_3418 - _3415), 0.0f, 1.0f).xx);
                                float _3446 = _3444.y;
                                float3 _3447 = float3(_3444.x, _362, _3446);
                                float3 _3448 = lerp(_3397, _3447, 0.5f.xxx);
                                float _3480;
                                do
                                {
                                    _3453 = (_3448.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    _3454 = _3453.x;
                                    _3456 = !(_3454 < 0.0f);
                                    bool _3460;
                                    if (_3456)
                                    {
                                        _3460 = _3454 > 1.0f;
                                    }
                                    else
                                    {
                                        _3460 = true;
                                    }
                                    bool _3466;
                                    if (!_3460)
                                    {
                                        _3466 = _3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        _3466 = true;
                                    }
                                    bool _3472;
                                    if (!_3466)
                                    {
                                        _3472 = _3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        _3472 = true;
                                    }
                                    if (_3472)
                                    {
                                        _3480 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        _3480 = rocky_terrain.SampleLevel(sampler_default, _3453, 0.0f).x;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                if (_3480 < 0.00999999977648258209228515625f)
                                {
                                    _3753 = _346;
                                    _3754 = _349;
                                    _3755 = _351;
                                    _3756 = _353;
                                    break;
                                }
                                bool _3504;
                                if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_3397.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                {
                                    _3504 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_3447.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                }
                                else
                                {
                                    _3504 = false;
                                }
                                if (_3504)
                                {
                                    _3753 = _346;
                                    _3754 = _349;
                                    _3755 = _351;
                                    _3756 = _353;
                                    break;
                                }
                                float _3507 = distance(_3397, _3447);
                                if (_3507 < 0.20000000298023223876953125f)
                                {
                                    _3753 = _346;
                                    _3754 = _349;
                                    _3755 = _351;
                                    _3756 = _353;
                                    break;
                                }
                                float4 _3512 = terrain_heightmap.GatherRed(sampler_default, _3453, int2(0, 0));
                                float _3517 = _3512.z;
                                float _3518 = _3512.y;
                                float _3520 = _3512.w;
                                float _3521 = _3512.x;
                                float2 _3530 = ((float2((_3517 + _3518) - (_3520 + _3521), (_3521 + _3518) - (_3520 + _3517)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                float _3563;
                                do
                                {
                                    bool _3543;
                                    if (_3456)
                                    {
                                        _3543 = _3454 > 1.0f;
                                    }
                                    else
                                    {
                                        _3543 = true;
                                    }
                                    bool _3549;
                                    if (!_3543)
                                    {
                                        _3549 = _3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        _3549 = true;
                                    }
                                    bool _3555;
                                    if (!_3549)
                                    {
                                        _3555 = _3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        _3555 = true;
                                    }
                                    if (_3555)
                                    {
                                        _3563 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        _3563 = rocky_terrain.SampleLevel(sampler_default, _3453, 0.0f).x;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                float _3589;
                                do
                                {
                                    bool _3569;
                                    if (_3456)
                                    {
                                        _3569 = _3454 > 1.0f;
                                    }
                                    else
                                    {
                                        _3569 = true;
                                    }
                                    bool _3575;
                                    if (!_3569)
                                    {
                                        _3575 = _3453.y < 0.0f;
                                    }
                                    else
                                    {
                                        _3575 = true;
                                    }
                                    bool _3581;
                                    if (!_3575)
                                    {
                                        _3581 = _3453.y > 1.0f;
                                    }
                                    else
                                    {
                                        _3581 = true;
                                    }
                                    if (_3581)
                                    {
                                        _3589 = 0.0f;
                                        break;
                                    }
                                    else
                                    {
                                        _3589 = rocky_terrain.SampleLevel(sampler_default, _3453, 0.0f).z;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                float3 _3593 = cross(normalize(float3(0.0f, _3530.y, 1.0f)), normalize(float3(1.0f, _3530.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_3563 + _3589, 0.0f, 1.0f));
                                float3 _3594 = _3448 - _3593;
                                float3 _3598 = normalize(_3447 - _3397);
                                uint _3603 = uint(((_3395 + _362) + _3446) * 100.0f);
                                float4x4 _3621 = transpose(float4x4(float4(_3598, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_3598, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                float4x4 _3623 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3594.x), float4(0.0f, 1.0f, 0.0f, _3594.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _3594.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_3621, float4x4(float4(_3507, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                uint _3628;
                                __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _3628);
                                uint _3642 = uint(int(_3628));
                                float _3644[12] = { _3623[0].x, _3623[1].x, _3623[2].x, _3623[0].y, _3623[1].y, _3623[2].y, _3623[0].z, _3623[1].z, _3623[2].z, _3623[0].w, _3623[1].w, _3623[2].w };
                                Affine3Packed _3645 = { _3644 };
                                Stair _3646 = { _3645 };
                                [unroll]
                                for (int _24ident = 0; _24ident < 12; _24ident++)
                                {
                                    __alias_stairs_buffer.Store<float>(_24ident * 4 + _3642 * 48 + 0, _3646.transform.data[_24ident]);
                                }
                                uint _3647 = _3603 ^ 23784u;
                                uint _3649 = _3647 + (_3647 << 10u);
                                uint _3651 = _3649 ^ (_3649 >> 6u);
                                uint _3653 = _3651 + (_3651 << 3u);
                                uint _3655 = _3653 ^ (_3653 >> 11u);
                                bool _3658 = _353 ? _353 : true;
                                uint _3659 = _353 ? _349 : 8388607u;
                                bool _3660 = _351 ? _351 : true;
                                uint _3661 = _351 ? _346 : 1065353216u;
                                uint _3749;
                                uint _3750;
                                bool _3751;
                                bool _3752;
                                if ((asfloat(((_3655 + (_3655 << 15u)) & _3659) | _3661) - 1.0f) > 0.85000002384185791015625f)
                                {
                                    uint _3669 = _3603 ^ 90455u;
                                    uint _3671 = _3669 + (_3669 << 10u);
                                    uint _3673 = _3671 ^ (_3671 >> 6u);
                                    uint _3675 = _3673 + (_3673 << 3u);
                                    uint _3677 = _3675 ^ (_3675 >> 11u);
                                    bool _3680 = _3658 ? _3658 : true;
                                    uint _3681 = _3658 ? _3659 : 8388607u;
                                    bool _3682 = _3660 ? _3660 : true;
                                    uint _3683 = _3660 ? _3661 : 1065353216u;
                                    float _3688 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_3677 + (_3677 << 15u)) & _3681) | _3683) - 1.0f);
                                    uint _3689 = _3603 ^ 3845u;
                                    uint _3691 = _3689 + (_3689 << 10u);
                                    uint _3693 = _3691 ^ (_3691 >> 6u);
                                    uint _3695 = _3693 + (_3693 << 3u);
                                    uint _3697 = _3695 ^ (_3695 >> 11u);
                                    uint _3701 = _3680 ? _3681 : 8388607u;
                                    uint _3703 = _3682 ? _3683 : 1065353216u;
                                    float3 _3710 = lerp(_3397, _3447, (asfloat(((_3697 + (_3697 << 15u)) & _3701) | _3703) - 1.0f).xxx) - _3593;
                                    uint _3714;
                                    __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _3714);
                                    uint _3719 = _Globals_pebble_buffer_offset + uint(int(_3714));
                                    float4x4 _3732 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3710.x), float4(0.0f, 1.0f, 0.0f, _3710.y), float4(0.0f, 0.0f, 1.0f, _3710.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_3621, float4x4(float4(_3688, 0.0f, 0.0f, 0.0f), float4(0.0f, _3688, 0.0f, 0.0f), float4(0.0f, 0.0f, _3688, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    float _3746[12] = { _3732[0].x, _3732[1].x, _3732[2].x, _3732[0].y, _3732[1].y, _3732[2].y, _3732[0].z, _3732[1].z, _3732[2].z, _3732[0].w, _3732[1].w, _3732[2].w };
                                    Affine3Packed _3747 = { _3746 };
                                    Stair _3748 = { _3747 };
                                    [unroll]
                                    for (int _25ident = 0; _25ident < 12; _25ident++)
                                    {
                                        __alias_stairs_buffer.Store<float>(_25ident * 4 + _3719 * 48 + 0, _3748.transform.data[_25ident]);
                                    }
                                    _3749 = _3703;
                                    _3750 = _3701;
                                    _3751 = _3682 ? _3682 : true;
                                    _3752 = _3680 ? _3680 : true;
                                }
                                else
                                {
                                    _3749 = _3661;
                                    _3750 = _3659;
                                    _3751 = _3660;
                                    _3752 = _3658;
                                }
                                _3753 = _3749;
                                _3754 = _3750;
                                _3755 = _3751;
                                _3756 = _3752;
                                break;
                            } while(false);
                            _3757 = _3753;
                            _3758 = _3754;
                            _3759 = _3755;
                            _3760 = _3756;
                        }
                        else
                        {
                            bool _432;
                            if (!(_389 == 4))
                            {
                                _432 = _389 == 11;
                            }
                            else
                            {
                                _432 = true;
                            }
                            uint _3339;
                            uint _3340;
                            bool _3341;
                            bool _3342;
                            if (_432)
                            {
                                uint _3335;
                                uint _3336;
                                bool _3337;
                                bool _3338;
                                do
                                {
                                    float _2932 = -_362;
                                    float2 _2943 = (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                    float _2950;
                                    float _2953;
                                    float2 _2955;
                                    float2 _2957;
                                    _2950 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    _2953 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2943, 0.0f).x);
                                    _2955 = _149[0];
                                    _2957 = _149[1];
                                    for (uint _2959 = 0u; _2959 < 2u; )
                                    {
                                        float2 _2963 = _2955 + _2957;
                                        float2 _2964 = _2963 * 0.5f;
                                        float _2971 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2964 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                        float _2972 = _2971 * _2950;
                                        bool _2973 = _2972 < 0.0f;
                                        bool2 _2974 = _2973.xx;
                                        _2950 = _2973 ? _2950 : _2971;
                                        _2953 = _2973 ? _2971 : _2953;
                                        _2955 = float2(_2974.x ? _2955.x : _2964.x, _2974.y ? _2955.y : _2964.y);
                                        _2957 = float2(_2974.x ? _2964.x : _2957.x, _2974.y ? _2964.y : _2957.y);
                                        _2959++;
                                        continue;
                                    }
                                    float2 _2979 = lerp(_2957, _2955, clamp(_2953 / (_2953 - _2950), 0.0f, 1.0f).xx);
                                    float _2980 = _2979.x;
                                    float3 _2982 = float3(_2980, _362, _2979.y);
                                    float _2997;
                                    float _3000;
                                    float2 _3002;
                                    float2 _3004;
                                    _2997 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2943, 0.0f).x);
                                    _3000 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                    _3002 = _149[1];
                                    _3004 = _149[2];
                                    for (uint _3006 = 0u; _3006 < 2u; )
                                    {
                                        float2 _3010 = _3002 + _3004;
                                        float2 _3011 = _3010 * 0.5f;
                                        float _3018 = _2932 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_3011 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                        float _3019 = _3018 * _2997;
                                        bool _3020 = _3019 < 0.0f;
                                        bool2 _3021 = _3020.xx;
                                        _2997 = _3020 ? _2997 : _3018;
                                        _3000 = _3020 ? _3018 : _3000;
                                        _3002 = float2(_3021.x ? _3002.x : _3011.x, _3021.y ? _3002.y : _3011.y);
                                        _3004 = float2(_3021.x ? _3011.x : _3004.x, _3021.y ? _3011.y : _3004.y);
                                        _3006++;
                                        continue;
                                    }
                                    float2 _3035;
                                    float _3036;
                                    bool _3038;
                                    float2 _3026 = lerp(_3004, _3002, clamp(_3000 / (_3000 - _2997), 0.0f, 1.0f).xx);
                                    float _3028 = _3026.y;
                                    float3 _3029 = float3(_3026.x, _362, _3028);
                                    float3 _3030 = lerp(_2982, _3029, 0.5f.xxx);
                                    float _3062;
                                    do
                                    {
                                        _3035 = (_3030.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        _3036 = _3035.x;
                                        _3038 = !(_3036 < 0.0f);
                                        bool _3042;
                                        if (_3038)
                                        {
                                            _3042 = _3036 > 1.0f;
                                        }
                                        else
                                        {
                                            _3042 = true;
                                        }
                                        bool _3048;
                                        if (!_3042)
                                        {
                                            _3048 = _3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            _3048 = true;
                                        }
                                        bool _3054;
                                        if (!_3048)
                                        {
                                            _3054 = _3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            _3054 = true;
                                        }
                                        if (_3054)
                                        {
                                            _3062 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            _3062 = rocky_terrain.SampleLevel(sampler_default, _3035, 0.0f).x;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    if (_3062 < 0.00999999977648258209228515625f)
                                    {
                                        _3335 = _346;
                                        _3336 = _349;
                                        _3337 = _351;
                                        _3338 = _353;
                                        break;
                                    }
                                    bool _3086;
                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_2982.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                    {
                                        _3086 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_3029.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                    }
                                    else
                                    {
                                        _3086 = false;
                                    }
                                    if (_3086)
                                    {
                                        _3335 = _346;
                                        _3336 = _349;
                                        _3337 = _351;
                                        _3338 = _353;
                                        break;
                                    }
                                    float _3089 = distance(_2982, _3029);
                                    if (_3089 < 0.20000000298023223876953125f)
                                    {
                                        _3335 = _346;
                                        _3336 = _349;
                                        _3337 = _351;
                                        _3338 = _353;
                                        break;
                                    }
                                    float4 _3094 = terrain_heightmap.GatherRed(sampler_default, _3035, int2(0, 0));
                                    float _3099 = _3094.z;
                                    float _3100 = _3094.y;
                                    float _3102 = _3094.w;
                                    float _3103 = _3094.x;
                                    float2 _3112 = ((float2((_3099 + _3100) - (_3102 + _3103), (_3103 + _3100) - (_3102 + _3099)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                    float _3145;
                                    do
                                    {
                                        bool _3125;
                                        if (_3038)
                                        {
                                            _3125 = _3036 > 1.0f;
                                        }
                                        else
                                        {
                                            _3125 = true;
                                        }
                                        bool _3131;
                                        if (!_3125)
                                        {
                                            _3131 = _3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            _3131 = true;
                                        }
                                        bool _3137;
                                        if (!_3131)
                                        {
                                            _3137 = _3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            _3137 = true;
                                        }
                                        if (_3137)
                                        {
                                            _3145 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            _3145 = rocky_terrain.SampleLevel(sampler_default, _3035, 0.0f).x;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    float _3171;
                                    do
                                    {
                                        bool _3151;
                                        if (_3038)
                                        {
                                            _3151 = _3036 > 1.0f;
                                        }
                                        else
                                        {
                                            _3151 = true;
                                        }
                                        bool _3157;
                                        if (!_3151)
                                        {
                                            _3157 = _3035.y < 0.0f;
                                        }
                                        else
                                        {
                                            _3157 = true;
                                        }
                                        bool _3163;
                                        if (!_3157)
                                        {
                                            _3163 = _3035.y > 1.0f;
                                        }
                                        else
                                        {
                                            _3163 = true;
                                        }
                                        if (_3163)
                                        {
                                            _3171 = 0.0f;
                                            break;
                                        }
                                        else
                                        {
                                            _3171 = rocky_terrain.SampleLevel(sampler_default, _3035, 0.0f).z;
                                            break;
                                        }
                                        break; // unreachable workaround
                                    } while(false);
                                    float3 _3175 = cross(normalize(float3(0.0f, _3112.y, 1.0f)), normalize(float3(1.0f, _3112.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_3145 + _3171, 0.0f, 1.0f));
                                    float3 _3176 = _3030 - _3175;
                                    float3 _3180 = normalize(_3029 - _2982);
                                    uint _3185 = uint(((_2980 + _362) + _3028) * 100.0f);
                                    float4x4 _3203 = transpose(float4x4(float4(_3180, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_3180, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                    float4x4 _3205 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3176.x), float4(0.0f, 1.0f, 0.0f, _3176.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _3176.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_3203, float4x4(float4(_3089, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                    uint _3210;
                                    __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _3210);
                                    uint _3224 = uint(int(_3210));
                                    float _3226[12] = { _3205[0].x, _3205[1].x, _3205[2].x, _3205[0].y, _3205[1].y, _3205[2].y, _3205[0].z, _3205[1].z, _3205[2].z, _3205[0].w, _3205[1].w, _3205[2].w };
                                    Affine3Packed _3227 = { _3226 };
                                    Stair _3228 = { _3227 };
                                    [unroll]
                                    for (int _26ident = 0; _26ident < 12; _26ident++)
                                    {
                                        __alias_stairs_buffer.Store<float>(_26ident * 4 + _3224 * 48 + 0, _3228.transform.data[_26ident]);
                                    }
                                    uint _3229 = _3185 ^ 23784u;
                                    uint _3231 = _3229 + (_3229 << 10u);
                                    uint _3233 = _3231 ^ (_3231 >> 6u);
                                    uint _3235 = _3233 + (_3233 << 3u);
                                    uint _3237 = _3235 ^ (_3235 >> 11u);
                                    bool _3240 = _353 ? _353 : true;
                                    uint _3241 = _353 ? _349 : 8388607u;
                                    bool _3242 = _351 ? _351 : true;
                                    uint _3243 = _351 ? _346 : 1065353216u;
                                    uint _3331;
                                    uint _3332;
                                    bool _3333;
                                    bool _3334;
                                    if ((asfloat(((_3237 + (_3237 << 15u)) & _3241) | _3243) - 1.0f) > 0.85000002384185791015625f)
                                    {
                                        uint _3251 = _3185 ^ 90455u;
                                        uint _3253 = _3251 + (_3251 << 10u);
                                        uint _3255 = _3253 ^ (_3253 >> 6u);
                                        uint _3257 = _3255 + (_3255 << 3u);
                                        uint _3259 = _3257 ^ (_3257 >> 11u);
                                        bool _3262 = _3240 ? _3240 : true;
                                        uint _3263 = _3240 ? _3241 : 8388607u;
                                        bool _3264 = _3242 ? _3242 : true;
                                        uint _3265 = _3242 ? _3243 : 1065353216u;
                                        float _3270 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_3259 + (_3259 << 15u)) & _3263) | _3265) - 1.0f);
                                        uint _3271 = _3185 ^ 3845u;
                                        uint _3273 = _3271 + (_3271 << 10u);
                                        uint _3275 = _3273 ^ (_3273 >> 6u);
                                        uint _3277 = _3275 + (_3275 << 3u);
                                        uint _3279 = _3277 ^ (_3277 >> 11u);
                                        uint _3283 = _3262 ? _3263 : 8388607u;
                                        uint _3285 = _3264 ? _3265 : 1065353216u;
                                        float3 _3292 = lerp(_2982, _3029, (asfloat(((_3279 + (_3279 << 15u)) & _3283) | _3285) - 1.0f).xxx) - _3175;
                                        uint _3296;
                                        __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _3296);
                                        uint _3301 = _Globals_pebble_buffer_offset + uint(int(_3296));
                                        float4x4 _3314 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _3292.x), float4(0.0f, 1.0f, 0.0f, _3292.y), float4(0.0f, 0.0f, 1.0f, _3292.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_3203, float4x4(float4(_3270, 0.0f, 0.0f, 0.0f), float4(0.0f, _3270, 0.0f, 0.0f), float4(0.0f, 0.0f, _3270, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        float _3328[12] = { _3314[0].x, _3314[1].x, _3314[2].x, _3314[0].y, _3314[1].y, _3314[2].y, _3314[0].z, _3314[1].z, _3314[2].z, _3314[0].w, _3314[1].w, _3314[2].w };
                                        Affine3Packed _3329 = { _3328 };
                                        Stair _3330 = { _3329 };
                                        [unroll]
                                        for (int _27ident = 0; _27ident < 12; _27ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_27ident * 4 + _3301 * 48 + 0, _3330.transform.data[_27ident]);
                                        }
                                        _3331 = _3285;
                                        _3332 = _3283;
                                        _3333 = _3264 ? _3264 : true;
                                        _3334 = _3262 ? _3262 : true;
                                    }
                                    else
                                    {
                                        _3331 = _3243;
                                        _3332 = _3241;
                                        _3333 = _3242;
                                        _3334 = _3240;
                                    }
                                    _3335 = _3331;
                                    _3336 = _3332;
                                    _3337 = _3333;
                                    _3338 = _3334;
                                    break;
                                } while(false);
                                _3339 = _3335;
                                _3340 = _3336;
                                _3341 = _3337;
                                _3342 = _3338;
                            }
                            else
                            {
                                uint _2924;
                                uint _2925;
                                bool _2926;
                                bool _2927;
                                if (_389 == 5)
                                {
                                    float2 _2114;
                                    float _2115;
                                    float2 _2126;
                                    float2 _2166;
                                    float2 _2173;
                                    uint _2518;
                                    uint _2519;
                                    bool _2520;
                                    bool _2521;
                                    do
                                    {
                                        _2114 = _149[1];
                                        _2115 = -_362;
                                        float2 _2117 = (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        _2126 = (_2114 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float _2133;
                                        float _2136;
                                        float2 _2138;
                                        float2 _2140;
                                        _2133 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2117, 0.0f).x);
                                        _2136 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2126, 0.0f).x);
                                        _2138 = _149[0];
                                        _2140 = _2114;
                                        for (uint _2142 = 0u; _2142 < 2u; )
                                        {
                                            float2 _2146 = _2138 + _2140;
                                            float2 _2147 = _2146 * 0.5f;
                                            float _2154 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2147 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float _2155 = _2154 * _2133;
                                            bool _2156 = _2155 < 0.0f;
                                            bool2 _2157 = _2156.xx;
                                            _2133 = _2156 ? _2133 : _2154;
                                            _2136 = _2156 ? _2154 : _2136;
                                            _2138 = float2(_2157.x ? _2138.x : _2147.x, _2157.y ? _2138.y : _2147.y);
                                            _2140 = float2(_2157.x ? _2147.x : _2140.x, _2157.y ? _2147.y : _2140.y);
                                            _2142++;
                                            continue;
                                        }
                                        float2 _2162 = lerp(_2140, _2138, clamp(_2136 / (_2136 - _2133), 0.0f, 1.0f).xx);
                                        float _2163 = _2162.x;
                                        float3 _2165 = float3(_2163, _362, _2162.y);
                                        _2166 = _149[3];
                                        _2173 = (_2166 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float _2180;
                                        float _2183;
                                        float2 _2185;
                                        float2 _2187;
                                        _2180 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2117, 0.0f).x);
                                        _2183 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2173, 0.0f).x);
                                        _2185 = _149[0];
                                        _2187 = _2166;
                                        for (uint _2189 = 0u; _2189 < 2u; )
                                        {
                                            float2 _2193 = _2185 + _2187;
                                            float2 _2194 = _2193 * 0.5f;
                                            float _2201 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2194 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float _2202 = _2201 * _2180;
                                            bool _2203 = _2202 < 0.0f;
                                            bool2 _2204 = _2203.xx;
                                            _2180 = _2203 ? _2180 : _2201;
                                            _2183 = _2203 ? _2201 : _2183;
                                            _2185 = float2(_2204.x ? _2185.x : _2194.x, _2204.y ? _2185.y : _2194.y);
                                            _2187 = float2(_2204.x ? _2194.x : _2187.x, _2204.y ? _2194.y : _2187.y);
                                            _2189++;
                                            continue;
                                        }
                                        float2 _2218;
                                        float _2219;
                                        bool _2221;
                                        float2 _2209 = lerp(_2187, _2185, clamp(_2183 / (_2183 - _2180), 0.0f, 1.0f).xx);
                                        float _2211 = _2209.y;
                                        float3 _2212 = float3(_2209.x, _362, _2211);
                                        float3 _2213 = lerp(_2165, _2212, 0.5f.xxx);
                                        float _2245;
                                        do
                                        {
                                            _2218 = (_2213.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            _2219 = _2218.x;
                                            _2221 = !(_2219 < 0.0f);
                                            bool _2225;
                                            if (_2221)
                                            {
                                                _2225 = _2219 > 1.0f;
                                            }
                                            else
                                            {
                                                _2225 = true;
                                            }
                                            bool _2231;
                                            if (!_2225)
                                            {
                                                _2231 = _2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2231 = true;
                                            }
                                            bool _2237;
                                            if (!_2231)
                                            {
                                                _2237 = _2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2237 = true;
                                            }
                                            if (_2237)
                                            {
                                                _2245 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2245 = rocky_terrain.SampleLevel(sampler_default, _2218, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        if (_2245 < 0.00999999977648258209228515625f)
                                        {
                                            _2518 = _346;
                                            _2519 = _349;
                                            _2520 = _351;
                                            _2521 = _353;
                                            break;
                                        }
                                        bool _2269;
                                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_2165.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                        {
                                            _2269 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_2212.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                        }
                                        else
                                        {
                                            _2269 = false;
                                        }
                                        if (_2269)
                                        {
                                            _2518 = _346;
                                            _2519 = _349;
                                            _2520 = _351;
                                            _2521 = _353;
                                            break;
                                        }
                                        float _2272 = distance(_2165, _2212);
                                        if (_2272 < 0.20000000298023223876953125f)
                                        {
                                            _2518 = _346;
                                            _2519 = _349;
                                            _2520 = _351;
                                            _2521 = _353;
                                            break;
                                        }
                                        float4 _2277 = terrain_heightmap.GatherRed(sampler_default, _2218, int2(0, 0));
                                        float _2282 = _2277.z;
                                        float _2283 = _2277.y;
                                        float _2285 = _2277.w;
                                        float _2286 = _2277.x;
                                        float2 _2295 = ((float2((_2282 + _2283) - (_2285 + _2286), (_2286 + _2283) - (_2285 + _2282)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                        float _2328;
                                        do
                                        {
                                            bool _2308;
                                            if (_2221)
                                            {
                                                _2308 = _2219 > 1.0f;
                                            }
                                            else
                                            {
                                                _2308 = true;
                                            }
                                            bool _2314;
                                            if (!_2308)
                                            {
                                                _2314 = _2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2314 = true;
                                            }
                                            bool _2320;
                                            if (!_2314)
                                            {
                                                _2320 = _2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2320 = true;
                                            }
                                            if (_2320)
                                            {
                                                _2328 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2328 = rocky_terrain.SampleLevel(sampler_default, _2218, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float _2354;
                                        do
                                        {
                                            bool _2334;
                                            if (_2221)
                                            {
                                                _2334 = _2219 > 1.0f;
                                            }
                                            else
                                            {
                                                _2334 = true;
                                            }
                                            bool _2340;
                                            if (!_2334)
                                            {
                                                _2340 = _2218.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2340 = true;
                                            }
                                            bool _2346;
                                            if (!_2340)
                                            {
                                                _2346 = _2218.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2346 = true;
                                            }
                                            if (_2346)
                                            {
                                                _2354 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2354 = rocky_terrain.SampleLevel(sampler_default, _2218, 0.0f).z;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float3 _2358 = cross(normalize(float3(0.0f, _2295.y, 1.0f)), normalize(float3(1.0f, _2295.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_2328 + _2354, 0.0f, 1.0f));
                                        float3 _2359 = _2213 - _2358;
                                        float3 _2363 = normalize(_2212 - _2165);
                                        uint _2368 = uint(((_2163 + _362) + _2211) * 100.0f);
                                        float4x4 _2386 = transpose(float4x4(float4(_2363, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_2363, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                        float4x4 _2388 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2359.x), float4(0.0f, 1.0f, 0.0f, _2359.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _2359.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_2386, float4x4(float4(_2272, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint _2393;
                                        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _2393);
                                        uint _2407 = uint(int(_2393));
                                        float _2409[12] = { _2388[0].x, _2388[1].x, _2388[2].x, _2388[0].y, _2388[1].y, _2388[2].y, _2388[0].z, _2388[1].z, _2388[2].z, _2388[0].w, _2388[1].w, _2388[2].w };
                                        Affine3Packed _2410 = { _2409 };
                                        Stair _2411 = { _2410 };
                                        [unroll]
                                        for (int _28ident = 0; _28ident < 12; _28ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_28ident * 4 + _2407 * 48 + 0, _2411.transform.data[_28ident]);
                                        }
                                        uint _2412 = _2368 ^ 23784u;
                                        uint _2414 = _2412 + (_2412 << 10u);
                                        uint _2416 = _2414 ^ (_2414 >> 6u);
                                        uint _2418 = _2416 + (_2416 << 3u);
                                        uint _2420 = _2418 ^ (_2418 >> 11u);
                                        bool _2423 = _353 ? _353 : true;
                                        uint _2424 = _353 ? _349 : 8388607u;
                                        bool _2425 = _351 ? _351 : true;
                                        uint _2426 = _351 ? _346 : 1065353216u;
                                        uint _2514;
                                        uint _2515;
                                        bool _2516;
                                        bool _2517;
                                        if ((asfloat(((_2420 + (_2420 << 15u)) & _2424) | _2426) - 1.0f) > 0.85000002384185791015625f)
                                        {
                                            uint _2434 = _2368 ^ 90455u;
                                            uint _2436 = _2434 + (_2434 << 10u);
                                            uint _2438 = _2436 ^ (_2436 >> 6u);
                                            uint _2440 = _2438 + (_2438 << 3u);
                                            uint _2442 = _2440 ^ (_2440 >> 11u);
                                            bool _2445 = _2423 ? _2423 : true;
                                            uint _2446 = _2423 ? _2424 : 8388607u;
                                            bool _2447 = _2425 ? _2425 : true;
                                            uint _2448 = _2425 ? _2426 : 1065353216u;
                                            float _2453 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_2442 + (_2442 << 15u)) & _2446) | _2448) - 1.0f);
                                            uint _2454 = _2368 ^ 3845u;
                                            uint _2456 = _2454 + (_2454 << 10u);
                                            uint _2458 = _2456 ^ (_2456 >> 6u);
                                            uint _2460 = _2458 + (_2458 << 3u);
                                            uint _2462 = _2460 ^ (_2460 >> 11u);
                                            uint _2466 = _2445 ? _2446 : 8388607u;
                                            uint _2468 = _2447 ? _2448 : 1065353216u;
                                            float3 _2475 = lerp(_2165, _2212, (asfloat(((_2462 + (_2462 << 15u)) & _2466) | _2468) - 1.0f).xxx) - _2358;
                                            uint _2479;
                                            __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _2479);
                                            uint _2484 = _Globals_pebble_buffer_offset + uint(int(_2479));
                                            float4x4 _2497 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2475.x), float4(0.0f, 1.0f, 0.0f, _2475.y), float4(0.0f, 0.0f, 1.0f, _2475.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_2386, float4x4(float4(_2453, 0.0f, 0.0f, 0.0f), float4(0.0f, _2453, 0.0f, 0.0f), float4(0.0f, 0.0f, _2453, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            float _2511[12] = { _2497[0].x, _2497[1].x, _2497[2].x, _2497[0].y, _2497[1].y, _2497[2].y, _2497[0].z, _2497[1].z, _2497[2].z, _2497[0].w, _2497[1].w, _2497[2].w };
                                            Affine3Packed _2512 = { _2511 };
                                            Stair _2513 = { _2512 };
                                            [unroll]
                                            for (int _29ident = 0; _29ident < 12; _29ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_29ident * 4 + _2484 * 48 + 0, _2513.transform.data[_29ident]);
                                            }
                                            _2514 = _2468;
                                            _2515 = _2466;
                                            _2516 = _2447 ? _2447 : true;
                                            _2517 = _2445 ? _2445 : true;
                                        }
                                        else
                                        {
                                            _2514 = _2426;
                                            _2515 = _2424;
                                            _2516 = _2425;
                                            _2517 = _2423;
                                        }
                                        _2518 = _2514;
                                        _2519 = _2515;
                                        _2520 = _2516;
                                        _2521 = _2517;
                                        break;
                                    } while(false);
                                    uint _2920;
                                    uint _2921;
                                    bool _2922;
                                    bool _2923;
                                    do
                                    {
                                        float2 _2531 = (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                        float _2538;
                                        float _2541;
                                        float2 _2543;
                                        float2 _2545;
                                        _2538 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2126, 0.0f).x);
                                        _2541 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2531, 0.0f).x);
                                        _2543 = _2114;
                                        _2545 = _149[2];
                                        for (uint _2547 = 0u; _2547 < 2u; )
                                        {
                                            float2 _2551 = _2543 + _2545;
                                            float2 _2552 = _2551 * 0.5f;
                                            float _2559 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2552 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float _2560 = _2559 * _2538;
                                            bool _2561 = _2560 < 0.0f;
                                            bool2 _2562 = _2561.xx;
                                            _2538 = _2561 ? _2538 : _2559;
                                            _2541 = _2561 ? _2559 : _2541;
                                            _2543 = float2(_2562.x ? _2543.x : _2552.x, _2562.y ? _2543.y : _2552.y);
                                            _2545 = float2(_2562.x ? _2552.x : _2545.x, _2562.y ? _2552.y : _2545.y);
                                            _2547++;
                                            continue;
                                        }
                                        float2 _2567 = lerp(_2545, _2543, clamp(_2541 / (_2541 - _2538), 0.0f, 1.0f).xx);
                                        float _2568 = _2567.x;
                                        float3 _2570 = float3(_2568, _362, _2567.y);
                                        float _2582;
                                        float _2585;
                                        float2 _2587;
                                        float2 _2589;
                                        _2582 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2173, 0.0f).x);
                                        _2585 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _2531, 0.0f).x);
                                        _2587 = _2166;
                                        _2589 = _149[2];
                                        for (uint _2591 = 0u; _2591 < 2u; )
                                        {
                                            float2 _2595 = _2587 + _2589;
                                            float2 _2596 = _2595 * 0.5f;
                                            float _2603 = _2115 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_2596 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            float _2604 = _2603 * _2582;
                                            bool _2605 = _2604 < 0.0f;
                                            bool2 _2606 = _2605.xx;
                                            _2582 = _2605 ? _2582 : _2603;
                                            _2585 = _2605 ? _2603 : _2585;
                                            _2587 = float2(_2606.x ? _2587.x : _2596.x, _2606.y ? _2587.y : _2596.y);
                                            _2589 = float2(_2606.x ? _2596.x : _2589.x, _2606.y ? _2596.y : _2589.y);
                                            _2591++;
                                            continue;
                                        }
                                        float2 _2620;
                                        float _2621;
                                        bool _2623;
                                        float2 _2611 = lerp(_2589, _2587, clamp(_2585 / (_2585 - _2582), 0.0f, 1.0f).xx);
                                        float _2613 = _2611.y;
                                        float3 _2614 = float3(_2611.x, _362, _2613);
                                        float3 _2615 = lerp(_2570, _2614, 0.5f.xxx);
                                        float _2647;
                                        do
                                        {
                                            _2620 = (_2615.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                            _2621 = _2620.x;
                                            _2623 = !(_2621 < 0.0f);
                                            bool _2627;
                                            if (_2623)
                                            {
                                                _2627 = _2621 > 1.0f;
                                            }
                                            else
                                            {
                                                _2627 = true;
                                            }
                                            bool _2633;
                                            if (!_2627)
                                            {
                                                _2633 = _2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2633 = true;
                                            }
                                            bool _2639;
                                            if (!_2633)
                                            {
                                                _2639 = _2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2639 = true;
                                            }
                                            if (_2639)
                                            {
                                                _2647 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2647 = rocky_terrain.SampleLevel(sampler_default, _2620, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        if (_2647 < 0.00999999977648258209228515625f)
                                        {
                                            _2920 = _2518;
                                            _2921 = _2519;
                                            _2922 = _2520;
                                            _2923 = _2521;
                                            break;
                                        }
                                        bool _2671;
                                        if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_2570.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                        {
                                            _2671 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_2614.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                        }
                                        else
                                        {
                                            _2671 = false;
                                        }
                                        if (_2671)
                                        {
                                            _2920 = _2518;
                                            _2921 = _2519;
                                            _2922 = _2520;
                                            _2923 = _2521;
                                            break;
                                        }
                                        float _2674 = distance(_2570, _2614);
                                        if (_2674 < 0.20000000298023223876953125f)
                                        {
                                            _2920 = _2518;
                                            _2921 = _2519;
                                            _2922 = _2520;
                                            _2923 = _2521;
                                            break;
                                        }
                                        float4 _2679 = terrain_heightmap.GatherRed(sampler_default, _2620, int2(0, 0));
                                        float _2684 = _2679.z;
                                        float _2685 = _2679.y;
                                        float _2687 = _2679.w;
                                        float _2688 = _2679.x;
                                        float2 _2697 = ((float2((_2684 + _2685) - (_2687 + _2688), (_2688 + _2685) - (_2687 + _2684)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                        float _2730;
                                        do
                                        {
                                            bool _2710;
                                            if (_2623)
                                            {
                                                _2710 = _2621 > 1.0f;
                                            }
                                            else
                                            {
                                                _2710 = true;
                                            }
                                            bool _2716;
                                            if (!_2710)
                                            {
                                                _2716 = _2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2716 = true;
                                            }
                                            bool _2722;
                                            if (!_2716)
                                            {
                                                _2722 = _2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2722 = true;
                                            }
                                            if (_2722)
                                            {
                                                _2730 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2730 = rocky_terrain.SampleLevel(sampler_default, _2620, 0.0f).x;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float _2756;
                                        do
                                        {
                                            bool _2736;
                                            if (_2623)
                                            {
                                                _2736 = _2621 > 1.0f;
                                            }
                                            else
                                            {
                                                _2736 = true;
                                            }
                                            bool _2742;
                                            if (!_2736)
                                            {
                                                _2742 = _2620.y < 0.0f;
                                            }
                                            else
                                            {
                                                _2742 = true;
                                            }
                                            bool _2748;
                                            if (!_2742)
                                            {
                                                _2748 = _2620.y > 1.0f;
                                            }
                                            else
                                            {
                                                _2748 = true;
                                            }
                                            if (_2748)
                                            {
                                                _2756 = 0.0f;
                                                break;
                                            }
                                            else
                                            {
                                                _2756 = rocky_terrain.SampleLevel(sampler_default, _2620, 0.0f).z;
                                                break;
                                            }
                                            break; // unreachable workaround
                                        } while(false);
                                        float3 _2760 = cross(normalize(float3(0.0f, _2697.y, 1.0f)), normalize(float3(1.0f, _2697.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_2730 + _2756, 0.0f, 1.0f));
                                        float3 _2761 = _2615 - _2760;
                                        float3 _2765 = normalize(_2614 - _2570);
                                        uint _2770 = uint(((_2568 + _362) + _2613) * 100.0f);
                                        float4x4 _2788 = transpose(float4x4(float4(_2765, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_2765, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                        float4x4 _2790 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2761.x), float4(0.0f, 1.0f, 0.0f, _2761.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _2761.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_2788, float4x4(float4(_2674, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                        uint _2795;
                                        __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _2795);
                                        uint _2809 = uint(int(_2795));
                                        float _2811[12] = { _2790[0].x, _2790[1].x, _2790[2].x, _2790[0].y, _2790[1].y, _2790[2].y, _2790[0].z, _2790[1].z, _2790[2].z, _2790[0].w, _2790[1].w, _2790[2].w };
                                        Affine3Packed _2812 = { _2811 };
                                        Stair _2813 = { _2812 };
                                        [unroll]
                                        for (int _30ident = 0; _30ident < 12; _30ident++)
                                        {
                                            __alias_stairs_buffer.Store<float>(_30ident * 4 + _2809 * 48 + 0, _2813.transform.data[_30ident]);
                                        }
                                        uint _2814 = _2770 ^ 23784u;
                                        uint _2816 = _2814 + (_2814 << 10u);
                                        uint _2818 = _2816 ^ (_2816 >> 6u);
                                        uint _2820 = _2818 + (_2818 << 3u);
                                        uint _2822 = _2820 ^ (_2820 >> 11u);
                                        bool _2825 = _2521 ? _2521 : true;
                                        uint _2826 = _2521 ? _2519 : 8388607u;
                                        bool _2827 = _2520 ? _2520 : true;
                                        uint _2828 = _2520 ? _2518 : 1065353216u;
                                        uint _2916;
                                        uint _2917;
                                        bool _2918;
                                        bool _2919;
                                        if ((asfloat(((_2822 + (_2822 << 15u)) & _2826) | _2828) - 1.0f) > 0.85000002384185791015625f)
                                        {
                                            uint _2836 = _2770 ^ 90455u;
                                            uint _2838 = _2836 + (_2836 << 10u);
                                            uint _2840 = _2838 ^ (_2838 >> 6u);
                                            uint _2842 = _2840 + (_2840 << 3u);
                                            uint _2844 = _2842 ^ (_2842 >> 11u);
                                            bool _2847 = _2825 ? _2825 : true;
                                            uint _2848 = _2825 ? _2826 : 8388607u;
                                            bool _2849 = _2827 ? _2827 : true;
                                            uint _2850 = _2827 ? _2828 : 1065353216u;
                                            float _2855 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_2844 + (_2844 << 15u)) & _2848) | _2850) - 1.0f);
                                            uint _2856 = _2770 ^ 3845u;
                                            uint _2858 = _2856 + (_2856 << 10u);
                                            uint _2860 = _2858 ^ (_2858 >> 6u);
                                            uint _2862 = _2860 + (_2860 << 3u);
                                            uint _2864 = _2862 ^ (_2862 >> 11u);
                                            uint _2868 = _2847 ? _2848 : 8388607u;
                                            uint _2870 = _2849 ? _2850 : 1065353216u;
                                            float3 _2877 = lerp(_2570, _2614, (asfloat(((_2864 + (_2864 << 15u)) & _2868) | _2870) - 1.0f).xxx) - _2760;
                                            uint _2881;
                                            __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _2881);
                                            uint _2886 = _Globals_pebble_buffer_offset + uint(int(_2881));
                                            float4x4 _2899 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2877.x), float4(0.0f, 1.0f, 0.0f, _2877.y), float4(0.0f, 0.0f, 1.0f, _2877.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_2788, float4x4(float4(_2855, 0.0f, 0.0f, 0.0f), float4(0.0f, _2855, 0.0f, 0.0f), float4(0.0f, 0.0f, _2855, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            float _2913[12] = { _2899[0].x, _2899[1].x, _2899[2].x, _2899[0].y, _2899[1].y, _2899[2].y, _2899[0].z, _2899[1].z, _2899[2].z, _2899[0].w, _2899[1].w, _2899[2].w };
                                            Affine3Packed _2914 = { _2913 };
                                            Stair _2915 = { _2914 };
                                            [unroll]
                                            for (int _31ident = 0; _31ident < 12; _31ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_31ident * 4 + _2886 * 48 + 0, _2915.transform.data[_31ident]);
                                            }
                                            _2916 = _2870;
                                            _2917 = _2868;
                                            _2918 = _2849 ? _2849 : true;
                                            _2919 = _2847 ? _2847 : true;
                                        }
                                        else
                                        {
                                            _2916 = _2828;
                                            _2917 = _2826;
                                            _2918 = _2827;
                                            _2919 = _2825;
                                        }
                                        _2920 = _2916;
                                        _2921 = _2917;
                                        _2922 = _2918;
                                        _2923 = _2919;
                                        break;
                                    } while(false);
                                    _2924 = _2920;
                                    _2925 = _2921;
                                    _2926 = _2922;
                                    _2927 = _2923;
                                }
                                else
                                {
                                    bool _445;
                                    if (!(_389 == 6))
                                    {
                                        _445 = _389 == 9;
                                    }
                                    else
                                    {
                                        _445 = true;
                                    }
                                    uint _2107;
                                    uint _2108;
                                    bool _2109;
                                    bool _2110;
                                    if (_445)
                                    {
                                        uint _2103;
                                        uint _2104;
                                        bool _2105;
                                        bool _2106;
                                        do
                                        {
                                            float _1697 = -_362;
                                            float _1715;
                                            float _1718;
                                            float2 _1720;
                                            float2 _1722;
                                            _1715 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            _1718 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            _1720 = _149[0];
                                            _1722 = _149[1];
                                            for (uint _1724 = 0u; _1724 < 2u; )
                                            {
                                                float2 _1728 = _1720 + _1722;
                                                float2 _1729 = _1728 * 0.5f;
                                                float _1736 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1729 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                float _1737 = _1736 * _1715;
                                                bool _1738 = _1737 < 0.0f;
                                                bool2 _1739 = _1738.xx;
                                                _1715 = _1738 ? _1715 : _1736;
                                                _1718 = _1738 ? _1736 : _1718;
                                                _1720 = float2(_1739.x ? _1720.x : _1729.x, _1739.y ? _1720.y : _1729.y);
                                                _1722 = float2(_1739.x ? _1729.x : _1722.x, _1739.y ? _1729.y : _1722.y);
                                                _1724++;
                                                continue;
                                            }
                                            float2 _1744 = lerp(_1722, _1720, clamp(_1718 / (_1718 - _1715), 0.0f, 1.0f).xx);
                                            float _1745 = _1744.x;
                                            float3 _1747 = float3(_1745, _362, _1744.y);
                                            float _1765;
                                            float _1768;
                                            float2 _1770;
                                            float2 _1772;
                                            _1765 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            _1768 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[2] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                            _1770 = _149[3];
                                            _1772 = _149[2];
                                            for (uint _1774 = 0u; _1774 < 2u; )
                                            {
                                                float2 _1778 = _1770 + _1772;
                                                float2 _1779 = _1778 * 0.5f;
                                                float _1786 = _1697 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1779 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                float _1787 = _1786 * _1765;
                                                bool _1788 = _1787 < 0.0f;
                                                bool2 _1789 = _1788.xx;
                                                _1765 = _1788 ? _1765 : _1786;
                                                _1768 = _1788 ? _1786 : _1768;
                                                _1770 = float2(_1789.x ? _1770.x : _1779.x, _1789.y ? _1770.y : _1779.y);
                                                _1772 = float2(_1789.x ? _1779.x : _1772.x, _1789.y ? _1779.y : _1772.y);
                                                _1774++;
                                                continue;
                                            }
                                            float2 _1803;
                                            float _1804;
                                            bool _1806;
                                            float2 _1794 = lerp(_1772, _1770, clamp(_1768 / (_1768 - _1765), 0.0f, 1.0f).xx);
                                            float _1796 = _1794.y;
                                            float3 _1797 = float3(_1794.x, _362, _1796);
                                            float3 _1798 = lerp(_1747, _1797, 0.5f.xxx);
                                            float _1830;
                                            do
                                            {
                                                _1803 = (_1798.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                _1804 = _1803.x;
                                                _1806 = !(_1804 < 0.0f);
                                                bool _1810;
                                                if (_1806)
                                                {
                                                    _1810 = _1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    _1810 = true;
                                                }
                                                bool _1816;
                                                if (!_1810)
                                                {
                                                    _1816 = _1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    _1816 = true;
                                                }
                                                bool _1822;
                                                if (!_1816)
                                                {
                                                    _1822 = _1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    _1822 = true;
                                                }
                                                if (_1822)
                                                {
                                                    _1830 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    _1830 = rocky_terrain.SampleLevel(sampler_default, _1803, 0.0f).x;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            if (_1830 < 0.00999999977648258209228515625f)
                                            {
                                                _2103 = _346;
                                                _2104 = _349;
                                                _2105 = _351;
                                                _2106 = _353;
                                                break;
                                            }
                                            bool _1854;
                                            if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_1747.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                            {
                                                _1854 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_1797.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                            }
                                            else
                                            {
                                                _1854 = false;
                                            }
                                            if (_1854)
                                            {
                                                _2103 = _346;
                                                _2104 = _349;
                                                _2105 = _351;
                                                _2106 = _353;
                                                break;
                                            }
                                            float _1857 = distance(_1747, _1797);
                                            if (_1857 < 0.20000000298023223876953125f)
                                            {
                                                _2103 = _346;
                                                _2104 = _349;
                                                _2105 = _351;
                                                _2106 = _353;
                                                break;
                                            }
                                            float4 _1862 = terrain_heightmap.GatherRed(sampler_default, _1803, int2(0, 0));
                                            float _1867 = _1862.z;
                                            float _1868 = _1862.y;
                                            float _1870 = _1862.w;
                                            float _1871 = _1862.x;
                                            float2 _1880 = ((float2((_1867 + _1868) - (_1870 + _1871), (_1871 + _1868) - (_1870 + _1867)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                            float _1913;
                                            do
                                            {
                                                bool _1893;
                                                if (_1806)
                                                {
                                                    _1893 = _1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    _1893 = true;
                                                }
                                                bool _1899;
                                                if (!_1893)
                                                {
                                                    _1899 = _1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    _1899 = true;
                                                }
                                                bool _1905;
                                                if (!_1899)
                                                {
                                                    _1905 = _1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    _1905 = true;
                                                }
                                                if (_1905)
                                                {
                                                    _1913 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    _1913 = rocky_terrain.SampleLevel(sampler_default, _1803, 0.0f).x;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            float _1939;
                                            do
                                            {
                                                bool _1919;
                                                if (_1806)
                                                {
                                                    _1919 = _1804 > 1.0f;
                                                }
                                                else
                                                {
                                                    _1919 = true;
                                                }
                                                bool _1925;
                                                if (!_1919)
                                                {
                                                    _1925 = _1803.y < 0.0f;
                                                }
                                                else
                                                {
                                                    _1925 = true;
                                                }
                                                bool _1931;
                                                if (!_1925)
                                                {
                                                    _1931 = _1803.y > 1.0f;
                                                }
                                                else
                                                {
                                                    _1931 = true;
                                                }
                                                if (_1931)
                                                {
                                                    _1939 = 0.0f;
                                                    break;
                                                }
                                                else
                                                {
                                                    _1939 = rocky_terrain.SampleLevel(sampler_default, _1803, 0.0f).z;
                                                    break;
                                                }
                                                break; // unreachable workaround
                                            } while(false);
                                            float3 _1943 = cross(normalize(float3(0.0f, _1880.y, 1.0f)), normalize(float3(1.0f, _1880.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_1913 + _1939, 0.0f, 1.0f));
                                            float3 _1944 = _1798 - _1943;
                                            float3 _1948 = normalize(_1797 - _1747);
                                            uint _1953 = uint(((_1745 + _362) + _1796) * 100.0f);
                                            float4x4 _1971 = transpose(float4x4(float4(_1948, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_1948, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                            float4x4 _1973 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1944.x), float4(0.0f, 1.0f, 0.0f, _1944.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _1944.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1971, float4x4(float4(_1857, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                            uint _1978;
                                            __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _1978);
                                            uint _1992 = uint(int(_1978));
                                            float _1994[12] = { _1973[0].x, _1973[1].x, _1973[2].x, _1973[0].y, _1973[1].y, _1973[2].y, _1973[0].z, _1973[1].z, _1973[2].z, _1973[0].w, _1973[1].w, _1973[2].w };
                                            Affine3Packed _1995 = { _1994 };
                                            Stair _1996 = { _1995 };
                                            [unroll]
                                            for (int _32ident = 0; _32ident < 12; _32ident++)
                                            {
                                                __alias_stairs_buffer.Store<float>(_32ident * 4 + _1992 * 48 + 0, _1996.transform.data[_32ident]);
                                            }
                                            uint _1997 = _1953 ^ 23784u;
                                            uint _1999 = _1997 + (_1997 << 10u);
                                            uint _2001 = _1999 ^ (_1999 >> 6u);
                                            uint _2003 = _2001 + (_2001 << 3u);
                                            uint _2005 = _2003 ^ (_2003 >> 11u);
                                            bool _2008 = _353 ? _353 : true;
                                            uint _2009 = _353 ? _349 : 8388607u;
                                            bool _2010 = _351 ? _351 : true;
                                            uint _2011 = _351 ? _346 : 1065353216u;
                                            uint _2099;
                                            uint _2100;
                                            bool _2101;
                                            bool _2102;
                                            if ((asfloat(((_2005 + (_2005 << 15u)) & _2009) | _2011) - 1.0f) > 0.85000002384185791015625f)
                                            {
                                                uint _2019 = _1953 ^ 90455u;
                                                uint _2021 = _2019 + (_2019 << 10u);
                                                uint _2023 = _2021 ^ (_2021 >> 6u);
                                                uint _2025 = _2023 + (_2023 << 3u);
                                                uint _2027 = _2025 ^ (_2025 >> 11u);
                                                bool _2030 = _2008 ? _2008 : true;
                                                uint _2031 = _2008 ? _2009 : 8388607u;
                                                bool _2032 = _2010 ? _2010 : true;
                                                uint _2033 = _2010 ? _2011 : 1065353216u;
                                                float _2038 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_2027 + (_2027 << 15u)) & _2031) | _2033) - 1.0f);
                                                uint _2039 = _1953 ^ 3845u;
                                                uint _2041 = _2039 + (_2039 << 10u);
                                                uint _2043 = _2041 ^ (_2041 >> 6u);
                                                uint _2045 = _2043 + (_2043 << 3u);
                                                uint _2047 = _2045 ^ (_2045 >> 11u);
                                                uint _2051 = _2030 ? _2031 : 8388607u;
                                                uint _2053 = _2032 ? _2033 : 1065353216u;
                                                float3 _2060 = lerp(_1747, _1797, (asfloat(((_2047 + (_2047 << 15u)) & _2051) | _2053) - 1.0f).xxx) - _1943;
                                                uint _2064;
                                                __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _2064);
                                                uint _2069 = _Globals_pebble_buffer_offset + uint(int(_2064));
                                                float4x4 _2082 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _2060.x), float4(0.0f, 1.0f, 0.0f, _2060.y), float4(0.0f, 0.0f, 1.0f, _2060.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1971, float4x4(float4(_2038, 0.0f, 0.0f, 0.0f), float4(0.0f, _2038, 0.0f, 0.0f), float4(0.0f, 0.0f, _2038, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                float _2096[12] = { _2082[0].x, _2082[1].x, _2082[2].x, _2082[0].y, _2082[1].y, _2082[2].y, _2082[0].z, _2082[1].z, _2082[2].z, _2082[0].w, _2082[1].w, _2082[2].w };
                                                Affine3Packed _2097 = { _2096 };
                                                Stair _2098 = { _2097 };
                                                [unroll]
                                                for (int _33ident = 0; _33ident < 12; _33ident++)
                                                {
                                                    __alias_stairs_buffer.Store<float>(_33ident * 4 + _2069 * 48 + 0, _2098.transform.data[_33ident]);
                                                }
                                                _2099 = _2053;
                                                _2100 = _2051;
                                                _2101 = _2032 ? _2032 : true;
                                                _2102 = _2030 ? _2030 : true;
                                            }
                                            else
                                            {
                                                _2099 = _2011;
                                                _2100 = _2009;
                                                _2101 = _2010;
                                                _2102 = _2008;
                                            }
                                            _2103 = _2099;
                                            _2104 = _2100;
                                            _2105 = _2101;
                                            _2106 = _2102;
                                            break;
                                        } while(false);
                                        _2107 = _2103;
                                        _2108 = _2104;
                                        _2109 = _2105;
                                        _2110 = _2106;
                                    }
                                    else
                                    {
                                        bool _454;
                                        if (!(_389 == 7))
                                        {
                                            _454 = _389 == 8;
                                        }
                                        else
                                        {
                                            _454 = true;
                                        }
                                        uint _1689;
                                        uint _1690;
                                        bool _1691;
                                        bool _1692;
                                        if (_454)
                                        {
                                            uint _1685;
                                            uint _1686;
                                            bool _1687;
                                            bool _1688;
                                            do
                                            {
                                                float _1282 = -_362;
                                                float2 _1284 = (_149[0] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                float _1300;
                                                float _1303;
                                                float2 _1305;
                                                float2 _1307;
                                                _1300 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _1284, 0.0f).x);
                                                _1303 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                _1305 = _149[0];
                                                _1307 = _149[1];
                                                for (uint _1309 = 0u; _1309 < 2u; )
                                                {
                                                    float2 _1313 = _1305 + _1307;
                                                    float2 _1314 = _1313 * 0.5f;
                                                    float _1321 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1314 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                    float _1322 = _1321 * _1300;
                                                    bool _1323 = _1322 < 0.0f;
                                                    bool2 _1324 = _1323.xx;
                                                    _1300 = _1323 ? _1300 : _1321;
                                                    _1303 = _1323 ? _1321 : _1303;
                                                    _1305 = float2(_1324.x ? _1305.x : _1314.x, _1324.y ? _1305.y : _1314.y);
                                                    _1307 = float2(_1324.x ? _1314.x : _1307.x, _1324.y ? _1314.y : _1307.y);
                                                    _1309++;
                                                    continue;
                                                }
                                                float2 _1329 = lerp(_1307, _1305, clamp(_1303 / (_1303 - _1300), 0.0f, 1.0f).xx);
                                                float _1330 = _1329.x;
                                                float3 _1332 = float3(_1330, _362, _1329.y);
                                                float _1347;
                                                float _1350;
                                                float2 _1352;
                                                float2 _1354;
                                                _1347 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _1284, 0.0f).x);
                                                _1350 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                _1352 = _149[0];
                                                _1354 = _149[3];
                                                for (uint _1356 = 0u; _1356 < 2u; )
                                                {
                                                    float2 _1360 = _1352 + _1354;
                                                    float2 _1361 = _1360 * 0.5f;
                                                    float _1368 = _1282 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_1361 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                    float _1369 = _1368 * _1347;
                                                    bool _1370 = _1369 < 0.0f;
                                                    bool2 _1371 = _1370.xx;
                                                    _1347 = _1370 ? _1347 : _1368;
                                                    _1350 = _1370 ? _1368 : _1350;
                                                    _1352 = float2(_1371.x ? _1352.x : _1361.x, _1371.y ? _1352.y : _1361.y);
                                                    _1354 = float2(_1371.x ? _1361.x : _1354.x, _1371.y ? _1361.y : _1354.y);
                                                    _1356++;
                                                    continue;
                                                }
                                                float2 _1385;
                                                float _1386;
                                                bool _1388;
                                                float2 _1376 = lerp(_1354, _1352, clamp(_1350 / (_1350 - _1347), 0.0f, 1.0f).xx);
                                                float _1378 = _1376.y;
                                                float3 _1379 = float3(_1376.x, _362, _1378);
                                                float3 _1380 = lerp(_1332, _1379, 0.5f.xxx);
                                                float _1412;
                                                do
                                                {
                                                    _1385 = (_1380.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    _1386 = _1385.x;
                                                    _1388 = !(_1386 < 0.0f);
                                                    bool _1392;
                                                    if (_1388)
                                                    {
                                                        _1392 = _1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1392 = true;
                                                    }
                                                    bool _1398;
                                                    if (!_1392)
                                                    {
                                                        _1398 = _1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        _1398 = true;
                                                    }
                                                    bool _1404;
                                                    if (!_1398)
                                                    {
                                                        _1404 = _1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1404 = true;
                                                    }
                                                    if (_1404)
                                                    {
                                                        _1412 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        _1412 = rocky_terrain.SampleLevel(sampler_default, _1385, 0.0f).x;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                if (_1412 < 0.00999999977648258209228515625f)
                                                {
                                                    _1685 = _346;
                                                    _1686 = _349;
                                                    _1687 = _351;
                                                    _1688 = _353;
                                                    break;
                                                }
                                                bool _1436;
                                                if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_1332.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                {
                                                    _1436 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_1379.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                }
                                                else
                                                {
                                                    _1436 = false;
                                                }
                                                if (_1436)
                                                {
                                                    _1685 = _346;
                                                    _1686 = _349;
                                                    _1687 = _351;
                                                    _1688 = _353;
                                                    break;
                                                }
                                                float _1439 = distance(_1332, _1379);
                                                if (_1439 < 0.20000000298023223876953125f)
                                                {
                                                    _1685 = _346;
                                                    _1686 = _349;
                                                    _1687 = _351;
                                                    _1688 = _353;
                                                    break;
                                                }
                                                float4 _1444 = terrain_heightmap.GatherRed(sampler_default, _1385, int2(0, 0));
                                                float _1449 = _1444.z;
                                                float _1450 = _1444.y;
                                                float _1452 = _1444.w;
                                                float _1453 = _1444.x;
                                                float2 _1462 = ((float2((_1449 + _1450) - (_1452 + _1453), (_1453 + _1450) - (_1452 + _1449)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                                float _1495;
                                                do
                                                {
                                                    bool _1475;
                                                    if (_1388)
                                                    {
                                                        _1475 = _1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1475 = true;
                                                    }
                                                    bool _1481;
                                                    if (!_1475)
                                                    {
                                                        _1481 = _1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        _1481 = true;
                                                    }
                                                    bool _1487;
                                                    if (!_1481)
                                                    {
                                                        _1487 = _1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1487 = true;
                                                    }
                                                    if (_1487)
                                                    {
                                                        _1495 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        _1495 = rocky_terrain.SampleLevel(sampler_default, _1385, 0.0f).x;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                float _1521;
                                                do
                                                {
                                                    bool _1501;
                                                    if (_1388)
                                                    {
                                                        _1501 = _1386 > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1501 = true;
                                                    }
                                                    bool _1507;
                                                    if (!_1501)
                                                    {
                                                        _1507 = _1385.y < 0.0f;
                                                    }
                                                    else
                                                    {
                                                        _1507 = true;
                                                    }
                                                    bool _1513;
                                                    if (!_1507)
                                                    {
                                                        _1513 = _1385.y > 1.0f;
                                                    }
                                                    else
                                                    {
                                                        _1513 = true;
                                                    }
                                                    if (_1513)
                                                    {
                                                        _1521 = 0.0f;
                                                        break;
                                                    }
                                                    else
                                                    {
                                                        _1521 = rocky_terrain.SampleLevel(sampler_default, _1385, 0.0f).z;
                                                        break;
                                                    }
                                                    break; // unreachable workaround
                                                } while(false);
                                                float3 _1525 = cross(normalize(float3(0.0f, _1462.y, 1.0f)), normalize(float3(1.0f, _1462.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_1495 + _1521, 0.0f, 1.0f));
                                                float3 _1526 = _1380 - _1525;
                                                float3 _1530 = normalize(_1379 - _1332);
                                                uint _1535 = uint(((_1330 + _362) + _1378) * 100.0f);
                                                float4x4 _1553 = transpose(float4x4(float4(_1530, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_1530, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                float4x4 _1555 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1526.x), float4(0.0f, 1.0f, 0.0f, _1526.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _1526.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1553, float4x4(float4(_1439, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                uint _1560;
                                                __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _1560);
                                                uint _1574 = uint(int(_1560));
                                                float _1576[12] = { _1555[0].x, _1555[1].x, _1555[2].x, _1555[0].y, _1555[1].y, _1555[2].y, _1555[0].z, _1555[1].z, _1555[2].z, _1555[0].w, _1555[1].w, _1555[2].w };
                                                Affine3Packed _1577 = { _1576 };
                                                Stair _1578 = { _1577 };
                                                [unroll]
                                                for (int _34ident = 0; _34ident < 12; _34ident++)
                                                {
                                                    __alias_stairs_buffer.Store<float>(_34ident * 4 + _1574 * 48 + 0, _1578.transform.data[_34ident]);
                                                }
                                                uint _1579 = _1535 ^ 23784u;
                                                uint _1581 = _1579 + (_1579 << 10u);
                                                uint _1583 = _1581 ^ (_1581 >> 6u);
                                                uint _1585 = _1583 + (_1583 << 3u);
                                                uint _1587 = _1585 ^ (_1585 >> 11u);
                                                bool _1590 = _353 ? _353 : true;
                                                uint _1591 = _353 ? _349 : 8388607u;
                                                bool _1592 = _351 ? _351 : true;
                                                uint _1593 = _351 ? _346 : 1065353216u;
                                                uint _1681;
                                                uint _1682;
                                                bool _1683;
                                                bool _1684;
                                                if ((asfloat(((_1587 + (_1587 << 15u)) & _1591) | _1593) - 1.0f) > 0.85000002384185791015625f)
                                                {
                                                    uint _1601 = _1535 ^ 90455u;
                                                    uint _1603 = _1601 + (_1601 << 10u);
                                                    uint _1605 = _1603 ^ (_1603 >> 6u);
                                                    uint _1607 = _1605 + (_1605 << 3u);
                                                    uint _1609 = _1607 ^ (_1607 >> 11u);
                                                    bool _1612 = _1590 ? _1590 : true;
                                                    uint _1613 = _1590 ? _1591 : 8388607u;
                                                    bool _1614 = _1592 ? _1592 : true;
                                                    uint _1615 = _1592 ? _1593 : 1065353216u;
                                                    float _1620 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_1609 + (_1609 << 15u)) & _1613) | _1615) - 1.0f);
                                                    uint _1621 = _1535 ^ 3845u;
                                                    uint _1623 = _1621 + (_1621 << 10u);
                                                    uint _1625 = _1623 ^ (_1623 >> 6u);
                                                    uint _1627 = _1625 + (_1625 << 3u);
                                                    uint _1629 = _1627 ^ (_1627 >> 11u);
                                                    uint _1633 = _1612 ? _1613 : 8388607u;
                                                    uint _1635 = _1614 ? _1615 : 1065353216u;
                                                    float3 _1642 = lerp(_1332, _1379, (asfloat(((_1629 + (_1629 << 15u)) & _1633) | _1635) - 1.0f).xxx) - _1525;
                                                    uint _1646;
                                                    __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _1646);
                                                    uint _1651 = _Globals_pebble_buffer_offset + uint(int(_1646));
                                                    float4x4 _1664 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1642.x), float4(0.0f, 1.0f, 0.0f, _1642.y), float4(0.0f, 0.0f, 1.0f, _1642.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1553, float4x4(float4(_1620, 0.0f, 0.0f, 0.0f), float4(0.0f, _1620, 0.0f, 0.0f), float4(0.0f, 0.0f, _1620, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    float _1678[12] = { _1664[0].x, _1664[1].x, _1664[2].x, _1664[0].y, _1664[1].y, _1664[2].y, _1664[0].z, _1664[1].z, _1664[2].z, _1664[0].w, _1664[1].w, _1664[2].w };
                                                    Affine3Packed _1679 = { _1678 };
                                                    Stair _1680 = { _1679 };
                                                    [unroll]
                                                    for (int _35ident = 0; _35ident < 12; _35ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_35ident * 4 + _1651 * 48 + 0, _1680.transform.data[_35ident]);
                                                    }
                                                    _1681 = _1635;
                                                    _1682 = _1633;
                                                    _1683 = _1614 ? _1614 : true;
                                                    _1684 = _1612 ? _1612 : true;
                                                }
                                                else
                                                {
                                                    _1681 = _1593;
                                                    _1682 = _1591;
                                                    _1683 = _1592;
                                                    _1684 = _1590;
                                                }
                                                _1685 = _1681;
                                                _1686 = _1682;
                                                _1687 = _1683;
                                                _1688 = _1684;
                                                break;
                                            } while(false);
                                            _1689 = _1685;
                                            _1690 = _1686;
                                            _1691 = _1687;
                                            _1692 = _1688;
                                        }
                                        else
                                        {
                                            uint _1274;
                                            uint _1275;
                                            bool _1276;
                                            bool _1277;
                                            if (_389 == 10)
                                            {
                                                float2 _463;
                                                float _465;
                                                float2 _467;
                                                float2 _516;
                                                float2 _523;
                                                uint _868;
                                                uint _869;
                                                bool _870;
                                                bool _871;
                                                do
                                                {
                                                    _463 = _149[0];
                                                    _465 = -_362;
                                                    _467 = (_463 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float2 _476 = (_149[1] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float _483;
                                                    float _486;
                                                    float2 _488;
                                                    float2 _490;
                                                    _483 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _467, 0.0f).x);
                                                    _486 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _476, 0.0f).x);
                                                    _488 = _463;
                                                    _490 = _149[1];
                                                    for (uint _492 = 0u; _492 < 2u; )
                                                    {
                                                        float2 _496 = _488 + _490;
                                                        float2 _497 = _496 * 0.5f;
                                                        float _504 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_497 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float _505 = _504 * _483;
                                                        bool _506 = _505 < 0.0f;
                                                        bool2 _507 = _506.xx;
                                                        _483 = _506 ? _483 : _504;
                                                        _486 = _506 ? _504 : _486;
                                                        _488 = float2(_507.x ? _488.x : _497.x, _507.y ? _488.y : _497.y);
                                                        _490 = float2(_507.x ? _497.x : _490.x, _507.y ? _497.y : _490.y);
                                                        _492++;
                                                        continue;
                                                    }
                                                    float2 _512 = lerp(_490, _488, clamp(_486 / (_486 - _483), 0.0f, 1.0f).xx);
                                                    float _513 = _512.x;
                                                    float3 _515 = float3(_513, _362, _512.y);
                                                    _516 = _149[2];
                                                    _523 = (_516 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float _530;
                                                    float _533;
                                                    float2 _535;
                                                    float2 _537;
                                                    _530 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _476, 0.0f).x);
                                                    _533 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _523, 0.0f).x);
                                                    _535 = _149[1];
                                                    _537 = _516;
                                                    for (uint _539 = 0u; _539 < 2u; )
                                                    {
                                                        float2 _543 = _535 + _537;
                                                        float2 _544 = _543 * 0.5f;
                                                        float _551 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_544 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float _552 = _551 * _530;
                                                        bool _553 = _552 < 0.0f;
                                                        bool2 _554 = _553.xx;
                                                        _530 = _553 ? _530 : _551;
                                                        _533 = _553 ? _551 : _533;
                                                        _535 = float2(_554.x ? _535.x : _544.x, _554.y ? _535.y : _544.y);
                                                        _537 = float2(_554.x ? _544.x : _537.x, _554.y ? _544.y : _537.y);
                                                        _539++;
                                                        continue;
                                                    }
                                                    float2 _568;
                                                    float _569;
                                                    bool _571;
                                                    float2 _559 = lerp(_537, _535, clamp(_533 / (_533 - _530), 0.0f, 1.0f).xx);
                                                    float _561 = _559.y;
                                                    float3 _562 = float3(_559.x, _362, _561);
                                                    float3 _563 = lerp(_515, _562, 0.5f.xxx);
                                                    float _595;
                                                    do
                                                    {
                                                        _568 = (_563.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                        _569 = _568.x;
                                                        _571 = !(_569 < 0.0f);
                                                        bool _575;
                                                        if (_571)
                                                        {
                                                            _575 = _569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _575 = true;
                                                        }
                                                        bool _581;
                                                        if (!_575)
                                                        {
                                                            _581 = _568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _581 = true;
                                                        }
                                                        bool _587;
                                                        if (!_581)
                                                        {
                                                            _587 = _568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _587 = true;
                                                        }
                                                        if (_587)
                                                        {
                                                            _595 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _595 = rocky_terrain.SampleLevel(sampler_default, _568, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    if (_595 < 0.00999999977648258209228515625f)
                                                    {
                                                        _868 = _346;
                                                        _869 = _349;
                                                        _870 = _351;
                                                        _871 = _353;
                                                        break;
                                                    }
                                                    bool _619;
                                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_515.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                    {
                                                        _619 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_562.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                    }
                                                    else
                                                    {
                                                        _619 = false;
                                                    }
                                                    if (_619)
                                                    {
                                                        _868 = _346;
                                                        _869 = _349;
                                                        _870 = _351;
                                                        _871 = _353;
                                                        break;
                                                    }
                                                    float _622 = distance(_515, _562);
                                                    if (_622 < 0.20000000298023223876953125f)
                                                    {
                                                        _868 = _346;
                                                        _869 = _349;
                                                        _870 = _351;
                                                        _871 = _353;
                                                        break;
                                                    }
                                                    float4 _627 = terrain_heightmap.GatherRed(sampler_default, _568, int2(0, 0));
                                                    float _632 = _627.z;
                                                    float _633 = _627.y;
                                                    float _635 = _627.w;
                                                    float _636 = _627.x;
                                                    float2 _645 = ((float2((_632 + _633) - (_635 + _636), (_636 + _633) - (_635 + _632)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                                    float _678;
                                                    do
                                                    {
                                                        bool _658;
                                                        if (_571)
                                                        {
                                                            _658 = _569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _658 = true;
                                                        }
                                                        bool _664;
                                                        if (!_658)
                                                        {
                                                            _664 = _568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _664 = true;
                                                        }
                                                        bool _670;
                                                        if (!_664)
                                                        {
                                                            _670 = _568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _670 = true;
                                                        }
                                                        if (_670)
                                                        {
                                                            _678 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _678 = rocky_terrain.SampleLevel(sampler_default, _568, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float _704;
                                                    do
                                                    {
                                                        bool _684;
                                                        if (_571)
                                                        {
                                                            _684 = _569 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _684 = true;
                                                        }
                                                        bool _690;
                                                        if (!_684)
                                                        {
                                                            _690 = _568.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _690 = true;
                                                        }
                                                        bool _696;
                                                        if (!_690)
                                                        {
                                                            _696 = _568.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _696 = true;
                                                        }
                                                        if (_696)
                                                        {
                                                            _704 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _704 = rocky_terrain.SampleLevel(sampler_default, _568, 0.0f).z;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float3 _708 = cross(normalize(float3(0.0f, _645.y, 1.0f)), normalize(float3(1.0f, _645.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_678 + _704, 0.0f, 1.0f));
                                                    float3 _709 = _563 - _708;
                                                    float3 _713 = normalize(_562 - _515);
                                                    uint _718 = uint(((_513 + _362) + _561) * 100.0f);
                                                    float4x4 _736 = transpose(float4x4(float4(_713, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_713, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                    float4x4 _738 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _709.x), float4(0.0f, 1.0f, 0.0f, _709.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _709.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_736, float4x4(float4(_622, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint _743;
                                                    __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _743);
                                                    uint _757 = uint(int(_743));
                                                    float _759[12] = { _738[0].x, _738[1].x, _738[2].x, _738[0].y, _738[1].y, _738[2].y, _738[0].z, _738[1].z, _738[2].z, _738[0].w, _738[1].w, _738[2].w };
                                                    Affine3Packed _760 = { _759 };
                                                    Stair _761 = { _760 };
                                                    [unroll]
                                                    for (int _36ident = 0; _36ident < 12; _36ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_36ident * 4 + _757 * 48 + 0, _761.transform.data[_36ident]);
                                                    }
                                                    uint _762 = _718 ^ 23784u;
                                                    uint _764 = _762 + (_762 << 10u);
                                                    uint _766 = _764 ^ (_764 >> 6u);
                                                    uint _768 = _766 + (_766 << 3u);
                                                    uint _770 = _768 ^ (_768 >> 11u);
                                                    bool _773 = _353 ? _353 : true;
                                                    uint _774 = _353 ? _349 : 8388607u;
                                                    bool _775 = _351 ? _351 : true;
                                                    uint _776 = _351 ? _346 : 1065353216u;
                                                    uint _864;
                                                    uint _865;
                                                    bool _866;
                                                    bool _867;
                                                    if ((asfloat(((_770 + (_770 << 15u)) & _774) | _776) - 1.0f) > 0.85000002384185791015625f)
                                                    {
                                                        uint _784 = _718 ^ 90455u;
                                                        uint _786 = _784 + (_784 << 10u);
                                                        uint _788 = _786 ^ (_786 >> 6u);
                                                        uint _790 = _788 + (_788 << 3u);
                                                        uint _792 = _790 ^ (_790 >> 11u);
                                                        bool _795 = _773 ? _773 : true;
                                                        uint _796 = _773 ? _774 : 8388607u;
                                                        bool _797 = _775 ? _775 : true;
                                                        uint _798 = _775 ? _776 : 1065353216u;
                                                        float _803 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_792 + (_792 << 15u)) & _796) | _798) - 1.0f);
                                                        uint _804 = _718 ^ 3845u;
                                                        uint _806 = _804 + (_804 << 10u);
                                                        uint _808 = _806 ^ (_806 >> 6u);
                                                        uint _810 = _808 + (_808 << 3u);
                                                        uint _812 = _810 ^ (_810 >> 11u);
                                                        uint _816 = _795 ? _796 : 8388607u;
                                                        uint _818 = _797 ? _798 : 1065353216u;
                                                        float3 _825 = lerp(_515, _562, (asfloat(((_812 + (_812 << 15u)) & _816) | _818) - 1.0f).xxx) - _708;
                                                        uint _829;
                                                        __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _829);
                                                        uint _834 = _Globals_pebble_buffer_offset + uint(int(_829));
                                                        float4x4 _847 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _825.x), float4(0.0f, 1.0f, 0.0f, _825.y), float4(0.0f, 0.0f, 1.0f, _825.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_736, float4x4(float4(_803, 0.0f, 0.0f, 0.0f), float4(0.0f, _803, 0.0f, 0.0f), float4(0.0f, 0.0f, _803, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                        float _861[12] = { _847[0].x, _847[1].x, _847[2].x, _847[0].y, _847[1].y, _847[2].y, _847[0].z, _847[1].z, _847[2].z, _847[0].w, _847[1].w, _847[2].w };
                                                        Affine3Packed _862 = { _861 };
                                                        Stair _863 = { _862 };
                                                        [unroll]
                                                        for (int _37ident = 0; _37ident < 12; _37ident++)
                                                        {
                                                            __alias_stairs_buffer.Store<float>(_37ident * 4 + _834 * 48 + 0, _863.transform.data[_37ident]);
                                                        }
                                                        _864 = _818;
                                                        _865 = _816;
                                                        _866 = _797 ? _797 : true;
                                                        _867 = _795 ? _795 : true;
                                                    }
                                                    else
                                                    {
                                                        _864 = _776;
                                                        _865 = _774;
                                                        _866 = _775;
                                                        _867 = _773;
                                                    }
                                                    _868 = _864;
                                                    _869 = _865;
                                                    _870 = _866;
                                                    _871 = _867;
                                                    break;
                                                } while(false);
                                                uint _1270;
                                                uint _1271;
                                                bool _1272;
                                                bool _1273;
                                                do
                                                {
                                                    float2 _881 = (_149[3] * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                    float _888;
                                                    float _891;
                                                    float2 _893;
                                                    float2 _895;
                                                    _888 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _467, 0.0f).x);
                                                    _891 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _881, 0.0f).x);
                                                    _893 = _463;
                                                    _895 = _149[3];
                                                    for (uint _897 = 0u; _897 < 2u; )
                                                    {
                                                        float2 _901 = _893 + _895;
                                                        float2 _902 = _901 * 0.5f;
                                                        float _909 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_902 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float _910 = _909 * _888;
                                                        bool _911 = _910 < 0.0f;
                                                        bool2 _912 = _911.xx;
                                                        _888 = _911 ? _888 : _909;
                                                        _891 = _911 ? _909 : _891;
                                                        _893 = float2(_912.x ? _893.x : _902.x, _912.y ? _893.y : _902.y);
                                                        _895 = float2(_912.x ? _902.x : _895.x, _912.y ? _902.y : _895.y);
                                                        _897++;
                                                        continue;
                                                    }
                                                    float2 _917 = lerp(_895, _893, clamp(_891 / (_891 - _888), 0.0f, 1.0f).xx);
                                                    float _918 = _917.x;
                                                    float3 _920 = float3(_918, _362, _917.y);
                                                    float _932;
                                                    float _935;
                                                    float2 _937;
                                                    float2 _939;
                                                    _932 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _881, 0.0f).x);
                                                    _935 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _523, 0.0f).x);
                                                    _937 = _149[3];
                                                    _939 = _516;
                                                    for (uint _941 = 0u; _941 < 2u; )
                                                    {
                                                        float2 _945 = _937 + _939;
                                                        float2 _946 = _945 * 0.5f;
                                                        float _953 = _465 + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_946 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                                                        float _954 = _953 * _932;
                                                        bool _955 = _954 < 0.0f;
                                                        bool2 _956 = _955.xx;
                                                        _932 = _955 ? _932 : _953;
                                                        _935 = _955 ? _953 : _935;
                                                        _937 = float2(_956.x ? _937.x : _946.x, _956.y ? _937.y : _946.y);
                                                        _939 = float2(_956.x ? _946.x : _939.x, _956.y ? _946.y : _939.y);
                                                        _941++;
                                                        continue;
                                                    }
                                                    float2 _970;
                                                    float _971;
                                                    bool _973;
                                                    float2 _961 = lerp(_939, _937, clamp(_935 / (_935 - _932), 0.0f, 1.0f).xx);
                                                    float _963 = _961.y;
                                                    float3 _964 = float3(_961.x, _362, _963);
                                                    float3 _965 = lerp(_920, _964, 0.5f.xxx);
                                                    float _997;
                                                    do
                                                    {
                                                        _970 = (_965.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                                                        _971 = _970.x;
                                                        _973 = !(_971 < 0.0f);
                                                        bool _977;
                                                        if (_973)
                                                        {
                                                            _977 = _971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _977 = true;
                                                        }
                                                        bool _983;
                                                        if (!_977)
                                                        {
                                                            _983 = _970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _983 = true;
                                                        }
                                                        bool _989;
                                                        if (!_983)
                                                        {
                                                            _989 = _970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _989 = true;
                                                        }
                                                        if (_989)
                                                        {
                                                            _997 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _997 = rocky_terrain.SampleLevel(sampler_default, _970, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    if (_997 < 0.00999999977648258209228515625f)
                                                    {
                                                        _1270 = _868;
                                                        _1271 = _869;
                                                        _1272 = _870;
                                                        _1273 = _871;
                                                        break;
                                                    }
                                                    bool _1021;
                                                    if (smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_920.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f)
                                                    {
                                                        _1021 = smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, (_964.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x) < 0.00999999977648258209228515625f;
                                                    }
                                                    else
                                                    {
                                                        _1021 = false;
                                                    }
                                                    if (_1021)
                                                    {
                                                        _1270 = _868;
                                                        _1271 = _869;
                                                        _1272 = _870;
                                                        _1273 = _871;
                                                        break;
                                                    }
                                                    float _1024 = distance(_920, _964);
                                                    if (_1024 < 0.20000000298023223876953125f)
                                                    {
                                                        _1270 = _868;
                                                        _1271 = _869;
                                                        _1272 = _870;
                                                        _1273 = _871;
                                                        break;
                                                    }
                                                    float4 _1029 = terrain_heightmap.GatherRed(sampler_default, _970, int2(0, 0));
                                                    float _1034 = _1029.z;
                                                    float _1035 = _1029.y;
                                                    float _1037 = _1029.w;
                                                    float _1038 = _1029.x;
                                                    float2 _1047 = ((float2((_1034 + _1035) - (_1037 + _1038), (_1038 + _1035) - (_1037 + _1034)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
                                                    float _1080;
                                                    do
                                                    {
                                                        bool _1060;
                                                        if (_973)
                                                        {
                                                            _1060 = _971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _1060 = true;
                                                        }
                                                        bool _1066;
                                                        if (!_1060)
                                                        {
                                                            _1066 = _970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _1066 = true;
                                                        }
                                                        bool _1072;
                                                        if (!_1066)
                                                        {
                                                            _1072 = _970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _1072 = true;
                                                        }
                                                        if (_1072)
                                                        {
                                                            _1080 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _1080 = rocky_terrain.SampleLevel(sampler_default, _970, 0.0f).x;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float _1106;
                                                    do
                                                    {
                                                        bool _1086;
                                                        if (_973)
                                                        {
                                                            _1086 = _971 > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _1086 = true;
                                                        }
                                                        bool _1092;
                                                        if (!_1086)
                                                        {
                                                            _1092 = _970.y < 0.0f;
                                                        }
                                                        else
                                                        {
                                                            _1092 = true;
                                                        }
                                                        bool _1098;
                                                        if (!_1092)
                                                        {
                                                            _1098 = _970.y > 1.0f;
                                                        }
                                                        else
                                                        {
                                                            _1098 = true;
                                                        }
                                                        if (_1098)
                                                        {
                                                            _1106 = 0.0f;
                                                            break;
                                                        }
                                                        else
                                                        {
                                                            _1106 = rocky_terrain.SampleLevel(sampler_default, _970, 0.0f).z;
                                                            break;
                                                        }
                                                        break; // unreachable workaround
                                                    } while(false);
                                                    float3 _1110 = cross(normalize(float3(0.0f, _1047.y, 1.0f)), normalize(float3(1.0f, _1047.x, 0.0f))) * lerp(0.0f, 0.300000011920928955078125f, clamp(_1080 + _1106, 0.0f, 1.0f));
                                                    float3 _1111 = _965 - _1110;
                                                    float3 _1115 = normalize(_964 - _920);
                                                    uint _1120 = uint(((_918 + _362) + _963) * 100.0f);
                                                    float4x4 _1138 = transpose(float4x4(float4(_1115, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(cross(_1115, float3(0.0f, 1.0f, 0.0f)), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
                                                    float4x4 _1140 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1111.x), float4(0.0f, 1.0f, 0.0f, _1111.y - 0.5f), float4(0.0f, 0.0f, 1.0f, _1111.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1138, float4x4(float4(_1024, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.800000011920928955078125f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                    uint _1145;
                                                    __alias_draw_commands.InterlockedAdd(uint(_Globals_draw_indirect_buffer_offset) * 20 + 4, 1u, _1145);
                                                    uint _1159 = uint(int(_1145));
                                                    float _1161[12] = { _1140[0].x, _1140[1].x, _1140[2].x, _1140[0].y, _1140[1].y, _1140[2].y, _1140[0].z, _1140[1].z, _1140[2].z, _1140[0].w, _1140[1].w, _1140[2].w };
                                                    Affine3Packed _1162 = { _1161 };
                                                    Stair _1163 = { _1162 };
                                                    [unroll]
                                                    for (int _38ident = 0; _38ident < 12; _38ident++)
                                                    {
                                                        __alias_stairs_buffer.Store<float>(_38ident * 4 + _1159 * 48 + 0, _1163.transform.data[_38ident]);
                                                    }
                                                    uint _1164 = _1120 ^ 23784u;
                                                    uint _1166 = _1164 + (_1164 << 10u);
                                                    uint _1168 = _1166 ^ (_1166 >> 6u);
                                                    uint _1170 = _1168 + (_1168 << 3u);
                                                    uint _1172 = _1170 ^ (_1170 >> 11u);
                                                    bool _1175 = _871 ? _871 : true;
                                                    uint _1176 = _871 ? _869 : 8388607u;
                                                    bool _1177 = _870 ? _870 : true;
                                                    uint _1178 = _870 ? _868 : 1065353216u;
                                                    uint _1266;
                                                    uint _1267;
                                                    bool _1268;
                                                    bool _1269;
                                                    if ((asfloat(((_1172 + (_1172 << 15u)) & _1176) | _1178) - 1.0f) > 0.85000002384185791015625f)
                                                    {
                                                        uint _1186 = _1120 ^ 90455u;
                                                        uint _1188 = _1186 + (_1186 << 10u);
                                                        uint _1190 = _1188 ^ (_1188 >> 6u);
                                                        uint _1192 = _1190 + (_1190 << 3u);
                                                        uint _1194 = _1192 ^ (_1192 >> 11u);
                                                        bool _1197 = _1175 ? _1175 : true;
                                                        uint _1198 = _1175 ? _1176 : 8388607u;
                                                        bool _1199 = _1177 ? _1177 : true;
                                                        uint _1200 = _1177 ? _1178 : 1065353216u;
                                                        float _1205 = lerp(0.20000000298023223876953125f, 0.4000000059604644775390625f, asfloat(((_1194 + (_1194 << 15u)) & _1198) | _1200) - 1.0f);
                                                        uint _1206 = _1120 ^ 3845u;
                                                        uint _1208 = _1206 + (_1206 << 10u);
                                                        uint _1210 = _1208 ^ (_1208 >> 6u);
                                                        uint _1212 = _1210 + (_1210 << 3u);
                                                        uint _1214 = _1212 ^ (_1212 >> 11u);
                                                        uint _1218 = _1197 ? _1198 : 8388607u;
                                                        uint _1220 = _1199 ? _1200 : 1065353216u;
                                                        float3 _1227 = lerp(_920, _964, (asfloat(((_1214 + (_1214 << 15u)) & _1218) | _1220) - 1.0f).xxx) - _1110;
                                                        uint _1231;
                                                        __alias_draw_commands.InterlockedAdd(_Globals_pebble_indirect_draw_idx * 20 + 4, 1u, _1231);
                                                        uint _1236 = _Globals_pebble_buffer_offset + uint(int(_1231));
                                                        float4x4 _1249 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, _1227.x), float4(0.0f, 1.0f, 0.0f, _1227.y), float4(0.0f, 0.0f, 1.0f, _1227.z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_1138, float4x4(float4(_1205, 0.0f, 0.0f, 0.0f), float4(0.0f, _1205, 0.0f, 0.0f), float4(0.0f, 0.0f, _1205, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
                                                        float _1263[12] = { _1249[0].x, _1249[1].x, _1249[2].x, _1249[0].y, _1249[1].y, _1249[2].y, _1249[0].z, _1249[1].z, _1249[2].z, _1249[0].w, _1249[1].w, _1249[2].w };
                                                        Affine3Packed _1264 = { _1263 };
                                                        Stair _1265 = { _1264 };
                                                        [unroll]
                                                        for (int _39ident = 0; _39ident < 12; _39ident++)
                                                        {
                                                            __alias_stairs_buffer.Store<float>(_39ident * 4 + _1236 * 48 + 0, _1265.transform.data[_39ident]);
                                                        }
                                                        _1266 = _1220;
                                                        _1267 = _1218;
                                                        _1268 = _1199 ? _1199 : true;
                                                        _1269 = _1197 ? _1197 : true;
                                                    }
                                                    else
                                                    {
                                                        _1266 = _1178;
                                                        _1267 = _1176;
                                                        _1268 = _1177;
                                                        _1269 = _1175;
                                                    }
                                                    _1270 = _1266;
                                                    _1271 = _1267;
                                                    _1272 = _1268;
                                                    _1273 = _1269;
                                                    break;
                                                } while(false);
                                                _1274 = _1270;
                                                _1275 = _1271;
                                                _1276 = _1272;
                                                _1277 = _1273;
                                            }
                                            else
                                            {
                                                _1274 = _346;
                                                _1275 = _349;
                                                _1276 = _351;
                                                _1277 = _353;
                                            }
                                            _1689 = _1274;
                                            _1690 = _1275;
                                            _1691 = _1276;
                                            _1692 = _1277;
                                        }
                                        _2107 = _1689;
                                        _2108 = _1690;
                                        _2109 = _1691;
                                        _2110 = _1692;
                                    }
                                    _2924 = _2107;
                                    _2925 = _2108;
                                    _2926 = _2109;
                                    _2927 = _2110;
                                }
                                _3339 = _2924;
                                _3340 = _2925;
                                _3341 = _2926;
                                _3342 = _2927;
                            }
                            _3757 = _3339;
                            _3758 = _3340;
                            _3759 = _3341;
                            _3760 = _3342;
                        }
                        _4172 = _3757;
                        _4173 = _3758;
                        _4174 = _3759;
                        _4175 = _3760;
                    }
                    _4587 = _4172;
                    _4588 = _4173;
                    _4589 = _4174;
                    _4590 = _4175;
                }
                _347 = _4587;
                _350 = _4588;
                _352 = _4589;
                _354 = _4590;
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
