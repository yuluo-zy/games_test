// PATCHED: modules s.cs.hlsl_cs_pass1_0.bin:cs_pass1, s.cs.hlsl_cs_pass2_0.bin:cs_pass2 are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_cs_pass1_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_cs_pass2_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/wall/generate_draw_lists.cs.hlsl | evidence: raw disasm of
// _wall_generate_draw_lists.cs.hlsl_cs_pass1_0.bin / cs_pass2_0.bin (LocalSize 64,1,1 both):
// per brick instance, build the wall-space transform, frustum (+ HZB in pass2) cull, set
// next-visibility bit, and append the instance to a LOD-tagged draw command with the
// instanceCount bumped; push first_instance / instance_count_and_flags | status:
// O (bindings/entries/LocalSize, algorithm); I (frustum/LOD/HZB constant details;
// frame_constants_storage_buffer read path; helper bodies)
//
// MERGE (wave2 nani-cull-merge, per evidence/render-passes/review/nani-cull/review.md):
//   base = candidate B binding shell (RWStructuredBuffer draw_commands u13,
//   frame_constants_storage_buffer t1/s1, $Globals b14) + candidate A body
//   (cs_pass1 4-plane frustum loop + LOD buckets 0..3 + screen-space LOD ring buckets 4..7;
//   cs_pass2 8-corner AABB -> HZB mip -> 4x OpImageFetch depth test) + bin-corrected gating.
//   cs_pass1 gating (bin): LOD-bucket append + InterlockedOr run ONLY when prev-visible AND
//   in-frustum; the screen-space ring-bucket loop runs UNCONDITIONALLY (bin reaches the ring
//   code from both branches; no early-return) — A's `if (!was_visible) return;` was dropped.
//   inc/frame_constants.hlsl IS included (bin embeds it; cascades.hlsl needs the global
//   view_constants to compile). Verified: with installed dxc the unused set1/b0 UBO + space2/t0
//   trigger are DCE'd, so no set1/b0 binding is emitted and the caps stay {Shader,
//   RuntimeDescriptorArray} (see merge-log / verify.json) — this supersedes adjudicated inc-fix
//   #4's stated "would add the set1/b0 UBO" rationale, while FrameConstants is still redeclared
//   locally for the StructuredBuffer<FrameConstants> element at set1/b1.
//   WALL_BRICK_* flag macros + wall_brick_draw_list_slot are inlined here from A's wall/inc
//   bodies (B's wall/inc files stay closure-only; see merge-log).

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
#include "wall/inc/brick_instance.hlsl"
#include "wall/inc/draw_lists.hlsl"
#include "wall/inc/brick_instance_flags.hlsl"

// FrameConstants storage-buffer element (same layout as the frame_constants UBO; the wall
// chain reads it as a StructuredBuffer at set1/b1). ViewConstants comes from
// inc/frame_constants.hlsl; FrameConstants is redeclared locally per adjudicated inc-fix #4.
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

// Brick flag decode (inlined from candidate A's wall/inc/brick_instance_flags.hlsl). Kept as
// #defines (not file-scope static consts) so they never spill into the implicit $Globals cbuffer,
// whose wall-family layout is exactly { int draw_indirect_buffer_offset; uint2 hzb_tex_inner_dims; }.
#define WALL_BRICK_FLAG_BEVEL 0x80u
#define WALL_BRICK_FLAG_PASS2 0x80000000u
#define WALL_BRICK_INSTANCE_COUNT_MASK 0xFFFFFFu
#define WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY 0x200000u

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

// Brick draw-list slot: bucket = draw-command index (0..3 pass1, 4..7 LOD rings), capacity per
// bucket 0x200000 bricks (inlined from candidate A's wall/inc/draw_lists.hlsl).
uint wall_brick_draw_list_slot(uint bucket, uint prev)
{
    return bucket * WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY + prev;
}

