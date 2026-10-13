// ==RECONSTRUCTED== shaders/wall/generate_draw_lists.cs.hlsl | nani-cull-a
// Observation: _wall_generate_draw_lists.cs.hlsl_cs_pass1_0.bin + cs_pass2_0.bin
// (LocalSize 64,1,1 both). Two-round wall cull:
//   cs_pass1 (W21): frustum-cull each brick, keep prev-visibility bits, write
//     the survivor into draw buckets 0..3 (LOD 0/1 + pass2-flag offset) and the
//     screen-space LOD ring buckets 4..7.
//   cs_pass2 (W27): HZB-occlusion-cull the surviving bricks and append them to
//     the LOD buckets only.
// Frame constants arrive as a StructuredBuffer<FrameConstants> at t1/set1
// (NOT the usual set1/b0 UBO); brick records as StructuredBuffer<InstancedWallData>.
// RuntimeDescriptorArray (SPV_EXT_descriptor_indexing) is carried by the
// bindless-texture trigger that the original frame_constants umbrella provided.
#include "inc/common.hlsl"
#include "inc/draw_indirect_types.hlsl"
#include "wall/inc/brick_instance.hlsl"
#include "wall/inc/draw_lists.hlsl"
#include "wall/inc/brick_instance_flags.hlsl"

Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);

struct ViewConstants
{
    float4x4 view_to_clip;
    float4x4 clip_to_view;
    float4x4 view_to_sample;
    float4x4 sample_to_view;
    float4x4 world_to_view;
    float4x4 view_to_world;
    float4x4 world_to_sample;
    float4x4 clip_to_prev_clip;
    float4x4 prev_view_to_prev_clip;
    float4x4 prev_clip_to_prev_view;
    float4x4 prev_world_to_prev_view;
    float4x4 prev_view_to_prev_world;
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

struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 hzb_tex_inner_dims;
};

StructuredBuffer<FrameConstants> frame_constants_storage_buffer : register(t1, space1);
StructuredBuffer<InstancedWallData> brick_instance_data : register(t15, space0);
StructuredBuffer<uint> prev_visibility_bitmask : register(t16, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u17, space0);
RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
RWStructuredBuffer<uint> __alias_brick_draw_list : register(u18, space0);

static float3x4 load_brick_transform(uint inst)
{
    Affine3Packed affine = brick_instance_data[inst].transform;
    return transpose(float4x3(
        float3(affine.data[0], affine.data[1], affine.data[2]),
        float3(affine.data[3], affine.data[4], affine.data[5]),
        float3(affine.data[6], affine.data[7], affine.data[8]),
        float3(affine.data[9], affine.data[10], affine.data[11])));
}

