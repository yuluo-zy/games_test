// PATCHED: modules ain_6f1dd5dd8a66e104.bin:cs_main, ain_72a2a95d73e37edc.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 全分辨率散焦 LOD1 (dof/fullres_all_lod1):最粗的 LOD 聚散,5 tap x 2 槽(除数 1.0),
//   双环(6f1d)或四环(72a2)采集后按 CoC 权重归一化写出。
// 入口: cs_main (compute, numthreads 8x8x1)。变体: 6f1dd5dd8a66e104 与
//   72a2a95d73e37edc 的 cs_main 均 lifted。
// 绑定: blue_noise t14, color_tex t15, coc_tiles_dilated t16, coc_tex t18,
//   img_output u19 (Rgba16f), tile_offsets b20 (ByteAddress), tile_coords b21
//   (ByteAddress), $Globals b13, frame_constants set1/b0。
#if !((defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/fullres_all_lod1.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (format-correct RWTexture2D<half4>, marginal histogram edge). blue_noise_texture declared locally (main inc/blue_noise.hlsl no longer declares the global). 6f1d noise scheme only (72a2 residual).
//   Full-res bokeh pass at LOD1 scale: 5-tap ring, 2 bins, no half-res upsample
//   blend (tile list offsets[2]).

#include "inc/common.hlsl"
#include "dof/multires.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/hash.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/morton.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"
#include "dof/tiles.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);

groupshared uint dof_ring_samples[64][3];

// 把一对 half 打包成 uint(共享内存存 CoC 对)。
uint PackRingPair(float2 v)
{
    return f32tof16(v.x) | (f32tof16(v.y) << 16u);
}

float2 UnpackRingPair(uint v)
{
    return float2(f16tof32(v & 0xFFFFu), f16tof32(v >> 16u));
}

// 由 dispatch id 与桶索引算 8x8 像素块原点。
uint2 TileOrigin(uint3 group_id, uint3 dispatch_id, uint offset_index)
{
    return (dispatch_id.xy & uint2(7u, 7u))
        + uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offset_index] + group_id.x];
}

// 按视线方向做各向异性的采样方向旋转/拉伸。
float2 BokehDir(float2 dir, float2 uv)
{
    if (bokeh_anisotropy > 0.0f)
    {
        float4 v = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
        float2 p = v.xy / v.z;
        p.y = -p.y;
        float2 n = normalize(p);
        float2x2 rot = float2x2(n, float2(-n.y, n.x));
        float a = lerp(0.699999988079071f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(p, p)));
        return mul(transpose(rot), mul(rot, dir) * float2(a, 1.0f / a));
    }
    return dir;
}