// Packed affine -> 3x4 column-major brick transform (inlined from candidate A).
static float3x4 load_brick_transform(uint inst)
{
    Affine3Packed affine = brick_instance_data[inst].transform;
    return transpose(float4x3(
        float3(affine.data[0], affine.data[1], affine.data[2]),
        float3(affine.data[3], affine.data[4], affine.data[5]),
        float3(affine.data[6], affine.data[7], affine.data[8]),
        float3(affine.data[9], affine.data[10], affine.data[11])));
}

struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};

[[vk::push_constant]]
PushConstants push_constants;

// ---- Round 1: frustum cull + visibility + LOD / ring draw buckets ----
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

    // LOD-bucket append + next-visibility OR: only when prev-visible AND in-frustum (bin gating).
    if (in_frustum && (prev_visibility_bitmask[inst / 32u] & bit) != 0u)
    {
        InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit);

        // LOD 0 / 1 by projected brick screen height (30 px threshold); bevel bricks stay at LOD 0.
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
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + bucket].instanceCount, 1u, prev);
        __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;
    }

    // Screen-space LOD ring: append the brick to ring buckets 4..7 when it falls inside the
    // 1 / 2 / 7 pixel expansion rings around the projected position. Runs UNCONDITIONALLY
    // (the bin reaches this loop from both the visible and non-visible branches; no early-return).
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
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + ring_bucket].instanceCount, 1u, prev_ring);
        __alias_brick_draw_list[wall_brick_draw_list_slot(ring_bucket, prev_ring)] = inst;
    }
}

// ---- Round 2: HZB occlusion cull of the surviving bricks (8-corner AABB) ----
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
    InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit);

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
        InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset) + bucket].instanceCount, 1u, prev);
        __alias_brick_draw_list[wall_brick_draw_list_slot(bucket, prev)] = inst;
    }
}

#endif