// ---- Round 1: frustum cull + visibility + LOD-ring draw buckets ----
[numthreads(64, 1, 1)]
void cs_pass1(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x >= (push_constants.instance_count_and_flags & WALL_BRICK_INSTANCE_COUNT_MASK))
    {
        return;
    }
    uint inst = push_constants.first_instance + dispatchThreadID.x;

    float3x4 brick_mat = load_brick_transform(inst);
    float half_extent = 0.5f * length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);

    ViewConstants view = frame_constants_storage_buffer[0].view_constants[0];
    float4x4 world_to_view = view.world_to_view;
    float4x4 view_to_clip = view.view_to_clip;

    // Frustum planes (left/right/top/bottom) in view space from the clip matrix.
    float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
    float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
    float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
    float4 planes[4];
    planes[0] = plane_origin + plane_lr;
    planes[1] = plane_origin - plane_lr;
    planes[2] = plane_origin + plane_tb;
    planes[3] = plane_origin - plane_tb;
    [unroll]
    for (uint p = 0; p < 4u; ++p)
    {
        planes[p] /= length(planes[p].xyz).xxxx;
    }

    float4 brick_center = mul(world_to_view, float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
    float center_depth = -brick_center.z;

    bool in_frustum;
    if ((center_depth + half_extent) > view_to_clip[2].w)
    {
        in_frustum = (center_depth - half_extent) < 3.4028234663852886e+38f;
    }
    else
    {
        in_frustum = false;
    }
    [unroll]
    for (uint q = 0; q < 4u && in_frustum; ++q)
    {
        in_frustum = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (half_extent * (-0.5f));
    }

    uint bit = 1u << (inst % 32u);
    bool was_visible = in_frustum && (prev_visibility_bitmask[inst / 32u] & bit) != 0u;
    if (!was_visible)
    {
        return;
    }

    uint unused;
    InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit, unused);

    // LOD 0 / 1 by projected brick screen height (30 px threshold); bevel
    // bricks stay at LOD 0.
    uint lod;
    if ((brick_instance_data[inst].flags & WALL_BRICK_FLAG_BEVEL) != 0u)
    {
        lod = 0u;
    }
    else
    {
        float proj_brick_height = (half_extent / center_depth)
            * ((0.5f * view_to_clip[1].y) * float(clamp(frame_constants_storage_buffer[0].inner_rendering_resolution.y, 1080u, 2160u)));
        lod = uint(proj_brick_height > 30.0f ? 0 : 1);
    }
    uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & WALL_BRICK_FLAG_PASS2) != 0u));
    uint prev;
    __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + bucket) * 20u + 4u, 1u, prev);
    __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;

    // Screen-space LOD ring: append the brick to ring buckets 4..7 when it
    // falls inside the 1 / 2 / 7 pixel expansion rings around the projected
    // position.
    float4x4 ring_mat = frame_constants_storage_buffer[0].view_constants[2].world_to_sample;
    float4x4 ring_view = frame_constants_storage_buffer[0].view_constants[2].view_to_sample;
    float ring = half_extent * (2.0f * ring_view[0].x);
    float2 proj_xy = abs(mul(ring_mat, brick_center).xy);
    float2 ring_lo0 = (-ring).xx;
    float2 ring_hi0 = (1.0f + ring).xx;
    uint ring_lod = (any(bool2(proj_xy.x >= ring_lo0.x, proj_xy.y >= ring_lo0.y)) ? all(bool2(proj_xy.x <= ring_hi0.x, proj_xy.y <= ring_hi0.y)) : false) ? 0u : 3u;
    float2 ring_lo1 = (1.0f - ring).xx;
    float2 ring_hi1 = (2.0f + ring).xx;
    bool in_ring1 = any(bool2(proj_xy.x >= ring_lo1.x, proj_xy.y >= ring_lo1.y)) ? all(bool2(proj_xy.x <= ring_hi1.x, proj_xy.y <= ring_hi1.y)) : false;
    uint ring_lod_min;
    if (in_ring1)
    {
        ring_lod_min = min(ring_lod, 1u);
    }
    else
    {
        ring_lod_min = ring_lod;
    }
    uint ring_lod_max = uint(in_ring1);

    float2 ring_lo2 = (2.0f - ring).xx;
    float2 ring_hi2 = (7.0f + ring).xx;
    bool in_ring2 = any(bool2(proj_xy.x >= ring_lo2.x, proj_xy.y >= ring_lo2.y)) ? all(bool2(proj_xy.x <= ring_hi2.x, proj_xy.y <= ring_hi2.y)) : false;
    if (in_ring2)
    {
        ring_lod_max = max(ring_lod_max, 2u);
        ring_lod_min = min(ring_lod_min, 2u);
    }
    [unroll]
    for (uint r = ring_lod_min; r <= ring_lod_max; ++r)
    {
        uint ring_bucket = 4u + r;
        uint prev_ring;
        __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + ring_bucket) * 20u + 4u, 1u, prev_ring);
        __alias_brick_draw_list[wall_brick_draw_list_slot(ring_bucket, prev_ring)] = inst;
    }
}

// ---- Round 2: HZB occlusion cull of the surviving bricks ----
Texture2D<float4> hzb_tex : register(t19, space0);

