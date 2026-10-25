// PATCHED: modules ain_14dca8ba9c11c197.bin:cs_main, ain_8faee3cc8c252c04.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
#if !((defined(TGV_14dca8ba9c11c197) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_8faee3cc8c252c04) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/light_binning/bin_lights_to_cells.cs.hlsl
// 中文说明:
//   灯光装箱:把灯笼按影响球 AABB 写入 32x16x32 灯光网格。证据来自
//   _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin:
//   - 影响半径 r = 6*sqrt(intensity)(asfloat 位技巧);
//   - 网格空间 AABB = (pos ± r),对 AABB 内每个球体相交的单元格预留槽位:
//     InterlockedAdd(indirection_grid[cell], 1<<20) 递增计数字段
//     (低 20 位 = 首下标),把灯笼 id 写入 light_indices[start+count];
//   - 超过 64 盏灯后,reservoir 采样以概率 64/(count+1) 替换哈希选中的槽位。
//   两个发行变体(14dca8ba 全 / 8faee3cc 精简):精简变体去掉 frame_constants
//   与 light_indices 写入(BIN_LIGHTS_FULL 编译期开关)。

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

// 全变体中在灯笼下标与 hash(frame_index) 的 LCG 混合后观察到的
// 二级整数哈希(Wang)。
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
// 转译自 _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin
// (spirv-cross;行为精确,变量名已语义化)。全变体:计数 + reservoir 替换。
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

