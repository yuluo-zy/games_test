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