[numthreads(64, 1, 1)]
void cs_pass2(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x >= (push_constants.instance_count_and_flags & WALL_BRICK_INSTANCE_COUNT_MASK))
    {
        return;
    }
    uint inst = push_constants.first_instance + dispatchThreadID.x;

    float3x4 brick_mat = load_brick_transform(inst);
    float half_extent = 0.5f * length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);

    ViewConstants view = frame_constants_storage_buffer[0].view_constants[0];
    float4x4 world_to_view = view.world_to_view;
    float4x4 view_to_clip = view.view_to_clip;

    float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
    float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
    float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
    float4 planes[4];
    planes[0] = plane_origin + plane_lr;
    planes[1] = plane_origin - plane_lr;
    planes[2] = plane_origin + plane_tb;
    planes[3] = plane_origin - plane_tb;
    [unroll]
    for (uint p = 0; p < 4u; ++p)
    {
        planes[p] /= length(planes[p].xyz).xxxx;
    }

    float4 brick_center = mul(world_to_view, float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
    float center_depth = -brick_center.z;

    bool in_frustum;
    if ((center_depth + half_extent) > view_to_clip[2].w)
    {
        in_frustum = (center_depth - half_extent) < 3.4028234663852886e+38f;
    }
    else
    {
        in_frustum = false;
    }
    [unroll]
    for (uint q = 0; q < 4u && in_frustum; ++q)
    {
        in_frustum = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (half_extent * (-0.5f));
    }

    // 8 brick corners -> screen-space AABB -> HZB mip -> depth compare.
    float4x4 mvp = frame_constants_storage_buffer[0].view_constants[0].world_to_sample;
    float4 axis_x = mul(mvp, float4(mul(brick_mat, float4(1.0f, 0.0f, 0.0f, 0.0f)), 0.0f));
    float4 axis_y = mul(mvp, float4(mul(brick_mat, float4(0.0f, 1.0f, 0.0f, 0.0f)), 0.0f));
    float4 axis_z = mul(mvp, float4(mul(brick_mat, float4(0.0f, 0.0f, 1.0f, 0.0f)), 0.0f));
    float4 corner0 = mul(mvp, float4(mul(brick_mat, float4(-0.5f, -0.5f, -0.5f, 1.0f)), 1.0f));

    float4 corners[8];
    corners[0] = corner0;
    corners[1] = corner0 + axis_z;
    corners[2] = corner0 + axis_y;
    corners[3] = corner0 + axis_y + axis_z;
    corners[4] = corner0 + axis_x;
    corners[5] = corner0 + axis_x + axis_z;
    corners[6] = corner0 + axis_x + axis_y;
    corners[7] = corner0 + axis_x + axis_y + axis_z;

    float4 bounds;
    float max_depth;
    bool has_bounds;
    if (in_frustum)
    {
        float w[8];
        [unroll]
        for (uint c = 0; c < 8u; ++c)
        {
            w[c] = corners[c].w;
        }
        if (max(max(max(w[0], w[1]), max(w[2], w[3])), max(max(w[4], w[5]), max(w[6], w[7]))) < view_to_clip[2].w)
        {
            bounds = 0.0f.xxxx;
            max_depth = 0.0f;
            has_bounds = false;
        }
        else
        {
            float2 ndc_min = min(min(min(corners[0].xy / w[0].xx, corners[1].xy / w[1].xx), min(corners[2].xy / w[2].xx, corners[3].xy / w[3].xx)), min(min(corners[4].xy / w[4].xx, corners[5].xy / w[5].xx), min(corners[6].xy / w[6].xx, corners[7].xy / w[7].xx)));
            float2 ndc_max = max(max(max(corners[0].xy / w[0].xx, corners[1].xy / w[1].xx), max(corners[2].xy / w[2].xx, corners[3].xy / w[3].xx)), max(max(corners[4].xy / w[4].xx, corners[5].xy / w[5].xx), max(corners[6].xy / w[6].xx, corners[7].xy / w[7].xx)));
            bounds = (float4(ndc_min.x, ndc_min.y, ndc_max.x, ndc_max.y).xwzy * float4(0.5f, -0.5f, 0.5f, -0.5f)) + 0.5f.xxxx;
            max_depth = -min(min(min(w[0], w[1]), min(w[2], w[3])), min(min(w[4], w[5]), min(w[6], w[7])));
            has_bounds = true;
        }
    }
    else
    {
        bounds = 0.0f.xxxx;
        max_depth = 0.0f;
        has_bounds = false;
    }

    float4 hzb_dims = float4(hzb_tex_inner_dims.xyxy);
    float4 uv = clamp(bounds * hzb_dims, 0.0f.xxxx, hzb_dims - 1.0f.xxxx);
    uint mip = uint(max(0.0f, ceil(log2(max(uv.z - uv.x, uv.w - uv.y)))));

    bool occluded;
    if (has_bounds)
    {
        uint2 mip_mask = mip.xx & uint2(31u, 31u);
        uint2 uv_min = uint2(uv.xy) >> mip_mask;
        uint2 uv_max = uint2(uv.zw) >> mip_mask;
        float4x4 clip_to_view = frame_constants_storage_buffer[0].view_constants[0].clip_to_view;
        float hzb_depth = min(min(hzb_tex.Load(int3(uv_min, mip)).x, hzb_tex.Load(int3(uint2(uv_min.x, uv_max.y), mip)).x), min(hzb_tex.Load(int3(uint2(uv_max.x, uv_min.y), mip)).x, hzb_tex.Load(int3(uv_max, mip)).x));
        occluded = max_depth > (1.0f / (hzb_depth * (-clip_to_view[3].z)));
    }
    else
    {
        occluded = has_bounds;
    }
    if (!occluded)
    {
        return;
    }

    uint bit = 1u << (inst % 32u);
    uint unused;
    InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit, unused);

    if ((prev_visibility_bitmask[inst / 32u] & bit) == 0u)
    {
        uint lod;
        if ((brick_instance_data[inst].flags & WALL_BRICK_FLAG_BEVEL) != 0u)
        {
            lod = 0u;
        }
        else
        {
            float proj_brick_height = (half_extent / center_depth)
                * ((0.5f * view_to_clip[1].y) * float(clamp(frame_constants_storage_buffer[0].inner_rendering_resolution.y, 1080u, 2160u)));
            lod = uint(proj_brick_height > 30.0f ? 0 : 1);
        }
        uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & WALL_BRICK_FLAG_PASS2) != 0u));
        uint prev;
        __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + bucket) * 20u + 4u, 1u, prev);
        __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;
    }
}
