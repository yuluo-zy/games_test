// PATCHED: modules ty.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/roof_tiles_gravity.cs.hlsl | F-V | bin: _roof_tiles_gravity.cs.hlsl_cs_main_0.bin | entry cs:cs_main | LocalSize 64,1,1
// Roof tile gravity/placement simulation. Each thread updates one RoofShingle record
// (in RWStructuredBuffer<RoofShingle> __alias_instanced_data at set0/u13): it computes a
// downward velocity from the packed half2x16 `velocity_rotation`, integrates height with the
// SSBO frame delta_time (frame_constants_storage_buffer, set1/t1), clamps tiles to the roof
// plane (push_constants.roof_pivot_y) and to a max fall speed, eases them to the plane with a per-seed
// delay, and writes back the record with a new packed velocity/rotation.
// Note: the shader reads frame constants from an SSBO (not the cbuffer); inc/frame_constants.hlsl
// is still included (it appears in the embedded include list) but only for ViewConstants.

#include "roof/roof_tile.inc.hlsl"
#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"

// Frame constants as an SSBO (delta_time_seconds at byte 132).
struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);

RWStructuredBuffer<RoofShingle> __alias_instanced_data : register(u13);

// RoofPushConstants (push_constants.first_instance, push_constants.instance_count, push_constants.wall_height_delta, push_constants.roof_pivot_y,
// push_constants.animation_t) is defined in roof/roof_tile.inc.hlsl.

[[vk::push_constant]]
ConstantBuffer<RoofPushConstants> push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= push_constants.instance_count)
    {
        return;
    }

    uint index = push_constants.first_instance + gl_GlobalInvocationID.x;
    RoofShingle s = __alias_instanced_data[index];

    bool wall_clip;
    if (push_constants.wall_height_delta < 0.0f)
    {
        wall_clip = (push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) > -120.0f;
    }
    else
    {
        wall_clip = false;
    }

    bool below_roof;
    if (wall_clip)
    {
        below_roof = s.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
    }
    else
    {
        below_roof = false;
    }

    float t = floor(unpack_half2x16((s.scale_t.y >> 16u) & 0xFFFFu).x * 50.0f) * 0.02f;
    uint old_velocity = s.velocity_rotation & 0xFFFFu;
    float velocity = unpack_half2x16(old_velocity).x
                   + (-98.1f * lerp(1.0f, 0.4f, t)) * frame_constants_storage_buffer[0].delta_time_seconds;
    float new_height = (below_roof ? push_constants.roof_pivot_y : s.current_height)
                     + velocity * frame_constants_storage_buffer[0].delta_time_seconds;

    float clamped_velocity;
    float height;
    if (new_height <= push_constants.roof_pivot_y)
    {
        clamped_velocity = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer[0].delta_time_seconds) * 0.166f, velocity));
        height = max(new_height, push_constants.roof_pivot_y);
    }
    else
    {
        clamped_velocity = velocity;
        height = new_height;
    }

    float eased_height;
    if (t > 0.9f)
    {
        eased_height = push_constants.roof_pivot_y;
    }
    else
    {
        float settle = 1.0f - ((t * t) * (t * t));
        float delay = 1.0f - exp(-frame_constants_storage_buffer[0].delta_time_seconds
                                * (5.0f * lerp(0.0f, 2.0f, settle))
                                * ((rand01(hash(uint(s.seed))) * t > 0.3f) ? 1.0f : 1000.0f));
        eased_height = lerp(height, push_constants.roof_pivot_y, delay);
    }

    float delta = abs(push_constants.roof_pivot_y - eased_height);
    float rot;
    if (clamped_velocity > 0.0f)
    {
        rot = (1.0f - exp(-((delta * delta) * delta))) * -1.57f;
    }
    else
    {
        rot = 0.0f;
    }

    uint new_velocity_rotation = ((old_velocity | (pack_half2x16(float2(rot, 0.0f)) << 16u)) & 0xFFF00000u)
                               | pack_half2x16(float2(clamped_velocity, 0.0f));

    RoofShingle out_s;
    out_s.base_position = s.base_position;
    out_s.current_height = eased_height;
    out_s.clipping_plane_ = s.clipping_plane_;
    out_s.scale_t = s.scale_t;
    out_s.rotation_quat_ = s.rotation_quat_;
    out_s.roof_id = s.roof_id;
    out_s.velocity_rotation = new_velocity_rotation;
    out_s.animation_t = push_constants.animation_t;
    out_s.seed = s.seed;
    out_s.is_circle = s.is_circle;
    out_s.is_main_roof_body = s.is_main_roof_body;
    __alias_instanced_data[index] = out_s;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _roof_tiles_gravity.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
