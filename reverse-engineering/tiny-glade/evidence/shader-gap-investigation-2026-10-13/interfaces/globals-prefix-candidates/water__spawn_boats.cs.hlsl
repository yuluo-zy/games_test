// PATCHED: modules ts.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/spawn_boats.cs.hlsl | evidence: spirv-cross + raw disasm of _water_spawn_boats.cs.hlsl_cs_main_0.bin (groupshared best_choice: per-thread score from water-depth min across a boat-sized patch + bow-sample gate + path/building clearance, packed as uint(score*0xFFFFFF00)|localIndex, InterlockedMax; winner writes BoatXform = T(world)*S(0.85)*R(axis normalize(1,0,1), bob)*R(axis normalize(0,1,0), yaw)) | status: O (algorithm from SPIR-V); I (boat size constants 2.2/1.1/0.2 world units, bob amplitude, -0.68 draft)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/perlin.hlsl"

struct BoatXform
{
    Affine3Packed transform;
};

// Rodrigues rotation around a runtime (normalized) axis — transposed row-major
// 4x4. Axis is passed normalized so the full 9-term matrix is emitted.
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

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 buildings_mask_dims;
    uint2 grid_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);
RWStructuredBuffer<BoatXform> __alias_instance_buffer : register(u18, space0);

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array decl
// the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