#if defined(TGV_cs_pass1_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _wall_generate_draw_lists.cs.hlsl_cs_pass1_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 hzb_tex_inner_dims : packoffset(c0.y);
};

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
ByteAddressBuffer brick_instance_data : register(t15, space0);
ByteAddressBuffer prev_visibility_bitmask : register(t16, space0);
RWByteAddressBuffer __alias_next_visibility_bitmask : register(u17, space0);
RWByteAddressBuffer __alias_brick_draw_list : register(u18, space0);
struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};
[[vk::push_constant]]
PushConstants push_constants;


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    if (gl_GlobalInvocationID.x < (push_constants.instance_count_and_flags & 16777215u))
    {
        uint _130 = push_constants.first_instance + gl_GlobalInvocationID.x;
        float _132[12];
        [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _132[_3ident] = brick_instance_data.Load<float>(_3ident * 4 + _130 * 96 + 0);
    }
        float3x4 _150 = transpose(float4x3(float3(_132[0], _132[1], _132[2]), float3(_132[3], _132[4], _132[5]), float3(_132[6], _132[7], _132[8]), float3(_132[9], _132[10], _132[11])));
        float _160 = length(mul(float3x4(abs(_150[0]), abs(_150[1]), abs(_150[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);
        float _161 = 0.5f * _160;
        float4x4 _164 = float4x4(frame_constants_storage_buffer.Load<float>(496), frame_constants_storage_buffer.Load<float>(512), frame_constants_storage_buffer.Load<float>(528), frame_constants_storage_buffer.Load<float>(544), frame_constants_storage_buffer.Load<float>(500), frame_constants_storage_buffer.Load<float>(516), frame_constants_storage_buffer.Load<float>(532), frame_constants_storage_buffer.Load<float>(548), frame_constants_storage_buffer.Load<float>(504), frame_constants_storage_buffer.Load<float>(520), frame_constants_storage_buffer.Load<float>(536), frame_constants_storage_buffer.Load<float>(552), frame_constants_storage_buffer.Load<float>(508), frame_constants_storage_buffer.Load<float>(524), frame_constants_storage_buffer.Load<float>(540), frame_constants_storage_buffer.Load<float>(556));
        float4x4 _166 = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
        float4 _171 = float4(-_166[0].x, _166[0].y, 0.0f, _166[0].w);
        float4 _175 = float4(_166[1].x, _166[1].y, 0.0f, _166[1].w);
        float4 _179 = float4(_166[3].x, _166[3].y, -1.0f, _166[3].w);
        float4 _119[4];
        _119[0] = _179 + _171;
        _119[1] = _179 - _171;
        _119[2] = _179 + _175;
        _119[3] = _179 - _175;
        for (int _190 = 0; _190 < 4; )
        {
            _119[_190] /= length(_119[_190].xyz).xxxx;
            _190++;
            continue;
        }
        float4 _118[4] = _119;
        float4 _206 = float4(mul(_150, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f);
        float4 _207 = mul(_164, _206);
        float _209 = -_207.z;
        bool _216;
        if ((_209 + _161) > _166[2].w)
        {
            _216 = (_209 - _161) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _216 = false;
        }
        bool _218;
        _218 = _216;
        bool _219;
        for (int _221 = 0; _221 < 4; _218 = _219, _221++)
        {
            if (_218)
            {
                _219 = dot(float4(_207.xyz, 1.0f), _118[_221]) > (_160 * (-0.5f));
            }
            else
            {
                _219 = false;
            }
        }
        bool _246;
        if (_218)
        {
            _246 = (prev_visibility_bitmask.Load<uint>((_130 / 32u) * 4 + 0) & (1u << ((_130 % 32u) & 31u))) != 0u;
        }
        else
        {
            _246 = false;
        }
        if (_246)
        {
            uint _254;
            __alias_next_visibility_bitmask.InterlockedOr((_130 / 32u) * 4 + 0, 1u << ((_130 % 32u) & 31u), _254);
            uint _275;
            if (!((brick_instance_data.Load<uint>(_130 * 96 + 92) & 128u) != 0u))
            {
                float4x4 _263 = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
                _275 = uint((((_161 / _209) * ((0.5f * _263[1].y) * float(clamp(frame_constants_storage_buffer.Load<uint>(148), 1080u, 2160u)))) > 30.0f) ? 0 : 1);
            }
            else
            {
                _275 = 0u;
            }
            uint _281 = clamp(_275, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));
            uint _287;
            __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + _281) * 20 + 4, 1u, _287);
            __alias_brick_draw_list.Store<uint>(((_281 * 2097152u) + _287) * 4 + 0, _130);
        }
        float4x4 _292 = float4x4(frame_constants_storage_buffer.Load<float>(2224), frame_constants_storage_buffer.Load<float>(2240), frame_constants_storage_buffer.Load<float>(2256), frame_constants_storage_buffer.Load<float>(2272), frame_constants_storage_buffer.Load<float>(2228), frame_constants_storage_buffer.Load<float>(2244), frame_constants_storage_buffer.Load<float>(2260), frame_constants_storage_buffer.Load<float>(2276), frame_constants_storage_buffer.Load<float>(2232), frame_constants_storage_buffer.Load<float>(2248), frame_constants_storage_buffer.Load<float>(2264), frame_constants_storage_buffer.Load<float>(2280), frame_constants_storage_buffer.Load<float>(2236), frame_constants_storage_buffer.Load<float>(2252), frame_constants_storage_buffer.Load<float>(2268), frame_constants_storage_buffer.Load<float>(2284));
        float4x4 _296 = float4x4(frame_constants_storage_buffer.Load<float>(1968), frame_constants_storage_buffer.Load<float>(1984), frame_constants_storage_buffer.Load<float>(2000), frame_constants_storage_buffer.Load<float>(2016), frame_constants_storage_buffer.Load<float>(1972), frame_constants_storage_buffer.Load<float>(1988), frame_constants_storage_buffer.Load<float>(2004), frame_constants_storage_buffer.Load<float>(2020), frame_constants_storage_buffer.Load<float>(1976), frame_constants_storage_buffer.Load<float>(1992), frame_constants_storage_buffer.Load<float>(2008), frame_constants_storage_buffer.Load<float>(2024), frame_constants_storage_buffer.Load<float>(1980), frame_constants_storage_buffer.Load<float>(1996), frame_constants_storage_buffer.Load<float>(2012), frame_constants_storage_buffer.Load<float>(2028));
        float _299 = _161 * (2.0f * _296[0].x);
        float2 _300 = abs(mul(_292, _206).xy);
        float2 _302 = (-_299).xx;
        float2 _306 = (1.0f + _299).xx;
        uint _310 = (any(bool2(_300.x >= _302.x, _300.y >= _302.y)) ? all(bool2(_300.x <= _306.x, _300.y <= _306.y)) : false) ? 0u : 3u;
        float2 _312 = (1.0f - _299).xx;
        float2 _316 = (2.0f + _299).xx;
        bool _319 = any(bool2(_300.x >= _312.x, _300.y >= _312.y)) ? all(bool2(_300.x <= _316.x, _300.y <= _316.y)) : false;
        uint _323;
        if (_319)
        {
            _323 = min(_310, 1u);
        }
        else
        {
            _323 = _310;
        }
        uint _324 = uint(_319);
        float2 _326 = (2.0f - _299).xx;
        float2 _330 = (7.0f + _299).xx;
        uint _338;
        uint _339;
        if (any(bool2(_300.x >= _326.x, _300.y >= _326.y)) ? all(bool2(_300.x <= _330.x, _300.y <= _330.y)) : false)
        {
            _338 = max(_324, 2u);
            _339 = min(_323, 2u);
        }
        else
        {
            _338 = _324;
            _339 = _323;
        }
        for (uint _341 = _339; _341 <= _338; )
        {
            uint _346 = 4u + _341;
            uint _352;
            __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + _346) * 20 + 4, 1u, _352);
            __alias_brick_draw_list.Store<uint>(((_346 * 2097152u) + _352) * 4 + 0, _130);
            _341++;
            continue;
        }
    }
}

