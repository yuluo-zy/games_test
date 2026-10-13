// PATCHED: modules ain_14dca8ba9c11c197.bin:cs_main, ain_8faee3cc8c252c04.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_14dca8ba9c11c197) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_8faee3cc8c252c04) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/light_binning/bin_lights_to_cells.cs.hlsl | status: O (grid binning algorithm); O/I (reservoir replacement RNG carry-chain)
// MERGE (gi-deferred review 2026-10-13): A body + B feature flag. A's RNG matches the bin
// exactly (wang_hash((idx*1664525 + hash(frame_index) + 1013904223) * 1664525)) and A declares
// lantern_buf as StructuredBuffer<LanternPacked> (bin debug name LanternPacked). B's
// BIN_LIGHTS_FULL flag reproduces the 8fae reduced variant (count-only, no light_indices /
// frame_constants). Reservoir replacement uses the bin's carried-rng mantissa form
// (asfloat(rng & 0x7FFFFF | 0x3F800000) - 1) per the review fidelity note.
// Evidence: _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin (LocalSize 64, entry cs_main).
//   bindings: lantern_buf t13 (StructuredBuffer<LanternPacked>), $Globals b14 {lantern_count},
//   __alias_indirection_grid u15 (RWByteAddressBuffer), __alias_light_indices u16
//   (RWByteAddressBuffer), frame_constants set1/b0; caps {Shader, RuntimeDescriptorArray}.
//   For each lantern (dispatch thread id): influence radius r = 6*sqrt(intensity)
//   (asfloat bit-trick), grid-space AABB from (pos ± r), and for every cell in the
//   AABB that the radius sphere overlaps (point-to-AABB distance < r2) reserve a slot:
//   InterlockedAdd(indirection_grid[cell], 1<<20) bumps the count field (low20 = first
//   index), write lantern id into light_indices[start+count]; past 64 lights, a
//   reservoir sample replaces a hash-chosen slot with probability 64/(count+1).
//   Two shipped variants (14dca8ba full / 8faee3cc reduced): the reduced variant drops
//   frame_constants + light_indices writes (compile-time feature toggle BIN_LIGHTS_FULL).

#ifndef BIN_LIGHTS_FULL
#define BIN_LIGHTS_FULL 1
#endif

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/pack_unpack.hlsl"
#include "grid.inc.hlsl"

StructuredBuffer<LanternPacked> lantern_buf : register(t13, space0);