static uint seed_sentinel;

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
    uint lantern_count : packoffset(c0);
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
        if (gl_GlobalInvocationID.x >= lantern_count)
        {
            break;
        }
        float2 intensity_pack = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float intensity = intensity_pack.x;
        if (intensity <= 0.0f)
        {
            break;
        }
        precise float radius = 6.0f * asfloat(532487669 + (asint(intensity) >> 1));
        float3 radius_vec = radius.xxx;
        uint3 cell_min = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - radius_vec) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 cell_max = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + radius_vec) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint fh1 = frame_constants_frame_index + (frame_constants_frame_index << 10u);
        uint fh2 = fh1 ^ (fh1 >> 6u);
        uint fh3 = fh2 + (fh2 << 3u);
        uint fh4 = fh3 ^ (fh3 >> 11u);
        uint lh1 = (((gl_GlobalInvocationID.x * 1664525u) + (fh4 + (fh4 << 15u))) + 1013904223u) * 1664525u;
        uint lh2 = lh1 ^ (lh1 >> 11u);
        uint lh3 = lh2 ^ ((lh2 << 7u) & 2636928640u);
        uint lh4 = lh3 ^ ((lh3 << 15u) & 4022730752u);
        uint rng_init = lh4 ^ (lh4 >> 18u);
        uint z0 = cell_min.z;
        uint res_offset_n;
        uint res_mask_n;
        bool res_flag1_n;
        bool res_flag2_n;
        uint res_rng_n;
        uint z = z0;
        uint res_offset = 0u;
        uint res_mask = 0u;
        bool res_flag1 = false;
        bool res_flag2 = false;
        uint res_rng = rng_init;
        for (; z < cell_max.z; z++, res_offset = res_offset_n, res_mask = res_mask_n, res_flag1 = res_flag1_n, res_flag2 = res_flag2_n, res_rng = res_rng_n)
        {
            uint y0 = cell_min.y;
            res_offset_n = res_offset;
            res_mask_n = res_mask;
            res_flag1_n = res_flag1;
            res_flag2_n = res_flag2;
            res_rng_n = res_rng;
            uint res_offset_b;
            uint res_mask_b;
            bool res_flag1_b;
            bool res_flag2_b;
            uint res_rng_b;
            for (uint y = y0; y < cell_max.y; y++, res_offset_n = res_offset_b, res_mask_n = res_mask_b, res_flag1_n = res_flag1_b, res_flag2_n = res_flag2_b, res_rng_n = res_rng_b)
            {
                uint x0 = cell_min.x;
                res_offset_b = res_offset_n;
                res_mask_b = res_mask_n;
                res_flag1_b = res_flag1_n;
                res_flag2_b = res_flag2_n;
                res_rng_b = res_rng_n;
                uint res_offset_c;
                uint res_mask_c;
                bool res_flag1_c;
                bool res_flag2_c;
                uint res_rng_c;
                for (uint x = x0; x < cell_max.x; res_offset_b = res_offset_c, res_mask_b = res_mask_c, res_flag1_b = res_flag1_c, res_flag2_b = res_flag2_c, res_rng_b = res_rng_c, x++)
                {
                    float3 cell = float3(uint3(x, y, z));
                    float3 aabb_min = (cell - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 aabb_max = (cell + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float r2 = radius * radius;
                    float mnx = aabb_min.x;
                    float d2x;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < mnx)
                    {
                        float dx_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - mnx;
                        d2x = r2 - (dx_lo * dx_lo);
                    }
                    else
                    {
                        float mxx = aabb_max.x;
                        float d2x_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > mxx)
                        {
                            float dx_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - mxx;
                            d2x_b = r2 - (dx_hi * dx_hi);
                        }
                        else
                        {
                            d2x_b = r2;
                        }
                        d2x = d2x_b;
                    }
                    float mny = aabb_min.y;
                    float d2y;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < mny)
                    {
                        float dy_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - mny;
                        d2y = d2x - (dy_lo * dy_lo);
                    }
                    else
                    {
                        float mxy = aabb_max.y;
                        float d2y_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > mxy)
                        {
                            float dy_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - mxy;
                            d2y_b = d2x - (dy_hi * dy_hi);
                        }
                        else
                        {
                            d2y_b = d2x;
                        }
                        d2y = d2y_b;
                    }
                    float mnz = aabb_min.z;
                    float d2z;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < mnz)
                    {
                        float dz_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - mnz;
                        d2z = d2y - (dz_lo * dz_lo);
                    }
                    else
                    {
                        float mxz = aabb_max.z;
                        float d2z_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > mxz)
                        {
                            float dz_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - mxz;
                            d2z_b = d2y - (dz_hi * dz_hi);
                        }
                        else
                        {
                            d2z_b = d2y;
                        }
                        d2z = d2z_b;
                    }
                    if (d2z > 0.0f)
                    {
                        uint prev;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * y) + z)) + x) * 4 + 0, 1048576u, prev);
                        uint start = prev & 1048575u;
                        uint count = prev >> 20u;
                        uint ro;
                        uint rm;
                        bool rf1;
                        bool rf2;
                        uint rr;
                        if (count < 64u)
                        {
                            __alias_light_indices.Store<uint>((start + count) * 4 + 0, gl_GlobalInvocationID.x);
                            ro = res_offset_b;
                            rm = res_mask_b;
                            rf1 = res_flag1_b;
                            rf2 = res_flag2_b;
                            rr = res_rng_b;
                        }
                        else
                        {
                            uint ra1 = res_rng_b + (res_rng_b << 10u);
                            uint ra2 = ra1 ^ (ra1 >> 6u);
                            uint ra3 = ra2 + (ra2 << 3u);
                            uint ra4 = ra3 ^ (ra3 >> 11u);
                            uint ra5 = ra4 + (ra4 << 15u);
                            uint mask = res_flag2_b ? res_mask_b : 8388607u;
                            uint offset = res_flag1_b ? res_offset_b : 1065353216u;
                            uint rng_next;
                            if ((asfloat((res_rng_b & mask) | offset) - 1.0f) < (64.0f / float(count + 1u)))
                            {
                                uint rb1 = ra5 + (ra5 << 10u);
                                uint rb2 = rb1 ^ (rb1 >> 6u);
                                uint rb3 = rb2 + (rb2 << 3u);
                                uint rb4 = rb3 ^ (rb3 >> 11u);
                                __alias_light_indices.Store<uint>((start + (ra5 % 64u)) * 4 + 0, gl_GlobalInvocationID.x);
                                rng_next = rb4 + (rb4 << 15u);
                            }
                            else
                            {
                                rng_next = ra5;
                            }
                            ro = offset;
                            rm = mask;
                            rf1 = res_flag1_b ? res_flag1_b : true;
                            rf2 = res_flag2_b ? res_flag2_b : true;
                            rr = rng_next;
                        }
                        res_offset_c = ro;
                        res_mask_c = rm;
                        res_flag1_c = rf1;
                        res_flag2_c = rf2;
                        res_rng_c = rr;
                    }
                    else
                    {
                        res_offset_c = res_offset_b;
                        res_mask_c = res_mask_b;
                        res_flag1_c = res_flag1_b;
                        res_flag2_c = res_flag2_b;
                        res_rng_c = res_rng_b;
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
// 转译自 _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_8faee3cc8c252c04.bin
// (spirv-cross;行为精确,变量名已语义化)。精简变体:仅计数,无 light_indices。
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
    uint lantern_count : packoffset(c0);
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
        if (gl_GlobalInvocationID.x >= lantern_count)
        {
            break;
        }
        float2 intensity_pack = spvUnpackHalf2x16(lantern_buf.Load<uint>(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float intensity = intensity_pack.x;
        if (intensity <= 0.0f)
        {
            break;
        }
        precise float radius = 6.0f * asfloat(532487669 + (asint(intensity) >> 1));
        float3 radius_vec = radius.xxx;
        uint3 cell_min = uint3(clamp(floor(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) - radius_vec) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 cell_max = uint3(clamp(ceil(((lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0) + radius_vec) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint z0 = cell_min.z;
        for (uint z = z0; z < cell_max.z; z++)
        {
            uint y0 = cell_min.y;
            for (uint y = y0; y < cell_max.y; y++)
            {
                uint x0 = cell_min.x;
                for (uint x = x0; x < cell_max.x; x++)
                {
                    float3 cell = float3(uint3(x, y, z));
                    float3 aabb_min = (cell - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 aabb_max = (cell + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    precise float r2 = radius * radius;
                    float mnx = aabb_min.x;
                    float d2x;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x < mnx)
                    {
                        float dx_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - mnx;
                        d2x = r2 - (dx_lo * dx_lo);
                    }
                    else
                    {
                        float mxx = aabb_max.x;
                        float d2x_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x > mxx)
                        {
                            float dx_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).x - mxx;
                            d2x_b = r2 - (dx_hi * dx_hi);
                        }
                        else
                        {
                            d2x_b = r2;
                        }
                        d2x = d2x_b;
                    }
                    float mny = aabb_min.y;
                    float d2y;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y < mny)
                    {
                        float dy_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - mny;
                        d2y = d2x - (dy_lo * dy_lo);
                    }
                    else
                    {
                        float mxy = aabb_max.y;
                        float d2y_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y > mxy)
                        {
                            float dy_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).y - mxy;
                            d2y_b = d2x - (dy_hi * dy_hi);
                        }
                        else
                        {
                            d2y_b = d2x;
                        }
                        d2y = d2y_b;
                    }
                    float mnz = aabb_min.z;
                    float d2z;
                    if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z < mnz)
                    {
                        float dz_lo = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - mnz;
                        d2z = d2y - (dz_lo * dz_lo);
                    }
                    else
                    {
                        float mxz = aabb_max.z;
                        float d2z_b;
                        if (lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z > mxz)
                        {
                            float dz_hi = lantern_buf.Load<float3>(gl_GlobalInvocationID.x * 16 + 0).z - mxz;
                            d2z_b = d2y - (dz_hi * dz_hi);
                        }
                        else
                        {
                            d2z_b = d2y;
                        }
                        d2z = d2z_b;
                    }
                    if (d2z > 0.0f)
                    {
                        uint prev;
                        __alias_indirection_grid.InterlockedAdd(((32u * ((32u * y) + z)) + x) * 4 + 0, 1u, prev);
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
