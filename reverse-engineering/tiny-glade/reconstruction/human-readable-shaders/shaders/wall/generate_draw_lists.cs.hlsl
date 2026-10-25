// PATCHED: modules s.cs.hlsl_cs_pass1_0.bin:cs_pass1, s.cs.hlsl_cs_pass2_0.bin:cs_pass2 are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.

// 墙体绘制列表生成(cs_pass1 视锥裁剪 + LOD/环分桶;cs_pass2 在 pass1 基础上
// 追加 HZB 遮挡裁剪)。转译段(变量为 SPIR-V 编号)已语义化命名;逻辑未改。
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

// FrameConstants 存储缓冲元素(与 frame_constants UBO 同布局;墙体链在 set1/b1 以
// StructuredBuffer 读取)。ViewConstants 来自 inc/frame_constants.hlsl;
// 按已裁定的 inc-fix #4,FrameConstants 在本地重声明。
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

// 砖块标志位解码(内联自 candidate A 的 wall/inc/brick_instance_flags.hlsl)。保留为
// #define(而非文件级 static const),避免泄入隐式 $Globals cbuffer,
// 其墙体家族布局恰为 { int draw_indirect_buffer_offset; uint2 hzb_tex_inner_dims; }。
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

// 砖块绘制列表槽:桶 = 绘制命令索引(pass1 用 0..3,LOD 环用 4..7),
// 每桶容量 0x200000 块砖(内联自 candidate A 的 wall/inc/draw_lists.hlsl)。
uint wall_brick_draw_list_slot(uint bucket, uint prev)
{
    return bucket * WALL_BRICK_DRAW_LIST_BUCKET_CAPACITY + prev;
}

// 打包仿射 -> 3x4 列主序砖块变换(内联自 candidate A)。
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