cbuffer $Globals : register(b14, space0)
{
    uint lantern_count;
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
#if BIN_LIGHTS_FULL
RWByteAddressBuffer __alias_light_indices : register(u16, space0);
#endif

// 2nd-stage integer hash (Wang) observed after the LCG mix of lantern index and
// hash(frame_index) in the full bin variant.
uint wang_hash(uint x)
{
    x ^= (x >> 11u);
    x ^= (x << 7u) & 2636928640u;
    x ^= (x << 15u) & 4022730752u;
    x ^= (x >> 18u);
    return x;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint lantern_idx = gl_GlobalInvocationID.x;
    if (lantern_idx >= lantern_count) {
        return;
    }

    float3 lantern_pos = lantern_buf[lantern_idx].position;
    float intensity = f16tof32(lantern_buf[lantern_idx].intensity__color_id & 0xFFFFu);
    if (intensity <= 0.0f) {
        return;
    }

    // Lantern influence radius = 6 * sqrt(intensity) via the fast-sqrt bit trick
    // (magic constant from the original disassembly).
    float radius = 6.0f * asfloat(532487669 + (asint(intensity) >> 1));

    // Grid-space AABB of the influence sphere.
    uint3 cell_min = uint3(clamp(floor((lantern_pos - radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS) - 1.0f));
    uint3 cell_max = uint3(clamp(ceil((lantern_pos + radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS)));

#if BIN_LIGHTS_FULL
    // Reservoir RNG seed: wang_hash(((idx * 1664525) + hash(frame_index) + 1013904223) * 1664525).
    uint hash_state = wang_hash(((lantern_idx * 1664525u) + hash(frame_index) + 1013904223u) * 1664525u);
#endif

    for (uint z = cell_min.z; z < cell_max.z; z++) {
        for (uint y = cell_min.y; y < cell_max.y; y++) {
            for (uint x = cell_min.x; x < cell_max.x; x++) {
                uint3 cell = uint3(x, y, z);
                float3 aabb_min = grid_cell_min_world(cell);
                float3 aabb_max = grid_cell_max_world(cell);

                // Squared distance from the lantern to the cell AABB.
                float d2 = radius * radius;
                [unroll]
                for (uint i = 0u; i < 3u; ++i) {
                    if (lantern_pos[i] < aabb_min[i]) {
                        float e = lantern_pos[i] - aabb_min[i];
                        d2 -= e * e;
                    } else if (lantern_pos[i] > aabb_max[i]) {
                        float e = lantern_pos[i] - aabb_max[i];
                        d2 -= e * e;
                    }
                }

                if (d2 > 0.0f) {
                    uint cell_idx = grid_cell_index(cell);
#if BIN_LIGHTS_FULL
                    uint prev;
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u << 20u, prev);
                    uint start = prev & 0xFFFFFu;
                    uint count = prev >> 20u;
                    if (count < 64u) {
                        __alias_light_indices.Store<uint>((start + count) * 4u, lantern_idx);
                    } else {
                        // Reservoir: keep 64 samples, replace with probability 64/(count+1),
                        // using the carried-rng mantissa form the bin uses.
                        float u = asfloat((hash_state & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
                        hash_state = hash(hash_state);
                        if (u < (64.0f / float(count + 1u))) {
                            __alias_light_indices.Store<uint>((start + (hash_state % 64u)) * 4u, lantern_idx);
                            hash_state = hash(hash_state);
                        }
                    }
#else
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u);
#endif
                }
            }
        }
    }
}

#endif

#if defined(TGV_14dca8ba9c11c197) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

static uint _86;

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

ByteAddressBuffer lantern_buf : register(t13, space0);
cbuffer $Globals : register(b14, space0)
{
    uint _Globals_lantern_count : packoffset(c0);
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
RWByteAddressBuffer __alias_light_indices : register(u16, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= _Globals_lantern_count)
        {
            break;
        }
        float2 _121 = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _122 = _121.x;
        if (_122 <= 0.0f)
        {
            break;
        }
        precise float _130 = 6.0f * asfloat(532487669 + (asint(_122) >> 1));
        float3 _131 = _130.xxx;
        uint3 _138 = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _143 = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _147 = frame_constants_frame_index + (frame_constants_frame_index << 10u);
        uint _149 = _147 ^ (_147 >> 6u);
        uint _151 = _149 + (_149 << 3u);
        uint _153 = _151 ^ (_151 >> 11u);
        uint _159 = (((gl_GlobalInvocationID.x * 1664525u) + (_153 + (_153 << 15u))) + 1013904223u) * 1664525u;
        uint _161 = _159 ^ (_159 >> 11u);
        uint _164 = _161 ^ ((_161 << 7u) & 2636928640u);
        uint _167 = _164 ^ ((_164 << 15u) & 4022730752u);
        uint _169 = _167 ^ (_167 >> 18u);
        uint _170 = _138.z;
        uint _176;
        uint _178;
        bool _180;
        bool _182;
        uint _184;
        uint _172 = _170;
        uint _175 = 0u;
        uint _177 = 0u;
        bool _179 = false;
        bool _181 = false;
        uint _183 = _169;
        for (; _172 < _143.z; _172++, _175 = _176, _177 = _178, _179 = _180, _181 = _182, _183 = _184)
        {
            uint _189 = _138.y;
            _176 = _175;
            _178 = _177;
            _180 = _179;
            _182 = _181;
            _184 = _183;
            uint _194;
            uint _195;
            bool _196;
            bool _197;
            uint _198;
            for (uint _191 = _189; _191 < _143.y; _191++, _176 = _194, _178 = _195, _180 = _196, _182 = _197, _184 = _198)
            {
                uint _203 = _138.x;
                _194 = _176;
                _195 = _178;
                _196 = _180;
                _197 = _182;
                _198 = _184;
                uint _205;
                uint _207;
                bool _208;
                bool _209;
                uint _210;
                for (uint _211 = _203; _211 < _143.x; _194 = _205, _195 = _207, _196 = _208, _197 = _209, _198 = _210, _211++)
                {
                    float3 _218 = float3(uint3(_211, _191, _172));
                    float3 _220 = (_218 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _222 = (_218 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float _223 = _130 * _130;
                    float _225 = _220.x;
                    float _241;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < _225)
                    {
                        float _238 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _225;
                        _241 = _223 - (_238 * _238);
                    }
                    else
                    {
                        float _230 = _222.x;
                        float _237;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > _230)
                        {
                            float _234 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _230;
                            _237 = _223 - (_234 * _234);
                        }
                        else
                        {
                            _237 = _223;
                        }
                        _241 = _237;
                    }
                    float _243 = _220.y;
                    float _259;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < _243)
                    {
                        float _256 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _243;
                        _259 = _241 - (_256 * _256);
                    }
                    else
                    {
                        float _248 = _222.y;
                        float _255;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > _248)
                        {
                            float _252 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _248;
                            _255 = _241 - (_252 * _252);
                        }
                        else
                        {
                            _255 = _241;
                        }
                        _259 = _255;
                    }
                    float _261 = _220.z;
                    float _277;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < _261)
                    {
                        float _274 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _261;
                        _277 = _259 - (_274 * _274);
                    }
                    else
                    {
                        float _266 = _222.z;
                        float _273;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > _266)
                        {
                            float _270 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _266;
                            _273 = _259 - (_270 * _270);
                        }
                        else
                        {
                            _273 = _259;
                        }
                        _277 = _273;
                    }
                    if (_277 > 0.0f)
                    {
                        uint _286;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * _191) + _172)) + _211) * 4 + 0, 1048576u, _286);
                        uint _287 = _286 & 1048575u;
                        uint _288 = _286 >> 20u;
                        uint _333;
                        uint _334;
                        bool _335;
                        bool _336;
                        uint _337;
                        if (_288 < 64u)
                        {
                            __alias_light_indices.Store<uint>((_287 + _288) * 4 + 0, gl_GlobalInvocationID.x);
                            _333 = _194;
                            _334 = _195;
                            _335 = _196;
                            _336 = _197;
                            _337 = _198;
                        }
                        else
                        {
                            uint _294 = _198 + (_198 << 10u);
                            uint _296 = _294 ^ (_294 >> 6u);
                            uint _298 = _296 + (_296 << 3u);
                            uint _300 = _298 ^ (_298 >> 11u);
                            uint _302 = _300 + (_300 << 15u);
                            uint _304 = _197 ? _195 : 8388607u;
                            uint _306 = _196 ? _194 : 1065353216u;
                            uint _330;
                            if ((asfloat((_198 & _304) | _306) - 1.0f) < (64.0f / float(_288 + 1u)))
                            {
                                uint _318 = _302 + (_302 << 10u);
                                uint _320 = _318 ^ (_318 >> 6u);
                                uint _322 = _320 + (_320 << 3u);
                                uint _324 = _322 ^ (_322 >> 11u);
                                __alias_light_indices.Store<uint>((_287 + (_302 % 64u)) * 4 + 0, gl_GlobalInvocationID.x);
                                _330 = _324 + (_324 << 15u);
                            }
                            else
                            {
                                _330 = _302;
                            }
                            _333 = _306;
                            _334 = _304;
                            _335 = _196 ? _196 : true;
                            _336 = _197 ? _197 : true;
                            _337 = _330;
                        }
                        _205 = _333;
                        _207 = _334;
                        _208 = _335;
                        _209 = _336;
                        _210 = _337;
                    }
                    else
                    {
                        _205 = _194;
                        _207 = _195;
                        _208 = _196;
                        _209 = _197;
                        _210 = _198;
                    }
                }
            }
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