// 5 tap 采样环:累加 CoC 权重/最小/最大,并把相邻 tap 成对打包进共享内存。
void SampleRing(uint thread, float2 uv, float2 dir_px, float ring_offset, float coc_step, float coc_limit,
                float lerp_factor, inout float coc_acc, inout float min_abs_coc, inout float max_abs_coc)
{
    float pending = 0.0f;
    [unroll]
    for (uint k = 0u; k < 5u; ++k)
    {
        float coc = clamp(coc_tex.SampleLevel(sampler_nnc,
            uv + dir_px * ((float(int(k - 2u)) + ring_offset) * coc_step), 0.0f).x, -coc_limit, coc_limit);
        float coc_abs = abs(coc);
        coc_acc = lerp(coc_acc, max(coc_acc, coc_abs), lerp_factor);
        min_abs_coc = min(min_abs_coc, coc_abs);
        max_abs_coc = max(max_abs_coc, coc_abs);
        if ((k & 1u) == 0u)
        {
            pending = coc;
        }
        else
        {
            dof_ring_samples[thread][k >> 1u] =
                PackRingPair(float2(pending, 0.0f)) | (PackRingPair(float2(coc, 0.0f)) << 16u);
        }
    }
    dof_ring_samples[thread][2u] = PackRingPair(float2(pending, 0.0f));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint2 pixel = TileOrigin(group_id, dispatch_id, 2u);
    float2 uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);
    // CoC 上限。
    float coc_limit = min(float(color_tex_dims.y) * 0.041666667908430099f,
        coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    // 蓝色噪声抖动。
    float4 noise = ((blue_noise_texture.Load(int3((pixel + uint2(13u, 13u)
        + uint2(int(frame_index * 59u), int(frame_index * 37u))) & uint2(255u, 255u), 0)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
    // 每像素哈希抖动的子步进与符号选择。
    uint2 jitter_coord = uint2(7u, 7u) + pixel;
    float ring_jitter = (frac(52.982917785644531f * frac((0.067110560834407806f * float(jitter_coord.x))
        + (0.005837149918079376f * float(jitter_coord.y)))) - 0.5f) * 0.5f;
    float ring_phase = 0.5f + ring_jitter;
    float ring_offset = noise.y;
    float coc_step = coc_limit * 0.5f;
    float spoke_seed = 5.588237762451172f * float(frame_index % 64u);
    float spoke_rot = frac(52.982917785644531f * frac((0.067110560834407806f * (float(pixel.x) + spoke_seed))
        + (0.005837149918079376f * (float(pixel.y) + spoke_seed))));
    bool flip = ring_phase < 0.5f;
    float mask_phase = flip ? (0.5f - ring_jitter) : ring_phase;
    float spoke_sign = flip ? -1.0f : 1.0f;

    // 各累加器初始化。
    float4 color_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float coc_acc = 0.0f;
    float coc_weight = 0.0f;
    float weight_acc = 0.0f;
    float min_abs_coc = 1.0e10f;
    float max_abs_coc = -1.0e10f;

    // 双环 x 5 tap 的 CoC 聚散环采集(2 槽)。
    [loop]
    for (uint spoke = 0u; spoke < 2u; ++spoke)
    {
        float angle = (float(spoke) + spoke_rot) * 1.5707963705062866f;
        float2 dir = BokehDir(float2(cos(angle), sin(angle)), uv);
        float2 dir_px = dir * (spoke_sign / float2(color_tex_dims));
        SampleRing(local_index, uv, dir_px, ring_offset, coc_step, coc_limit, noise.x * 0.2f,
            coc_acc, min_abs_coc, max_abs_coc);

        // 记录每个 tap 落在哪个槽位。
        uint mask[2];
        [unroll]
        for (uint mi = 0u; mi < 2u; ++mi)
        {
            mask[mi] = 0u;
        }
        [loop]
        for (uint j = 0u; j < 5u; ++j)
        {
            uint lane_bit = j & 1u;
            uint pair = j >> 1u;
            float2 pair_coc = UnpackRingPair(dof_ring_samples[local_index][pair]);
            float ring_dist = (float(int(j - 2u)) + ring_offset) * coc_step;
            [unroll]
            for (uint b = 0u; b < 2u; ++b)
            {
                if ((ring_dist / ((float(b) + mask_phase) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[b] |= (1u << (j & 31u));
                }
            }
        }

        // 逐槽解析:取环上最近的两个 tap 做亚像素插值。
        float4 spoke_color = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint bin = 0u; bin < 2u; ++bin)
        {
            float mask_dist = (float(bin) + mask_phase) - 1.0f;
            float abs_dist = abs(mask_dist);
            coc_weight += abs_dist;
            if (mask[bin] == 0u)
            {
                continue;
            }
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                // 正向:用最前位与前一 bit 的 CoC 对。
                crossing = firstbitlow(mask[bin]);
                uint prev = uint(max(0, (int(crossing) - 1)));
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][prev >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[prev & 1u];
            }
            else
            {
                // 负向:用最高位与下一 bit 的 CoC 对。
                crossing = firstbithigh(mask[bin]);
                uint next = min(4u, crossing + 1u);
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][next >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[next & 1u];
            }
            float ring_dist = (float(int(crossing - 2u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : -1.0f);
            float candidate = ring_dist - step;
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs_dist * float(mask[bin] != 0u);
            float coc_pos = lerp(candidate, ring_dist,
                clamp(cross_weight / (cross_weight - (coc_a - dist_ratio)), 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (dir_px * coc_pos), 0.0f).x,
                -coc_limit, coc_limit);
            // 遮挡:按 CoC 跨度降权,并在相邻 lane 取最小。
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, max_abs_coc - min_abs_coc);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            mip = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                continue;
            }
            // 靠近环边缘时再做一次亚像素采样。
            float epsilon = cross_weight * 0.1f;
            bool near = false;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            float add_weight;
            float add_color_weight;
            // No 8/11 near-branch guard for lod1 (target).
            if (near)
            {
                // 近边缘:直接取该槽的颜色与权重。
                add_weight = weight_acc;
                add_color_weight = sample_weight;
            }
            else
            {
                float s = smoothstep(0.0f, 30.0f, coc_b) * smoothstep(0.2f * coc_limit, 0.0f, coc_a);
                add_weight = weight_acc + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            weight_acc = add_weight;
            spoke_color += color_tex.SampleLevel(sampler_nnc,
                uv + (dir_px * (near ? coc_pos : candidate)), min(mip, mip_y)) * add_color_weight;
        }
        color_acc += spoke_color;
    }

    // 散射项:超出景深权重阈值时补充 16 点背景散射。
    float scatter = weight_acc * smoothstep(0.01f * color_acc.w,
        max(9.9999997473787516e-06f, 0.2f * color_acc.w), weight_acc);
    if (scatter > 0.0f)
    {
        float min_coc = -coc_limit;
        float4 disk_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint d = 0u; d < 16u; ++d)
        {
            float2 disc = frac(frac((float2(0.7548776865005493f, 0.5698403120040894f) * float(d)) + 0.5f) + noise.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.2831854820251465f;
            float2 sample_uv = uv + (((float2(cos(theta) * r, sin(theta) * r) * 1.0f) / float2(color_tex_dims)) * coc_acc);
            float4 c = coc_tex.SampleLevel(sampler_nnc, sample_uv, 0.0f);
            if (c.x > (min_coc - 10.0f))
            {
                disk_acc += color_tex.SampleLevel(sampler_llc, sample_uv, 0.0f);
                min_coc = max(min_coc, c.x);
            }
        }
        color_acc += (disk_acc / max(1.0f, disk_acc.w)) * scatter;
    }

    // 按 CoC 权重归一化后写出(截断为 >=0)。
    img_output[pixel] = max(float4(0.0f, 0.0f, 0.0f, 0.0f), color_acc / coc_weight);
}

#endif

#if defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 6f1d 变体的 cs 段:双环 x 5 tap,2 槽(与手写 cs 语义一致,变量名已语义化)。
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

struct DofTileOffsets
{
    uint offsets[8];
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

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][3];

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
    // 当前 8x8 像素块(桶 2)。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(8) + gl_WorkGroupID.x) * 8 + 0));
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    // CoC 上限。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);
    // 蓝色噪声抖动。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 jitter_coord = uint2(7u, 7u) + pixel;
    // 每像素哈希抖动的子步进与符号选择。
    float ring_jitter = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(jitter_coord.x)) + (0.005837149918079376220703125f * float(jitter_coord.y)))) - 0.5f) * 0.5f;
    float ring_phase = 0.5f + ring_jitter;
    float ring_offset = dither.y;
    float coc_step = coc_limit * 0.5f;
    float spoke_seed = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float spoke_rot = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + spoke_seed)) + (0.005837149918079376220703125f * (float(pixel.y) + spoke_seed))));
    bool flip = ring_phase < 0.5f;
    float mask_phase;
    if (flip)
    {
        mask_phase = 0.5f - ring_jitter;
    }
    else
    {
        mask_phase = ring_phase;
    }
    float spoke_sign = flip ? (-1.0f) : 1.0f;
    // 累加器初始化。
    float4 color_accum;
    float coc_acc;
    float coc_weight;
    float weight_acc;
    color_accum = 0.0f.xxxx;
    coc_acc = 0.0f;
    coc_weight = 0.0f;
    weight_acc = 0.0f;
    float4 color_accum_next;
    float coc_acc_next;
    float coc_min_next;
    float coc_max_next;
    uint mask[2];
    float coc_weight_next;
    float weight_acc_next;
    uint spoke = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    // 双环 x 5 tap。
    [loop]
    for (; spoke < 2u; color_accum = color_accum_next, coc_acc = coc_acc_next, coc_weight = coc_weight_next, weight_acc = weight_acc_next, spoke++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(spoke) + spoke_rot) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性拉伸。
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((pixel_uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z.xx;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (spoke_sign.xx / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        // tap -2。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-2.0f) + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = dither.x * 0.20000000298023223876953125f;
        float w = lerp(coc_acc, max(coc_acc, abs_coc0), lerp_gain);
        // tap -1。
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-1.0f) + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        // tap 0。
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (ring_offset * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        // tap +1。
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        // tap +2。
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        coc_acc_next = lerp(w, max(w, abs_coc4), lerp_gain);
        // 更新该环 CoC 最小/最大跨度。
        coc_min_next = min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4);
        coc_max_next = max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f));
        // 清空槽位位图。
        for (uint i = 0u; i < 2u; )
        {
            mask[i] = 0u;
            i++;
            continue;
        }
        // 5 个 tap 标记到 2 个槽位。
        for (uint tap = 0u; tap < 5u; tap++)
        {
            uint lane_bit = tap & 1u;
            uint pair = tap >> 1u;
            float2 pair_coc = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][pair] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][pair] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 2u)) + ring_offset) * coc_step;
            for (int slot = 0; slot < 2; slot++)
            {
                if ((tap_coc / ((float(slot) + mask_phase) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[slot] |= (1u << (tap & 31u));
                }
            }
        }
        // 逐槽解析。
        float4 slot_color;
        weight_acc_next = weight_acc;
        coc_weight_next = coc_weight;
        slot_color = 0.0f.xxxx;
        float slot_weight_next;
        float slot_depth_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 2; weight_acc_next = slot_depth_next, coc_weight_next = slot_weight_next, slot_color = slot_color_next, slot++)
        {
            float mask_dist = (float(slot) + mask_phase) - 1.0f;
            float abs_dist = abs(mask_dist);
            slot_weight_next = coc_weight_next + abs_dist;
            if (mask[slot] == 0u)
            {
                slot_depth_next = weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                // 正向:最前位 CoC 对。
                uint low_bit = firstbitlow(mask[slot]);
                uint prev = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 c1 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev >> 1u;
                float2 c2 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                coc_a = c1[low_bit & 1u];
                coc_b = c2[prev & 1u];
                crossing = low_bit;
            }
            else
            {
                // 负向:最高位 CoC 对。
                uint high_bit = firstbithigh(mask[slot]);
                uint next = min(4u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 c1 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next >> 1u;
                float2 c2 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                coc_a = c1[high_bit & 1u];
                coc_b = c2[next & 1u];
                crossing = high_bit;
            }
            float ring_dist = (float(int(crossing - 2u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : (-1.0f));
            float candidate = ring_dist - step;
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs_dist * ((mask[slot] != 0u) ? 1.0f : 0.0f);
            float coc_pos = lerp(candidate, ring_dist, clamp(cross_weight / (cross_weight + (-(coc_a - dist_ratio))), 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * coc_pos), 0.0f).x, neg_coc, coc_limit);
            // 遮挡降权。
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max_next - coc_min_next);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                slot_depth_next = weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            float epsilon = cross_weight * 0.100000001490116119384765625f;
            bool near;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            else
            {
                near = false;
            }
            float add_weight;
            float add_color_weight;
            if (near)
            {
                // 近边缘:直接取该槽的颜色与权重(lod1 无 8/11 渐近护栏)。
                add_weight = weight_acc_next;
                add_color_weight = sample_weight;
            }
            else
            {
                float s = smoothstep(0.0f, 30.0f, coc_b) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, coc_a);
                add_weight = weight_acc_next + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            slot_depth_next = add_weight;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (near ? coc_pos : candidate)), min(occ_min, mip_y)) * add_color_weight);
        }
        // 累加该环的颜色贡献。
        color_accum_next = color_accum + slot_color;
    }
    // 散射项。
    float scatter = weight_acc * smoothstep(0.00999999977648258209228515625f * color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * color_accum.w), weight_acc);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        // 16 点背景散射。
        float min_coc = -coc_limit;
        float4 disk_acc;
        disk_acc = 0.0f.xxxx;
        float4 disk_acc_next;
        float min_coc_next;
        float min_coc_running = min_coc;
        uint d = 0u;
        for (; d < 16u; disk_acc = disk_acc_next, min_coc_running = min_coc_next, d++)
        {
            float2 disc = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(d)) + 0.5f.xx) + dither.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.283185482025146484375f;
            float2 sample_uv = pixel_uv + (((float2(cos(theta) * r, sin(theta) * r) * 1.0f) / float2(color_tex_dims)) * coc_acc);
            float4 c = coc_tex.SampleLevel(sampler_nnc, sample_uv, 0.0f);
            if (c.x > (min_coc_running - 10.0f))
            {
                disk_acc_next = disk_acc + color_tex.SampleLevel(sampler_llc, sample_uv, 0.0f);
                min_coc_next = max(min_coc_running, c.x);
            }
            else
            {
                disk_acc_next = disk_acc;
                min_coc_next = min_coc_running;
            }
        }
        final_color = color_accum + ((disk_acc / max(1.0f, disk_acc.w).xxxx) * scatter);
    }
    else
    {
        final_color = color_accum;
    }
    // 按 CoC 权重归一化后写出。
    float4 result = final_color / coc_weight.xxxx;
    img_output[pixel] = float4((result / max(1.0000000133514319600180897396058e-10f, result.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// 72a2 变体的 cs 段:四环 x 5 tap,仍为 2 槽(除数 1.0),抖动子步进改用 dither.z。
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

struct DofTileOffsets
{
    uint offsets[8];
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

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.y);
    uint2 color_tex_dims : packoffset(c1);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 depth_tex_dims : packoffset(c2);
    uint2 coc_tex_dims : packoffset(c2.z);
    uint2 img_output_dims : packoffset(c3);
};

ByteAddressBuffer tile_offsets_buffer : register(t20, space0);
ByteAddressBuffer tile_coords_buffer : register(t21, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);

static uint3 gl_WorkGroupID;
static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint ring_samples[64][3];

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
    // 当前 8x8 像素块(桶 2)。
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(8) + gl_WorkGroupID.x) * 8 + 0));
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f.xx) / float2(img_output_dims);
    // CoC 上限。
    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);
    // 蓝色噪声抖动。
    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float dither_z = dither.z;
    float ring_offset = dither.y;
    float coc_step = coc_limit * 0.5f;
    float spoke_rot = dither.x;
    // 用 dither.z 决定子步进:取 1-z 或 z。
    bool flip = dither_z < 0.5f;
    float mask_phase;
    if (flip)
    {
        mask_phase = 1.0f - dither_z;
    }
    else
    {
        mask_phase = dither_z;
    }
    float spoke_sign = flip ? (-1.0f) : 1.0f;
    // 累加器初始化。
    float4 color_accum;
    float coc_acc;
    float coc_weight;
    float weight_acc;
    color_accum = 0.0f.xxxx;
    coc_acc = 0.0f;
    coc_weight = 0.0f;
    weight_acc = 0.0f;
    float4 color_accum_next;
    float coc_acc_next;
    float coc_min_next;
    float coc_max_next;
    uint mask[2];
    float coc_weight_next;
    float weight_acc_next;
    uint spoke = 0u;
    float coc_min = 10000000000.0f;
    float coc_max = -10000000000.0f;
    // 四环 x 5 tap。
    [loop]
    for (; spoke < 4u; color_accum = color_accum_next, coc_acc = coc_acc_next, coc_weight = coc_weight_next, weight_acc = weight_acc_next, spoke++, coc_min = coc_min_next, coc_max = coc_max_next)
    {
        float angle = (float(spoke) + spoke_rot) * 0.785398185253143310546875f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            // 各向异性拉伸。
            float4 ctv = mul(frame_constants_view_constants[0u].clip_to_view, float4((pixel_uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z.xx;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (spoke_sign.xx / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        // tap -2。
        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-2.0f) + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc0 = abs(coc0);
        float lerp_gain = spoke_rot * 0.20000000298023223876953125f;
        float w = lerp(coc_acc, max(coc_acc, abs_coc0), lerp_gain);
        // tap -1。
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (((-1.0f) + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(coc0, 0.0f)) | (spvPackHalf2x16(float2(coc1, 0.0f)) << 16u);
        // tap 0。
        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (ring_offset * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc2 = abs(coc2);
        w = lerp(w, max(w, abs_coc2), lerp_gain);
        // tap +1。
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(coc2, 0.0f)) | (spvPackHalf2x16(float2(coc3, 0.0f)) << 16u);
        // tap +2。
        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + ring_offset) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float abs_coc4 = abs(coc4);
        coc_acc_next = lerp(w, max(w, abs_coc4), lerp_gain);
        // 更新该环 CoC 最小/最大跨度。
        coc_min_next = min(min(min(coc_min, abs_coc0), abs_coc2), abs_coc4);
        coc_max_next = max(max(max(coc_max, abs_coc0), abs_coc2), abs_coc4);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(coc4, 0.0f));
        // 清空槽位位图。
        for (uint i = 0u; i < 2u; )
        {
            mask[i] = 0u;
            i++;
            continue;
        }
        // 5 个 tap 标记到 2 个槽位。
        for (uint tap = 0u; tap < 5u; tap++)
        {
            uint lane_bit = tap & 1u;
            uint pair = tap >> 1u;
            float2 pair_coc = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][pair] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][pair] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 2u)) + ring_offset) * coc_step;
            for (int slot = 0; slot < 2; slot++)
            {
                if ((tap_coc / ((float(slot) + mask_phase) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[slot] |= (1u << (tap & 31u));
                }
            }
        }
        // 逐槽解析。
        float4 slot_color;
        weight_acc_next = weight_acc;
        coc_weight_next = coc_weight;
        slot_color = 0.0f.xxxx;
        float slot_weight_next;
        float slot_depth_next;
        float4 slot_color_next;
        for (int slot = 0; slot < 2; weight_acc_next = slot_depth_next, coc_weight_next = slot_weight_next, slot_color = slot_color_next, slot++)
        {
            float mask_dist = (float(slot) + mask_phase) - 1.0f;
            float abs_dist = abs(mask_dist);
            slot_weight_next = coc_weight_next + abs_dist;
            if (mask[slot] == 0u)
            {
                slot_depth_next = weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                // 正向:最前位 CoC 对。
                uint low_bit = firstbitlow(mask[slot]);
                uint prev = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 c1 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev >> 1u;
                float2 c2 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                coc_a = c1[low_bit & 1u];
                coc_b = c2[prev & 1u];
                crossing = low_bit;
            }
            else
            {
                // 负向:最高位 CoC 对。
                uint high_bit = firstbithigh(mask[slot]);
                uint next = min(4u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 c1 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next >> 1u;
                float2 c2 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                coc_a = c1[high_bit & 1u];
                coc_b = c2[next & 1u];
                crossing = high_bit;
            }
            float ring_dist = (float(int(crossing - 2u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : (-1.0f));
            float candidate = ring_dist - step;
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs_dist * ((mask[slot] != 0u) ? 1.0f : 0.0f);
            float coc_pos = lerp(candidate, ring_dist, clamp(cross_weight / (cross_weight + (-(coc_a - dist_ratio))), 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * coc_pos), 0.0f).x, neg_coc, coc_limit);
            // 遮挡降权。
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max_next - coc_min_next);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            float occ_min = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(occ_min, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                slot_depth_next = weight_acc_next;
                slot_color_next = slot_color;
                continue;
            }
            float epsilon = cross_weight * 0.100000001490116119384765625f;
            bool near;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            else
            {
                near = false;
            }
            float add_weight;
            float add_color_weight;
            if (near)
            {
                // 近边缘:直接取该槽的颜色与权重(lod1 无 8/11 渐近护栏)。
                add_weight = weight_acc_next;
                add_color_weight = sample_weight;
            }
            else
            {
                float s = smoothstep(0.0f, 30.0f, coc_b) * smoothstep(0.20000000298023223876953125f * coc_limit, 0.0f, coc_a);
                add_weight = weight_acc_next + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            slot_depth_next = add_weight;
            slot_color_next = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (near ? coc_pos : candidate)), min(occ_min, mip_y)) * add_color_weight);
        }
        // 累加该环的颜色贡献。
        color_accum_next = color_accum + slot_color;
    }
    // 散射项。
    float scatter = weight_acc * smoothstep(0.00999999977648258209228515625f * color_accum.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * color_accum.w), weight_acc);
    float4 final_color;
    [branch]
    if (int(scatter > 0.0f) != 0)
    {
        // 16 点背景散射。
        float min_coc = -coc_limit;
        float4 disk_acc;
        disk_acc = 0.0f.xxxx;
        float4 disk_acc_next;
        float min_coc_next;
        float min_coc_running = min_coc;
        uint d = 0u;
        for (; d < 16u; disk_acc = disk_acc_next, min_coc_running = min_coc_next, d++)
        {
            float2 disc = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(d)) + 0.5f.xx) + dither.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.283185482025146484375f;
            float2 sample_uv = pixel_uv + (((float2(cos(theta) * r, sin(theta) * r) * 1.0f) / float2(color_tex_dims)) * coc_acc);
            float4 c = coc_tex.SampleLevel(sampler_nnc, sample_uv, 0.0f);
            if (c.x > (min_coc_running - 10.0f))
            {
                disk_acc_next = disk_acc + color_tex.SampleLevel(sampler_llc, sample_uv, 0.0f);
                min_coc_next = max(min_coc_running, c.x);
            }
            else
            {
                disk_acc_next = disk_acc;
                min_coc_next = min_coc_running;
            }
        }
        final_color = color_accum + ((disk_acc / max(1.0f, disk_acc.w).xxxx) * scatter);
    }
    else
    {
        final_color = color_accum;
    }
    // 按 CoC 权重归一化后写出。
    float4 result = final_color / coc_weight.xxxx;
    img_output[pixel] = float4((result / max(1.0000000133514319600180897396058e-10f, result.w).xxxx).xyz, 1.0f);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