RWByteAddressBuffer __alias_instanced_data : register(u13, space0);
struct PushConstants
{
    uint first_instance;
    uint instance_count;
    float wall_height_delta;
    float roof_pivot_y;
    float animation_t;
};
[[vk::push_constant]]
PushConstants push_constants;


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
        if (gl_GlobalInvocationID.x >= push_constants.instance_count)
        {
            break;
        }
        uint _97 = push_constants.first_instance + gl_GlobalInvocationID.x;
        RoofShingle _105;
        _105.base_position = __alias_instanced_data.Load<float3>(_97 * 64 + 0);
        _105.current_height = __alias_instanced_data.Load<float>(_97 * 64 + 12);
        _105.clipping_plane_ = __alias_instanced_data.Load<uint2>(_97 * 64 + 16);
        _105.scale_t = __alias_instanced_data.Load<uint2>(_97 * 64 + 24);
        _105.rotation_quat_ = __alias_instanced_data.Load<uint2>(_97 * 64 + 32);
        _105.roof_id = __alias_instanced_data.Load<uint>(_97 * 64 + 40);
        _105.velocity_rotation = __alias_instanced_data.Load<uint>(_97 * 64 + 44);
        _105.animation_t = __alias_instanced_data.Load<float>(_97 * 64 + 48);
        _105.seed = __alias_instanced_data.Load<int>(_97 * 64 + 52);
        _105.is_circle = __alias_instanced_data.Load<int>(_97 * 64 + 56);
        _105.is_main_roof_body = __alias_instanced_data.Load<int>(_97 * 64 + 60);
        bool _121;
        if (push_constants.wall_height_delta < 0.0f)
        {
            _121 = (push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) > (-120.0f);
        }
        else
        {
            _121 = false;
        }
        bool _126;
        if (_121)
        {
            _126 = _105.current_height <= (push_constants.roof_pivot_y - push_constants.wall_height_delta);
        }
        else
        {
            _126 = false;
        }
        float _135 = floor(spvUnpackHalf2x16((_105.scale_t.y >> 16u) & 65535u).x * 50.0f) * 0.0199999995529651641845703125f;
        uint _138 = _105.velocity_rotation & 65535u;
        float _142 = spvUnpackHalf2x16(_138).x + (((-98.09999847412109375f) * lerp(1.0f, 0.4000000059604644775390625f, _135)) * frame_constants_storage_buffer.Load<float>(132));
        float _144 = (_126 ? push_constants.roof_pivot_y : _105.current_height) + (_142 * frame_constants_storage_buffer.Load<float>(132));
        float _153;
        float _154;
        if (_144 <= push_constants.roof_pivot_y)
        {
            _153 = min(15.0f, max((push_constants.wall_height_delta / frame_constants_storage_buffer.Load<float>(132)) * 0.16599999368190765380859375f, _142));
            _154 = max(_144, push_constants.roof_pivot_y);
        }
        else
        {
            _153 = _142;
            _154 = _144;
        }
        uint _155 = uint(_105.seed);
        uint _157 = _155 + (_155 << 10u);
        uint _159 = _157 ^ (_157 >> 6u);
        uint _161 = _159 + (_159 << 3u);
        uint _163 = _161 ^ (_161 >> 11u);
        float _189;
        if (_135 > 0.89999997615814208984375f)
        {
            _189 = push_constants.roof_pivot_y;
        }
        else
        {
            _189 = lerp(_154, push_constants.roof_pivot_y, 1.0f - exp((-frame_constants_storage_buffer.Load<float>(132)) * ((5.0f * lerp(0.0f, 2.0f, 1.0f - (((_135 * _135) * _135) * _135))) * ((((asfloat(((_163 + (_163 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _135) > 0.300000011920928955078125f) ? 1.0f : 1000.0f))));
        }
        float _191 = abs(push_constants.roof_pivot_y - _189);
        float _201;
        if (_153 > 0.0f)
        {
            _201 = (1.0f - exp(((-_191) * _191) * _191)) * (-1.57000005245208740234375f);
        }
        else
        {
            _201 = 0.0f;
        }
        RoofShingle _212 = { _105.base_position, _189, _105.clipping_plane_, _105.scale_t, _105.rotation_quat_, _105.roof_id, ((_138 | (spvPackHalf2x16(float2(_201, 0.0f)) << 16u)) & 4294901760u) | spvPackHalf2x16(float2(_153, 0.0f)), push_constants.animation_t, _105.seed, _105.is_circle, _105.is_main_roof_body };
        __alias_instanced_data.Store<float3>(_97 * 64 + 0, _212.base_position);
        __alias_instanced_data.Store<float>(_97 * 64 + 12, _212.current_height);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 16, _212.clipping_plane_);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 24, _212.scale_t);
        __alias_instanced_data.Store<uint2>(_97 * 64 + 32, _212.rotation_quat_);
        __alias_instanced_data.Store<uint>(_97 * 64 + 40, _212.roof_id);
        __alias_instanced_data.Store<uint>(_97 * 64 + 44, _212.velocity_rotation);
        __alias_instanced_data.Store<float>(_97 * 64 + 48, _212.animation_t);
        __alias_instanced_data.Store<int>(_97 * 64 + 52, _212.seed);
        __alias_instanced_data.Store<int>(_97 * 64 + 56, _212.is_circle);
        __alias_instanced_data.Store<int>(_97 * 64 + 60, _212.is_main_roof_body);
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