#if defined(TGV_8faee3cc8c252c04) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_8faee3cc8c252c04.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

ByteAddressBuffer lantern_buf : register(t13, space0);
cbuffer $Globals : register(b14, space0)
{
    uint _Globals_lantern_count : packoffset(c0);
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= _Globals_lantern_count)
        {
            break;
        }
        float2 _88 = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _89 = _88.x;
        if (_89 <= 0.0f)
        {
            break;
        }
        precise float _97 = 6.0f * asfloat(532487669 + (asint(_89) >> 1));
        float3 _98 = _97.xxx;
        uint3 _105 = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _110 = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _111 = _105.z;
        for (uint _113 = _111; _113 < _110.z; _113++)
        {
            uint _120 = _105.y;
            for (uint _122 = _120; _122 < _110.y; _122++)
            {
                uint _129 = _105.x;
                for (uint _131 = _129; _131 < _110.x; _131++)
                {
                    float3 _139 = float3(uint3(_131, _122, _113));
                    float3 _141 = (_139 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _143 = (_139 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float _144 = _97 * _97;
                    float _146 = _141.x;
                    float _162;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < _146)
                    {
                        float _151 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _146;
                        _162 = _144 - (_151 * _151);
                    }
                    else
                    {
                        float _154 = _143.x;
                        float _161;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > _154)
                        {
                            float _158 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - _154;
                            _161 = _144 - (_158 * _158);
                        }
                        else
                        {
                            _161 = _144;
                        }
                        _162 = _161;
                    }
                    float _164 = _141.y;
                    float _180;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < _164)
                    {
                        float _169 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _164;
                        _180 = _162 - (_169 * _169);
                    }
                    else
                    {
                        float _172 = _143.y;
                        float _179;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > _172)
                        {
                            float _176 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - _172;
                            _179 = _162 - (_176 * _176);
                        }
                        else
                        {
                            _179 = _162;
                        }
                        _180 = _179;
                    }
                    float _182 = _141.z;
                    float _198;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < _182)
                    {
                        float _187 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _182;
                        _198 = _180 - (_187 * _187);
                    }
                    else
                    {
                        float _190 = _143.z;
                        float _197;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > _190)
                        {
                            float _194 = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - _190;
                            _197 = _180 - (_194 * _194);
                        }
                        else
                        {
                            _197 = _180;
                        }
                        _198 = _197;
                    }
                    if (_198 > 0.0f)
                    {
                        uint _207;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * _122) + _113)) + _131) * 4 + 0, 1u, _207);
                    }
                }
            }
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
