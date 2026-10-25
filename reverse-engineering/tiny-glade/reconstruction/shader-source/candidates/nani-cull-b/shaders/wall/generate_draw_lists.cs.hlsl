// ==RECONSTRUCTED== shaders/wall/generate_draw_lists.cs.hlsl | evidence: raw disasm of
// _wall_generate_draw_lists.cs.hlsl_cs_pass1_0.bin / cs_pass2_0.bin: per brick instance, build the
// wall-space transform, frustum (+ HZB in pass2) cull, set next-visibility bit, and append the
// instance to a LOD-tagged draw command with the instanceCount bumped; push first_instance /
// instance_count_and_flags | status: O (bindings/entries/LocalSize, algorithm); I (frustum/LOD
// constant details; frame_constants_storage_buffer read path)

#line 1 "shaders/wall/generate_draw_lists.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/frustum.hlsl"
#include "inc/projection.hlsl"
#include "inc/math.hlsl"
#include "inc/cascades.hlsl"
#include "inc/lod.hlsl"
#include "inc/brick_instance.hlsl"
#include "inc/draw_lists.hlsl"
#include "inc/brick_instance_flags.hlsl"

// FrameConstants storage-buffer element (same layout as the frame_constants UBO; the wall
// chain reads it as a StructuredBuffer at set1/b1).
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

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 hzb_tex_inner_dims;
};

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);
StructuredBuffer<InstancedWallData> brick_instance_data : register(t15, space0);
StructuredBuffer<uint> prev_visibility_bitmask : register(t16, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u17, space0);
RWStructuredBuffer<uint> __alias_brick_draw_list : register(u18, space0);
Texture2D<float> hzb_tex : register(t19, space0);

struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_pass1(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x >= (push_constants.instance_count_and_flags & 16777215u))
    {
        return;
    }

    uint instance_index = push_constants.first_instance + thread_id.x;
    InstancedWallData data = brick_instance_data[instance_index];

    float4x4 m;
    m[0] = float4(data.transform.data[0], data.transform.data[3], data.transform.data[6], data.transform.data[9]);
    m[1] = float4(data.transform.data[1], data.transform.data[4], data.transform.data[7], data.transform.data[10]);
    m[2] = float4(data.transform.data[2], data.transform.data[5], data.transform.data[8], data.transform.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);

    float3 basis_x = abs(m[0].xyz);
    float3 basis_y = abs(m[1].xyz);
    float3 basis_z = abs(m[2].xyz);
    float extent = length(basis_x + basis_y + basis_z);
    float radius = 0.5f * extent;
    float3 center = m[3].xyz;

    float4 view_pos = mul(float4(center, 1.0f), frame_constants_storage_buffer[0].view_constants[0].world_to_view);
    float view_z = view_pos.z;

    // Visibility bitmask.
    uint bit_index = instance_index / 32u;
    uint bit_mask = 1u << (instance_index % 32u);
    InterlockedOr(__alias_next_visibility_bitmask[bit_index], bit_mask);

    if ((prev_visibility_bitmask[bit_index] & bit_mask) != 0u)
    {
        // LOD selection by projected size (approximated).
        float half_projected = 0.5f * (radius / max(-view_z, 1e-5f)) * frame_constants_storage_buffer[0].view_constants[0].view_to_clip[1][1];
        uint lod = (half_projected > 30.0f) ? 0u : 1u;
        uint draw_cmd_index = uint(draw_indirect_buffer_offset) + lod + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));

        uint slot;
        InterlockedAdd(__alias_draw_commands[draw_cmd_index].instanceCount, 1u, slot);
        __alias_brick_draw_list[(draw_cmd_index * 2097152u) + slot] = instance_index;
    }
}

[numthreads(64, 1, 1)]
void cs_pass2(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x >= (push_constants.instance_count_and_flags & 16777215u))
    {
        return;
    }

    uint instance_index = push_constants.first_instance + thread_id.x;
    InstancedWallData data = brick_instance_data[instance_index];

    float4x4 m;
    m[0] = float4(data.transform.data[0], data.transform.data[3], data.transform.data[6], data.transform.data[9]);
    m[1] = float4(data.transform.data[1], data.transform.data[4], data.transform.data[7], data.transform.data[10]);
    m[2] = float4(data.transform.data[2], data.transform.data[5], data.transform.data[8], data.transform.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);

    float3 basis_x = abs(m[0].xyz);
    float3 basis_y = abs(m[1].xyz);
    float3 basis_z = abs(m[2].xyz);
    float extent = length(basis_x + basis_y + basis_z);
    float radius = 0.5f * extent;
    float3 center = m[3].xyz;

    float4 view_pos = mul(float4(center, 1.0f), frame_constants_storage_buffer[0].view_constants[0].world_to_view);
    float view_z = view_pos.z;

    Frustum f = extract_frustum(frame_constants_storage_buffer[0].view_constants[0].world_to_view);
    bool visible = frustum_intersects_sphere(f, view_pos.xyz, radius);

    if (visible)
    {
        // HZB 2x2 sample (approximated screen extent).
        float4 clip_pos = mul(view_pos, frame_constants_storage_buffer[0].view_constants[0].view_to_clip);
        float2 ndc = clip_pos.xy / clip_pos.w;
        float2 screen = (ndc * 0.5f + 0.5f) * float2(hzb_tex_inner_dims);
        float2 screen_radius = (radius / max(view_z, 1e-5f)) * float2(hzb_tex_inner_dims);
        float mip = max(0.0f, ceil(log2(max(screen_radius.x, screen_radius.y))));
        uint mip_u = uint(mip);
        uint2 coord = min(uint2(screen) >> (uint2(mip_u, mip_u) & 31u), hzb_tex_inner_dims - 1u);
        float h0 = hzb_tex.Load(uint3(coord, mip_u));
        float h1 = hzb_tex.Load(uint3(coord + uint2(1u, 0u), mip_u));
        float h2 = hzb_tex.Load(uint3(coord + uint2(0u, 1u), mip_u));
        float h3 = hzb_tex.Load(uint3(coord + uint2(1u, 1u), mip_u));
        float hzb_min = min(min(h0, h1), min(h2, h3));
        visible = (view_z + radius) > (1.0f / (hzb_min * -frame_constants_storage_buffer[0].view_constants[0].clip_to_view[3][2]));
    }

    if (visible)
    {
        uint bit_index = instance_index / 32u;
        uint bit_mask = 1u << (instance_index % 32u);
        InterlockedOr(__alias_next_visibility_bitmask[bit_index], bit_mask);

        if ((prev_visibility_bitmask[bit_index] & bit_mask) != 0u)
        {
            float half_projected = 0.5f * (radius / max(-view_z, 1e-5f)) * frame_constants_storage_buffer[0].view_constants[0].view_to_clip[1][1];
            uint lod = (half_projected > 30.0f) ? 0u : 1u;
            uint draw_cmd_index = uint(draw_indirect_buffer_offset) + lod + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));

            uint slot;
            InterlockedAdd(__alias_draw_commands[draw_cmd_index].instanceCount, 1u, slot);
            __alias_brick_draw_list[(draw_cmd_index * 2097152u) + slot] = instance_index;
        }
    }
}