[numthreads(64, 1, 1)]
void cs_pass1(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_cs_pass2_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _wall_generate_draw_lists.cs.hlsl_cs_pass2_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

RWByteAddressBuffer __alias_draw_commands : register(u13, space0);
cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 hzb_tex_inner_dims : packoffset(c0.y);
};

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
ByteAddressBuffer brick_instance_data : register(t15, space0);
ByteAddressBuffer prev_visibility_bitmask : register(t16, space0);
RWByteAddressBuffer __alias_next_visibility_bitmask : register(u17, space0);
RWByteAddressBuffer __alias_brick_draw_list : register(u18, space0);
struct PushConstants
{
    uint first_instance;
    uint instance_count_and_flags;
};
[[vk::push_constant]]
PushConstants push_constants;

Texture2D<float4> hzb_tex : register(t19, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    if (gl_GlobalInvocationID.x < (push_constants.instance_count_and_flags & 16777215u))
    {
        uint _142 = push_constants.first_instance + gl_GlobalInvocationID.x;
        float _145[12];
        [unroll]
    for (int _7ident = 0; _7ident < 12; _7ident++)
    {
        _145[_7ident] = brick_instance_data.Load<float>(_7ident * 4 + _142 * 96 + 0);
    }
        float3x4 _163 = transpose(float4x3(float3(_145[0], _145[1], _145[2]), float3(_145[3], _145[4], _145[5]), float3(_145[6], _145[7], _145[8]), float3(_145[9], _145[10], _145[11])));
        float _173 = length(mul(float3x4(abs(_163[0]), abs(_163[1]), abs(_163[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);
        float _174 = 0.5f * _173;
        float4x4 _177 = float4x4(frame_constants_storage_buffer.Load<float>(496), frame_constants_storage_buffer.Load<float>(512), frame_constants_storage_buffer.Load<float>(528), frame_constants_storage_buffer.Load<float>(544), frame_constants_storage_buffer.Load<float>(500), frame_constants_storage_buffer.Load<float>(516), frame_constants_storage_buffer.Load<float>(532), frame_constants_storage_buffer.Load<float>(548), frame_constants_storage_buffer.Load<float>(504), frame_constants_storage_buffer.Load<float>(520), frame_constants_storage_buffer.Load<float>(536), frame_constants_storage_buffer.Load<float>(552), frame_constants_storage_buffer.Load<float>(508), frame_constants_storage_buffer.Load<float>(524), frame_constants_storage_buffer.Load<float>(540), frame_constants_storage_buffer.Load<float>(556));
        float4x4 _179 = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
        float4 _184 = float4(-_179[0].x, _179[0].y, 0.0f, _179[0].w);
        float4 _188 = float4(_179[1].x, _179[1].y, 0.0f, _179[1].w);
        float4 _192 = float4(_179[3].x, _179[3].y, -1.0f, _179[3].w);
        float4 _131[4];
        _131[0] = _192 + _184;
        _131[1] = _192 - _184;
        _131[2] = _192 + _188;
        _131[3] = _192 - _188;
        for (int _203 = 0; _203 < 4; )
        {
            _131[_203] /= length(_131[_203].xyz).xxxx;
            _203++;
            continue;
        }
        float4 _130[4] = _131;
        float4 _220 = mul(_177, float4(mul(_163, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
        float _222 = -_220.z;
        bool _229;
        if ((_222 + _174) > _179[2].w)
        {
            _229 = (_222 - _174) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            _229 = false;
        }
        bool _231;
        _231 = _229;
        bool _232;
        for (int _234 = 0; _234 < 4; _231 = _232, _234++)
        {
            if (_231)
            {
                _232 = dot(float4(_220.xyz, 1.0f), _130[_234]) > (_173 * (-0.5f));
            }
            else
            {
                _232 = false;
            }
        }
        float4 _382;
        float _383;
        bool _384;
        if (_231)
        {
            float4x4 _251 = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
            Affine3Packed _253;
            [unroll]
            for (int _8ident = 0; _8ident < 12; _8ident++)
            {
                _253.data[_8ident] = brick_instance_data.Load<float>(_8ident * 4 + _142 * 96 + 0);
            }
            float4 _379;
            float _380;
            bool _381;
            do
            {
                float3x4 _274 = transpose(float4x3(float3(_253.data[0], _253.data[1], _253.data[2]), float3(_253.data[3], _253.data[4], _253.data[5]), float3(_253.data[6], _253.data[7], _253.data[8]), float3(_253.data[9], _253.data[10], _253.data[11])));
                float4x4 _291 = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4x4 _293 = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 _294 = mul(_293, float4(mul(_274, float4(0.0f, 1.0f, 0.0f, 0.0f)), 0.0f));
                float4x4 _295 = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 _296 = mul(_295, float4(mul(_274, float4(0.0f, 0.0f, 1.0f, 0.0f)), 0.0f));
                float4x4 _297 = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 _303 = mul(_297, float4(mul(_274, float4(-0.5f, -0.5f, -0.5f, 1.0f)), 1.0f));
                float4 _304 = _303 + _296;
                float4 _305 = _303 + _294;
                float4 _306 = _305 + _296;
                float4 _307 = _303 + mul(_291, float4(mul(_274, float4(1.0f, 0.0f, 0.0f, 0.0f)), 0.0f));
                float4 _308 = _307 + _296;
                float4 _309 = _307 + _294;
                float4 _310 = _309 + _296;
                float _311 = _303.w;
                float _312 = _304.w;
                float _313 = _305.w;
                float _314 = _306.w;
                float _315 = _307.w;
                float _316 = _308.w;
                float _317 = _309.w;
                float _318 = _310.w;
                if (max(max(max(_311, _312), max(_313, _314)), max(max(_315, _316), max(_317, _318))) < _251[2].w)
                {
                    _379 = 0.0f.xxxx;
                    _380 = 0.0f;
                    _381 = false;
                    break;
                }
                float2 _339 = _303.xy / _311.xx;
                float2 _342 = _304.xy / _312.xx;
                float2 _345 = _305.xy / _313.xx;
                float2 _348 = _306.xy / _314.xx;
                float2 _351 = _307.xy / _315.xx;
                float2 _354 = _308.xy / _316.xx;
                float2 _357 = _309.xy / _317.xx;
                float2 _360 = _310.xy / _318.xx;
                float2 _367 = min(min(min(_339, _342), min(_345, _348)), min(min(_351, _354), min(_357, _360)));
                float2 _374 = max(max(max(_339, _342), max(_345, _348)), max(max(_351, _354), max(_357, _360)));
                _379 = (float4(_367.x, _367.y, _374.x, _374.y).xwzy * float4(0.5f, -0.5f, 0.5f, -0.5f)) + 0.5f.xxxx;
                _380 = -min(min(min(_311, _312), min(_313, _314)), min(min(_315, _316), min(_317, _318)));
                _381 = true;
                break;
            } while(false);
            _382 = _379;
            _383 = _380;
            _384 = _381;
        }
        else
        {
            _382 = 0.0f.xxxx;
            _383 = 0.0f;
            _384 = false;
        }
        float4 _388 = float4(hzb_tex_inner_dims.xyxy);
        float4 _391 = clamp(_382 * _388, 0.0f.xxxx, _388 - 1.0f.xxxx);
        uint _402 = uint(max(0.0f, ceil(log2(max(_391.z - _391.x, _391.w - _391.y)))));
        bool _439;
        if (_384 ? true : false)
        {
            uint2 _409 = _402.xx & uint2(31u, 31u);
            uint2 _410 = uint2(_391.xy) >> _409;
            uint2 _413 = uint2(_391.zw) >> _409;
            float4x4 _433 = float4x4(frame_constants_storage_buffer.Load<float>(304), frame_constants_storage_buffer.Load<float>(320), frame_constants_storage_buffer.Load<float>(336), frame_constants_storage_buffer.Load<float>(352), frame_constants_storage_buffer.Load<float>(308), frame_constants_storage_buffer.Load<float>(324), frame_constants_storage_buffer.Load<float>(340), frame_constants_storage_buffer.Load<float>(356), frame_constants_storage_buffer.Load<float>(312), frame_constants_storage_buffer.Load<float>(328), frame_constants_storage_buffer.Load<float>(344), frame_constants_storage_buffer.Load<float>(360), frame_constants_storage_buffer.Load<float>(316), frame_constants_storage_buffer.Load<float>(332), frame_constants_storage_buffer.Load<float>(348), frame_constants_storage_buffer.Load<float>(364));
            _439 = _383 > (1.0f / (min(min(hzb_tex.Load(int3(_410, _402)).x, hzb_tex.Load(int3(uint2(_410.x, _413.y), _402)).x), min(hzb_tex.Load(int3(uint2(_413.x, _410.y), _402)).x, hzb_tex.Load(int3(_413, _402)).x)) * (-_433[3].z)));
        }
        else
        {
            _439 = _384;
        }
        if (_439)
        {
            uint _446 = 1u << ((_142 % 32u) & 31u);
            uint _447;
            __alias_next_visibility_bitmask.InterlockedOr((_142 / 32u) * 4 + 0, _446, _447);
            if (!((prev_visibility_bitmask.Load<uint>((_142 / 32u) * 4 + 0) & _446) != 0u))
            {
                uint _475;
                if (!((brick_instance_data.Load<uint>(_142 * 96 + 92) & 128u) != 0u))
                {
                    float4x4 _463 = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
                    _475 = uint((((_174 / _222) * ((0.5f * _463[1].y) * float(clamp(frame_constants_storage_buffer.Load<uint>(148), 1080u, 2160u)))) > 30.0f) ? 0 : 1);
                }
                else
                {
                    _475 = 0u;
                }
                uint _481 = clamp(_475, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));
                uint _487;
                __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + _481) * 20 + 4, 1u, _487);
                __alias_brick_draw_list.Store<uint>(((_481 * 2097152u) + _487) * 4 + 0, _142);
            }
        }
    }
}

[numthreads(64, 1, 1)]
void cs_pass2(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