// ---- 第一轮:视锥裁剪 + 可见性 + LOD / 环绘制桶 ----
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

    // 由裁剪矩阵在视图空间构造视锥左右/上下平面。
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

    // LOD 桶追加 + 下一帧可见性 OR:仅在上一帧可见 且 在视锥内 时执行(bin 门控)。
    if (in_frustum && (prev_visibility_bitmask[inst / 32u] & bit) != 0u)
    {
        InterlockedOr(__alias_next_visibility_bitmask[inst / 32u], bit);

        // 按砖块投影屏幕高度选 LOD 0/1(阈值 30 px);倒角砖固定 LOD 0。
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

// ---- 第二轮:对存活砖块做 HZB 遮挡裁剪(8 角 AABB) ----
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

    // 8 个砖角 -> 屏幕空间 AABB -> HZB mip -> 深度比较。
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
// 转译段 cs_pass1:视锥裁剪 + 可见性 + LOD/环绘制桶(字节地址读取 frame_constants)。 && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
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
        uint inst = push_constants.first_instance + gl_GlobalInvocationID.x;
        float affine_data[12];
        [unroll]
    for (int i = 0; i < 12; i++)
    {
        affine_data[i] = brick_instance_data.Load<float>(i * 4 + inst * 96 + 0);
    }
        float3x4 brick_mat = transpose(float4x3(float3(affine_data[0], affine_data[1], affine_data[2]), float3(affine_data[3], affine_data[4], affine_data[5]), float3(affine_data[6], affine_data[7], affine_data[8]), float3(affine_data[9], affine_data[10], affine_data[11])));
        float axis_len = length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);
        float half_extent = 0.5f * axis_len;
        float4x4 world_to_view = float4x4(frame_constants_storage_buffer.Load<float>(496), frame_constants_storage_buffer.Load<float>(512), frame_constants_storage_buffer.Load<float>(528), frame_constants_storage_buffer.Load<float>(544), frame_constants_storage_buffer.Load<float>(500), frame_constants_storage_buffer.Load<float>(516), frame_constants_storage_buffer.Load<float>(532), frame_constants_storage_buffer.Load<float>(548), frame_constants_storage_buffer.Load<float>(504), frame_constants_storage_buffer.Load<float>(520), frame_constants_storage_buffer.Load<float>(536), frame_constants_storage_buffer.Load<float>(552), frame_constants_storage_buffer.Load<float>(508), frame_constants_storage_buffer.Load<float>(524), frame_constants_storage_buffer.Load<float>(540), frame_constants_storage_buffer.Load<float>(556));
        float4x4 view_to_clip = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
        float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
        float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
        float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
        float4 frustum_corners[4];
        frustum_corners[0] = plane_origin + plane_lr;
        frustum_corners[1] = plane_origin - plane_lr;
        frustum_corners[2] = plane_origin + plane_tb;
        frustum_corners[3] = plane_origin - plane_tb;
        for (int p = 0; p < 4; )
        {
            frustum_corners[p] /= length(frustum_corners[p].xyz).xxxx;
            p++;
            continue;
        }
        float4 planes[4] = frustum_corners;
        float4 world_center = float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f);
        float4 brick_center = mul(world_to_view, world_center);
        float center_depth = -brick_center.z;
        bool in_frustum;
        if ((center_depth + half_extent) > view_to_clip[2].w)
        {
            in_frustum = (center_depth - half_extent) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            in_frustum = false;
        }
        bool running;
        running = in_frustum;
        bool next;
        for (int q = 0; q < 4; running = next, q++)
        {
            if (running)
            {
                next = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (axis_len * (-0.5f));
            }
            else
            {
                next = false;
            }
        }
        bool was_visible;
        if (running)
        {
            was_visible = (prev_visibility_bitmask.Load<uint>((inst / 32u) * 4 + 0) & (1u << ((inst % 32u) & 31u))) != 0u;
        }
        else
        {
            was_visible = false;
        }
        if (was_visible)
        {
            uint or_dummy;
            __alias_next_visibility_bitmask.InterlockedOr((inst / 32u) * 4 + 0, 1u << ((inst % 32u) & 31u), or_dummy);
            uint lod;
            if (!((brick_instance_data.Load<uint>(inst * 96 + 92) & 128u) != 0u))
            {
                float4x4 vtc_y = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
                lod = uint((((half_extent / center_depth) * ((0.5f * vtc_y[1].y) * float(clamp(frame_constants_storage_buffer.Load<uint>(148), 1080u, 2160u)))) > 30.0f) ? 0 : 1);
            }
            else
            {
                lod = 0u;
            }
            uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));
            uint prev;
            __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + bucket) * 20 + 4, 1u, prev);
            __alias_brick_draw_list.Store<uint>(((bucket * 2097152u) + prev) * 4 + 0, inst);
        }
        float4x4 ring_mat = float4x4(frame_constants_storage_buffer.Load<float>(2224), frame_constants_storage_buffer.Load<float>(2240), frame_constants_storage_buffer.Load<float>(2256), frame_constants_storage_buffer.Load<float>(2272), frame_constants_storage_buffer.Load<float>(2228), frame_constants_storage_buffer.Load<float>(2244), frame_constants_storage_buffer.Load<float>(2260), frame_constants_storage_buffer.Load<float>(2276), frame_constants_storage_buffer.Load<float>(2232), frame_constants_storage_buffer.Load<float>(2248), frame_constants_storage_buffer.Load<float>(2264), frame_constants_storage_buffer.Load<float>(2280), frame_constants_storage_buffer.Load<float>(2236), frame_constants_storage_buffer.Load<float>(2252), frame_constants_storage_buffer.Load<float>(2268), frame_constants_storage_buffer.Load<float>(2284));
        float4x4 ring_view = float4x4(frame_constants_storage_buffer.Load<float>(1968), frame_constants_storage_buffer.Load<float>(1984), frame_constants_storage_buffer.Load<float>(2000), frame_constants_storage_buffer.Load<float>(2016), frame_constants_storage_buffer.Load<float>(1972), frame_constants_storage_buffer.Load<float>(1988), frame_constants_storage_buffer.Load<float>(2004), frame_constants_storage_buffer.Load<float>(2020), frame_constants_storage_buffer.Load<float>(1976), frame_constants_storage_buffer.Load<float>(1992), frame_constants_storage_buffer.Load<float>(2008), frame_constants_storage_buffer.Load<float>(2024), frame_constants_storage_buffer.Load<float>(1980), frame_constants_storage_buffer.Load<float>(1996), frame_constants_storage_buffer.Load<float>(2012), frame_constants_storage_buffer.Load<float>(2028));
        float ring = half_extent * (2.0f * ring_view[0].x);
        float2 proj_xy = abs(mul(ring_mat, world_center).xy);
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
        uint ring_lod_max_u = uint(in_ring1);
        float2 ring_lo2 = (2.0f - ring).xx;
        float2 ring_hi2 = (7.0f + ring).xx;
        uint ring_lod_max;
        uint ring_lod_min2;
        if (any(bool2(proj_xy.x >= ring_lo2.x, proj_xy.y >= ring_lo2.y)) ? all(bool2(proj_xy.x <= ring_hi2.x, proj_xy.y <= ring_hi2.y)) : false)
        {
            ring_lod_max = max(ring_lod_max_u, 2u);
            ring_lod_min2 = min(ring_lod_min, 2u);
        }
        else
        {
            ring_lod_max = ring_lod_max_u;
            ring_lod_min2 = ring_lod_min;
        }
        for (uint r = ring_lod_min2; r <= ring_lod_max; )
        {
            uint ring_bucket = 4u + r;
            uint prev_ring;
            __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + ring_bucket) * 20 + 4, 1u, prev_ring);
            __alias_brick_draw_list.Store<uint>(((ring_bucket * 2097152u) + prev_ring) * 4 + 0, inst);
            r++;
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
// 转译段 cs_pass2:HZB 遮挡裁剪 + 8 角 AABB 投影 + LOD 桶追加。 && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
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
        uint inst = push_constants.first_instance + gl_GlobalInvocationID.x;
        float affine_data[12];
        [unroll]
    for (int i = 0; i < 12; i++)
    {
        affine_data[i] = brick_instance_data.Load<float>(i * 4 + inst * 96 + 0);
    }
        float3x4 brick_mat = transpose(float4x3(float3(affine_data[0], affine_data[1], affine_data[2]), float3(affine_data[3], affine_data[4], affine_data[5]), float3(affine_data[6], affine_data[7], affine_data[8]), float3(affine_data[9], affine_data[10], affine_data[11])));
        float axis_len = length(mul(float3x4(abs(brick_mat[0]), abs(brick_mat[1]), abs(brick_mat[2])), float4(1.0f, 1.0f, 1.0f, 0.0f)).xxx);
        float half_extent = 0.5f * axis_len;
        float4x4 world_to_view = float4x4(frame_constants_storage_buffer.Load<float>(496), frame_constants_storage_buffer.Load<float>(512), frame_constants_storage_buffer.Load<float>(528), frame_constants_storage_buffer.Load<float>(544), frame_constants_storage_buffer.Load<float>(500), frame_constants_storage_buffer.Load<float>(516), frame_constants_storage_buffer.Load<float>(532), frame_constants_storage_buffer.Load<float>(548), frame_constants_storage_buffer.Load<float>(504), frame_constants_storage_buffer.Load<float>(520), frame_constants_storage_buffer.Load<float>(536), frame_constants_storage_buffer.Load<float>(552), frame_constants_storage_buffer.Load<float>(508), frame_constants_storage_buffer.Load<float>(524), frame_constants_storage_buffer.Load<float>(540), frame_constants_storage_buffer.Load<float>(556));
        float4x4 view_to_clip = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
        float4 plane_lr = float4(-view_to_clip[0].x, view_to_clip[0].y, 0.0f, view_to_clip[0].w);
        float4 plane_tb = float4(view_to_clip[1].x, view_to_clip[1].y, 0.0f, view_to_clip[1].w);
        float4 plane_origin = float4(view_to_clip[3].x, view_to_clip[3].y, -1.0f, view_to_clip[3].w);
        float4 frustum_corners[4];
        frustum_corners[0] = plane_origin + plane_lr;
        frustum_corners[1] = plane_origin - plane_lr;
        frustum_corners[2] = plane_origin + plane_tb;
        frustum_corners[3] = plane_origin - plane_tb;
        for (int p = 0; p < 4; )
        {
            frustum_corners[p] /= length(frustum_corners[p].xyz).xxxx;
            p++;
            continue;
        }
        float4 planes[4] = frustum_corners;
        float4 brick_center = mul(world_to_view, float4(mul(brick_mat, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f));
        float center_depth = -brick_center.z;
        bool in_frustum;
        if ((center_depth + half_extent) > view_to_clip[2].w)
        {
            in_frustum = (center_depth - half_extent) < 3.4028234663852885981170418348452e+38f;
        }
        else
        {
            in_frustum = false;
        }
        bool running;
        running = in_frustum;
        bool next;
        for (int q = 0; q < 4; running = next, q++)
        {
            if (running)
            {
                next = dot(float4(brick_center.xyz, 1.0f), planes[q]) > (axis_len * (-0.5f));
            }
            else
            {
                next = false;
            }
        }
        float4 bounds;
        float max_depth;
        bool has_bounds;
        if (running)
        {
            float4x4 view_to_clip_c = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
            Affine3Packed affine2;
            [unroll]
            for (int i2 = 0; i2 < 12; i2++)
            {
                affine2.data[i2] = brick_instance_data.Load<float>(i2 * 4 + inst * 96 + 0);
            }
            float4 bounds2;
            float max_depth2;
            bool has_bounds2;
            do
            {
                float3x4 brick_mat2 = transpose(float4x3(float3(affine2.data[0], affine2.data[1], affine2.data[2]), float3(affine2.data[3], affine2.data[4], affine2.data[5]), float3(affine2.data[6], affine2.data[7], affine2.data[8]), float3(affine2.data[9], affine2.data[10], affine2.data[11])));
                float4x4 mvp_x = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4x4 mvp_y = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 axis_y = mul(mvp_y, float4(mul(brick_mat2, float4(0.0f, 1.0f, 0.0f, 0.0f)), 0.0f));
                float4x4 mvp_z = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 axis_z = mul(mvp_z, float4(mul(brick_mat2, float4(0.0f, 0.0f, 1.0f, 0.0f)), 0.0f));
                float4x4 mvp_w = float4x4(frame_constants_storage_buffer.Load<float>(624), frame_constants_storage_buffer.Load<float>(640), frame_constants_storage_buffer.Load<float>(656), frame_constants_storage_buffer.Load<float>(672), frame_constants_storage_buffer.Load<float>(628), frame_constants_storage_buffer.Load<float>(644), frame_constants_storage_buffer.Load<float>(660), frame_constants_storage_buffer.Load<float>(676), frame_constants_storage_buffer.Load<float>(632), frame_constants_storage_buffer.Load<float>(648), frame_constants_storage_buffer.Load<float>(664), frame_constants_storage_buffer.Load<float>(680), frame_constants_storage_buffer.Load<float>(636), frame_constants_storage_buffer.Load<float>(652), frame_constants_storage_buffer.Load<float>(668), frame_constants_storage_buffer.Load<float>(684));
                float4 corner0 = mul(mvp_w, float4(mul(brick_mat2, float4(-0.5f, -0.5f, -0.5f, 1.0f)), 1.0f));
                float4 c1 = corner0 + axis_z;
                float4 c2 = corner0 + axis_y;
                float4 c3 = c2 + axis_z;
                float4 c4 = corner0 + mul(mvp_x, float4(mul(brick_mat2, float4(1.0f, 0.0f, 0.0f, 0.0f)), 0.0f));
                float4 c5 = c4 + axis_z;
                float4 c6 = c4 + axis_y;
                float4 c7 = c6 + axis_z;
                float w0 = corner0.w;
                float w1 = c1.w;
                float w2 = c2.w;
                float w3 = c3.w;
                float w4 = c4.w;
                float w5 = c5.w;
                float w6 = c6.w;
                float w7 = c7.w;
                if (max(max(max(w0, w1), max(w2, w3)), max(max(w4, w5), max(w6, w7))) < view_to_clip_c[2].w)
                {
                    bounds2 = 0.0f.xxxx;
                    max_depth2 = 0.0f;
                    has_bounds2 = false;
                    break;
                }
                float2 n0 = corner0.xy / w0.xx;
                float2 n1 = c1.xy / w1.xx;
                float2 n2 = c2.xy / w2.xx;
                float2 n3 = c3.xy / w3.xx;
                float2 n4 = c4.xy / w4.xx;
                float2 n5 = c5.xy / w5.xx;
                float2 n6 = c6.xy / w6.xx;
                float2 n7 = c7.xy / w7.xx;
                float2 ndc_min = min(min(min(n0, n1), min(n2, n3)), min(min(n4, n5), min(n6, n7)));
                float2 ndc_max = max(max(max(n0, n1), max(n2, n3)), max(max(n4, n5), max(n6, n7)));
                bounds2 = (float4(ndc_min.x, ndc_min.y, ndc_max.x, ndc_max.y).xwzy * float4(0.5f, -0.5f, 0.5f, -0.5f)) + 0.5f.xxxx;
                max_depth2 = -min(min(min(w0, w1), min(w2, w3)), min(min(w4, w5), min(w6, w7)));
                has_bounds2 = true;
                break;
            } while(false);
            bounds = bounds2;
            max_depth = max_depth2;
            has_bounds = has_bounds2;
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
        if (has_bounds ? true : false)
        {
            uint2 mip_mask = mip.xx & uint2(31u, 31u);
            uint2 uv_min = uint2(uv.xy) >> mip_mask;
            uint2 uv_max = uint2(uv.zw) >> mip_mask;
            float4x4 clip_to_view = float4x4(frame_constants_storage_buffer.Load<float>(304), frame_constants_storage_buffer.Load<float>(320), frame_constants_storage_buffer.Load<float>(336), frame_constants_storage_buffer.Load<float>(352), frame_constants_storage_buffer.Load<float>(308), frame_constants_storage_buffer.Load<float>(324), frame_constants_storage_buffer.Load<float>(340), frame_constants_storage_buffer.Load<float>(356), frame_constants_storage_buffer.Load<float>(312), frame_constants_storage_buffer.Load<float>(328), frame_constants_storage_buffer.Load<float>(344), frame_constants_storage_buffer.Load<float>(360), frame_constants_storage_buffer.Load<float>(316), frame_constants_storage_buffer.Load<float>(332), frame_constants_storage_buffer.Load<float>(348), frame_constants_storage_buffer.Load<float>(364));
            occluded = max_depth > (1.0f / (min(min(hzb_tex.Load(int3(uv_min, mip)).x, hzb_tex.Load(int3(uint2(uv_min.x, uv_max.y), mip)).x), min(hzb_tex.Load(int3(uint2(uv_max.x, uv_min.y), mip)).x, hzb_tex.Load(int3(uv_max, mip)).x)) * (-clip_to_view[3].z)));
        }
        else
        {
            occluded = has_bounds;
        }
        if (occluded)
        {
            uint bit = 1u << ((inst % 32u) & 31u);
            uint or_dummy;
            __alias_next_visibility_bitmask.InterlockedOr((inst / 32u) * 4 + 0, bit, or_dummy);
            if (!((prev_visibility_bitmask.Load<uint>((inst / 32u) * 4 + 0) & bit) != 0u))
            {
                uint lod;
                if (!((brick_instance_data.Load<uint>(inst * 96 + 92) & 128u) != 0u))
                {
                    float4x4 vtc_y = float4x4(frame_constants_storage_buffer.Load<float>(240), frame_constants_storage_buffer.Load<float>(256), frame_constants_storage_buffer.Load<float>(272), frame_constants_storage_buffer.Load<float>(288), frame_constants_storage_buffer.Load<float>(244), frame_constants_storage_buffer.Load<float>(260), frame_constants_storage_buffer.Load<float>(276), frame_constants_storage_buffer.Load<float>(292), frame_constants_storage_buffer.Load<float>(248), frame_constants_storage_buffer.Load<float>(264), frame_constants_storage_buffer.Load<float>(280), frame_constants_storage_buffer.Load<float>(296), frame_constants_storage_buffer.Load<float>(252), frame_constants_storage_buffer.Load<float>(268), frame_constants_storage_buffer.Load<float>(284), frame_constants_storage_buffer.Load<float>(300));
                    lod = uint((((half_extent / center_depth) * ((0.5f * vtc_y[1].y) * float(clamp(frame_constants_storage_buffer.Load<uint>(148), 1080u, 2160u)))) > 30.0f) ? 0 : 1);
                }
                else
                {
                    lod = 0u;
                }
                uint bucket = clamp(lod, 0u, 1u) + (2u * uint((push_constants.instance_count_and_flags & 2147483648u) != 0u));
                uint prev;
                __alias_draw_commands.InterlockedAdd((uint(draw_indirect_buffer_offset) + bucket) * 20 + 4, 1u, prev);
                __alias_brick_draw_list.Store<uint>(((bucket * 2097152u) + prev) * 4 + 0, inst);
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