groupshared uint best_choice;

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    if (local_index == 0u)
        best_choice = 0u;
    GroupMemoryBarrierWithGroupSync();

    float2 uv = (float2(id.xy) + 0.5f) / float2(grid_dims);
    float2 world_pos = (uv - 0.5f) * 130.0f;

    float angle = 40.665630340576171875f * float(id.x + id.y * grid_dims.x);
    float2 dir = float2(cos(angle), sin(angle));

    float2 fwd = dir * 0.016923077404499053955078125f;
    float2 side = float2(dir.y, -dir.x) * 0.003076923079788684844970703125f;
    float2 bow = uv + dir * 0.03384615480899810791015625f;

    float min_water = min(min(min(min(min(
        water_mask.SampleLevel(sampler_lnc, uv, 0.0f).x,
        water_mask.SampleLevel(sampler_lnc, uv - fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd + side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv + fwd - side, 0.0f).x),
        water_mask.SampleLevel(sampler_lnc, uv - fwd * 1.5f, 0.0f).x);

    bool deep = water_mask.SampleLevel(sampler_lnc, bow, 0.0f).x < 0.5f ? (min_water > 0.5f) : false;

    float deep_score = deep ? min_water : 0.0f;
    float score = (abs(water_mask.SampleLevel(sampler_lnc, uv + fwd, 0.0f).w - water_mask.SampleLevel(sampler_lnc, uv - fwd, 0.0f).w) > 0.1f)
        ? 0.0f
        : ((path_mask.Load(int3(uint2(int2(uv * float2(path_mask_dims))), 0u)).x > 0.0f)
            ? 0.0f
            : deep_score * clamp(buildings_mask.SampleLevel(sampler_lnc, uv + dir * 0.06769230841702222824096679688f, 0.0f).y - 0.3f, 0.0f, 1.0f));

    uint packed = uint(clamp(score, 0.0f, 1.0f) * 4294967040.0f) | local_index;
    uint prev;
    InterlockedMax(best_choice, packed, prev);
    GroupMemoryBarrierWithGroupSync();

    bool is_winner = best_choice == packed && score > 0.0f;

    if (is_winner)
    {
        uint seed = id.x * id.y + id.x;
        float rnd = rand01(hash(seed));
        float bob = 0.05f * sin((fmod(float(game_time_sec), TWO_PI) + game_time_frac) + TWO_PI * rnd);
        float yaw = cos(angle);

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 scale = float4x4(
            float4(0.85f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 0.85f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 0.85f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));
        float4x4 m = mul(mul(mul(
            affine_translation(float3(world_pos.x, -0.68f, world_pos.y)),
            mul(identity, scale)),
            rotate_axis(normalize(normalize(float3(1.0f, 0.0f, 1.0f))), bob)),
            rotate_axis(normalize(float3(0.0f, 1.0f, 0.0f)), yaw));

        uint base;
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, base);
        __alias_instance_buffer[base] = BoatXform(pack_affine(m));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_spawn_boats.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct BoatXform
{
    Affine3Packed transform;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 path_mask_dims : packoffset(c0.y);
    uint2 water_mask_dims : packoffset(c1);
    uint2 buildings_mask_dims : packoffset(c1.z);
    uint2 grid_dims : packoffset(c2);
};

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

RWByteAddressBuffer __alias_instance_buffer : register(u18, space0);
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> water_mask : register(t16, space0);
Texture2D<float4> buildings_mask : register(t17, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint best_choice;

void comp_main()
{
    if (0u == gl_LocalInvocationIndex)
    {
        best_choice = 0u;
    }
    GroupMemoryBarrierWithGroupSync();
    float2 _135 = (float2(gl_GlobalInvocationID.xy) + 0.5f.xx) / float2(grid_dims);
    float2 _137 = (_135 - 0.5f.xx) * 130.0f.xx;
    float _150 = 40.665630340576171875f * float(gl_GlobalInvocationID.x + (gl_GlobalInvocationID.y * grid_dims.x));
    float _151 = cos(_150);
    float _152 = sin(_150);
    float2 _153 = float2(_151, _152);
    float2 _158 = _153 * 0.03384615480899810791015625f;
    float2 _167 = _135 + (_153 * (-0.016923077404499053955078125f));
    float2 _168 = float2(_152, -_151) * 0.003076923079788684844970703125f;
    float2 _174 = _135 + (_153 * 0.016923077404499053955078125f);
    float _197 = min(min(min(min(min(water_mask.SampleLevel(sampler_lnc, _135, 0.0f).x, water_mask.SampleLevel(sampler_lnc, _167 + _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _174 + _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _167 - _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _174 - _168, 0.0f).x), water_mask.SampleLevel(sampler_lnc, _135 + ((_153 * (-1.5f)) * 0.016923077404499053955078125f), 0.0f).x);
    bool _202;
    if (water_mask.SampleLevel(sampler_lnc, _135 + _158, 0.0f).x < 0.5f)
    {
        _202 = _197 > 0.5f;
    }
    else
    {
        _202 = false;
    }
    float _203 = _202 ? _197 : 0.0f;
    float _228 = (abs(water_mask.SampleLevel(sampler_lnc, _174, 0.0f).w - water_mask.SampleLevel(sampler_lnc, _167, 0.0f).w) > 0.100000001490116119384765625f) ? 0.0f : ((path_mask.Load(int3(uint2(int2(_135 * float2(path_mask_dims))), 0u)).x > 0.0f) ? 0.0f : (_203 * clamp(buildings_mask.SampleLevel(sampler_lnc, _135 + (_158 * 2.0f), 0.0f).y - 0.300000011920928955078125f, 0.0f, 1.0f)));
    uint _232 = uint(clamp(_228, 0.0f, 1.0f) * 4294967040.0f) | gl_LocalInvocationIndex;
    uint _233;
    InterlockedMax(best_choice, _232, _233);
    GroupMemoryBarrierWithGroupSync();
    bool _239;
    if (best_choice == _232)
    {
        _239 = _228 > 0.0f;
    }
    else
    {
        _239 = false;
    }
    if (_239)
    {
        uint _258 = (gl_GlobalInvocationID.x * gl_GlobalInvocationID.y) + gl_GlobalInvocationID.x;
        uint _260 = _258 + (_258 << 10u);
        uint _262 = _260 ^ (_260 >> 6u);
        uint _264 = _262 + (_262 << 3u);
        uint _266 = _264 ^ (_264 >> 11u);
        float _276 = 0.0500000007450580596923828125f * sin((fmod(float(frame_constants_game_time_sec), 6.283185482025146484375f) + frame_constants_game_time_frac) + (6.283185482025146484375f * (asfloat(((_266 + (_266 << 15u)) & 8388607u) | 1065353216u) - 1.0f)));
        float3 _277 = normalize(normalize(float3(1.0f, 0.0f, 1.0f)));
        float _278 = sin(_276);
        float _279 = cos(_276);
        float _280 = 1.0f - _279;
        float _281 = _277.x;
        float _282 = _280 * _281;
        float _285 = _277.y;
        float _286 = _282 * _285;
        float _287 = _277.z;
        float _288 = _287 * _278;
        float _290 = _280 * _287;
        float _291 = _290 * _281;
        float _292 = _285 * _278;
        float _295 = _280 * _285;
        float _298 = _295 * _287;
        float _299 = _281 * _278;
        float3 _311 = normalize(float3(0.0f, 1.0f, 0.0f));
        float _312 = 1.0f - _151;
        float _313 = _311.x;
        float _314 = _312 * _313;
        float _317 = _311.y;
        float _318 = _314 * _317;
        float _319 = _311.z;
        float _320 = _319 * _152;
        float _322 = _312 * _319;
        float _323 = _322 * _313;
        float _324 = _317 * _152;
        float _327 = _312 * _317;
        float _330 = _327 * _319;
        float _331 = _313 * _152;
        float4x4 _342 = mul(mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, _137.x), float4(0.0f, 1.0f, 0.0f, -0.680000007152557373046875f), float4(0.0f, 0.0f, 1.0f, _137.y), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(float4x4(float4(1.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 1.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4x4(float4(0.85000002384185791015625f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.85000002384185791015625f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.85000002384185791015625f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_282 * _281) + _279, _286 - _288, _291 + _292, 0.0f), float4(_286 + _288, (_295 * _285) + _279, _298 - _299, 0.0f), float4(_291 - _292, _298 + _299, (_290 * _287) + _279, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), transpose(float4x4(float4((_314 * _313) + _151, _318 - _320, _323 + _324, 0.0f), float4(_318 + _320, (_327 * _317) + _151, _330 - _331, 0.0f), float4(_323 - _324, _330 + _331, (_322 * _319) + _151, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))));
        uint _359;
        __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, _359);
        uint _361 = uint(int(_359));
        float _363[12] = { _342[0].x, _342[1].x, _342[2].x, _342[0].y, _342[1].y, _342[2].y, _342[0].z, _342[1].z, _342[2].z, _342[0].w, _342[1].w, _342[2].w };
        Affine3Packed _364 = { _363 };
        BoatXform _365 = { _364 };
        [unroll]
        for (int _2ident = 0; _2ident < 12; _2ident++)
        {
            __alias_instance_buffer.Store<float>(_2ident * 4 + _361 * 48 + 0, _365.transform.data[_2ident]);
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
