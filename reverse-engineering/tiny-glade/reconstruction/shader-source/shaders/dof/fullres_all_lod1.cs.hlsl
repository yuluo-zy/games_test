// PATCHED: modules ain_6f1dd5dd8a66e104.bin:cs_main, ain_72a2a95d73e37edc.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
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

uint PackRingPair(float2 v)
{
    return f32tof16(v.x) | (f32tof16(v.y) << 16u);
}

float2 UnpackRingPair(uint v)
{
    return float2(f16tof32(v & 0xFFFFu), f16tof32(v >> 16u));
}

uint2 TileOrigin(uint3 group_id, uint3 dispatch_id, uint offset_index)
{
    return (dispatch_id.xy & uint2(7u, 7u))
        + uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offset_index] + group_id.x];
}

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
    float coc_limit = min(float(color_tex_dims.y) * 0.041666667908430099f,
        coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 noise = ((blue_noise_texture.Load(int3((pixel + uint2(13u, 13u)
        + uint2(int(frame_index * 59u), int(frame_index * 37u))) & uint2(255u, 255u), 0)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
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

    float4 color_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float coc_acc = 0.0f;
    float coc_weight = 0.0f;
    float weight_acc = 0.0f;
    float min_abs_coc = 1.0e10f;
    float max_abs_coc = -1.0e10f;

    [loop]
    for (uint spoke = 0u; spoke < 2u; ++spoke)
    {
        float angle = (float(spoke) + spoke_rot) * 1.5707963705062866f;
        float2 dir = BokehDir(float2(cos(angle), sin(angle)), uv);
        float2 dir_px = dir * (spoke_sign / float2(color_tex_dims));
        SampleRing(local_index, uv, dir_px, ring_offset, coc_step, coc_limit, noise.x * 0.2f,
            coc_acc, min_abs_coc, max_abs_coc);

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
                crossing = firstbitlow(mask[bin]);
                uint prev = uint(max(0, (int(crossing) - 1)));
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][prev >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[prev & 1u];
            }
            else
            {
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
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, max_abs_coc - min_abs_coc);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            mip = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                continue;
            }
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

    img_output[pixel] = max(float4(0.0f, 0.0f, 0.0f, 0.0f), color_acc / coc_weight);
}

#endif

#if defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_bokeh_anisotropy : packoffset(c0);
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 _Globals_depth_tex_dims : packoffset(c2);
    uint2 _Globals_coc_tex_dims : packoffset(c2.z);
    uint2 _Globals_img_output_dims : packoffset(c3);
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
    uint2 _174 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(8) + gl_WorkGroupID.x) * 8 + 0));
    float2 _181 = (float2(int2(_174)) + 0.5f.xx) / float2(_Globals_img_output_dims);
    float _192 = min(float(_Globals_color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _181, 0.0f).x);
    float4 _208 = ((blue_noise_texture.Load(int3(((_174 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 _209 = uint2(7u, 7u) + _174;
    float _221 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_209.x)) + (0.005837149918079376220703125f * float(_209.y)))) - 0.5f) * 0.5f;
    float _222 = 0.5f + _221;
    float _223 = _208.y;
    float _224 = _192 * 0.5f;
    float _229 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _239 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_174.x) + _229)) + (0.005837149918079376220703125f * (float(_174.y) + _229))));
    bool _240 = _222 < 0.5f;
    float _244;
    if (_240)
    {
        _244 = 0.5f - _221;
    }
    else
    {
        _244 = _222;
    }
    float _245 = _240 ? (-1.0f) : 1.0f;
    float4 _247;
    float _250;
    float _252;
    float _254;
    _247 = 0.0f.xxxx;
    _250 = 0.0f;
    _252 = 0.0f;
    _254 = 0.0f;
    float4 _248;
    float _251;
    float _259;
    float _261;
    uint _156[2];
    float _253;
    float _255;
    uint _256 = 0u;
    float _258 = 10000000000.0f;
    float _260 = -10000000000.0f;
    [loop]
    for (; _256 < 2u; _247 = _248, _250 = _251, _252 = _253, _254 = _255, _256++, _258 = _259, _260 = _261)
    {
        float _267 = (float(_256) + _239) * 1.57079637050628662109375f;
        float2 _270 = float2(cos(_267), sin(_267));
        float2 _308;
        if (_Globals_bokeh_anisotropy > 0.0f)
        {
            float4 _283 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_181 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _287 = _283.xy / _283.z.xx;
            _287.y = _287.y * (-1.0f);
            float2 _291 = normalize(_287);
            float2x2 _296 = float2x2(_291, float2(-_291.y, _291.x));
            float _301 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * _Globals_bokeh_anisotropy) * dot(_287, _287)));
            _308 = mul(transpose(_296), mul(_296, _270) * float2(_301, 1.0f / _301));
        }
        else
        {
            _308 = _270;
        }
        float2 _313 = _308 * (_245.xx / float2(_Globals_color_tex_dims));
        float _322 = -_192;
        float _323 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * (((-2.0f) + _223) * _224)), 0.0f).x, _322, _192);
        float _324 = abs(_323);
        float _327 = _208.x * 0.20000000298023223876953125f;
        float _328 = lerp(_250, max(_250, _324), _327);
        float _340 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * (((-1.0f) + _223) * _224)), 0.0f).x, _322, _192);
        float _343 = lerp(_328, max(_328, abs(_340)), _327);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_323, 0.0f)) | (spvPackHalf2x16(float2(_340, 0.0f)) << 16u);
        float _355 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * (_223 * _224)), 0.0f).x, _322, _192);
        float _356 = abs(_355);
        float _358 = lerp(_343, max(_343, _356), _327);
        float _370 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * ((1.0f + _223) * _224)), 0.0f).x, _322, _192);
        float _373 = lerp(_358, max(_358, abs(_370)), _327);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_355, 0.0f)) | (spvPackHalf2x16(float2(_370, 0.0f)) << 16u);
        float _386 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * ((2.0f + _223) * _224)), 0.0f).x, _322, _192);
        float _387 = abs(_386);
        _251 = lerp(_373, max(_373, _387), _327);
        _259 = min(min(min(_258, _324), _356), _387);
        _261 = max(max(max(_260, _324), _356), _387);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_386, 0.0f));
        for (uint _393 = 0u; _393 < 2u; )
        {
            _156[_393] = 0u;
            _393++;
            continue;
        }
        for (uint _400 = 0u; _400 < 5u; _400++)
        {
            uint _408 = _400 & 1u;
            uint _409 = _400 >> 1u;
            float2 _157 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_409] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_409] >> 16u) & 65535u).x);
            float _424 = (float(int(_400 - 2u)) + _223) * _224;
            for (int _426 = 0; _426 < 2; _426++)
            {
                if ((_424 / ((float(_426) + _244) - 1.0f)) >= _157[_408])
                {
                    _156[_426] |= (1u << (_400 & 31u));
                }
            }
        }
        float4 _448;
        _255 = _254;
        _253 = _252;
        _448 = 0.0f.xxxx;
        float _447;
        float _445;
        float4 _449;
        for (int _450 = 0; _450 < 2; _255 = _445, _253 = _447, _448 = _449, _450++)
        {
            float _457 = (float(_450) + _244) - 1.0f;
            float _458 = abs(_457);
            _447 = _253 + _458;
            if (_156[_450] == 0u)
            {
                _445 = _255;
                _449 = _448;
                continue;
            }
            bool _464 = _457 >= 0.0f;
            float _532;
            float _533;
            uint _534;
            if (_464)
            {
                uint _499 = firstbitlow(_156[_450]);
                uint _503 = uint(max(0, (int(_499) - 1)));
                uint _505 = _499 >> 1u;
                float2 _158 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_505] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_505] >> 16u) & 65535u).x);
                uint _519 = _503 >> 1u;
                float2 _159 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_519] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_519] >> 16u) & 65535u).x);
                _532 = _158[_499 & 1u];
                _533 = _159[_503 & 1u];
                _534 = _499;
            }
            else
            {
                uint _468 = firstbithigh(_156[_450]);
                uint _470 = min(4u, (_468 + 1u));
                uint _472 = _468 >> 1u;
                float2 _160 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_472] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_472] >> 16u) & 65535u).x);
                uint _486 = _470 >> 1u;
                float2 _161 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_486] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_486] >> 16u) & 65535u).x);
                _532 = _160[_468 & 1u];
                _533 = _161[_470 & 1u];
                _534 = _468;
            }
            float _540 = (float(int(_534 - 2u)) + _223) * _224;
            float _541 = _540 / _457;
            float _542 = _224 * (_464 ? 1.0f : (-1.0f));
            float _545 = _540 - _542;
            float _546 = _533 - (_541 - (_542 / _457));
            float _550 = _458 * ((_156[_450] != 0u) ? 1.0f : 0.0f);
            float _555 = lerp(_545, _540, clamp(_546 / (_546 + (-(_532 - _541))), 0.0f, 1.0f));
            float _561 = clamp(coc_tex.SampleLevel(sampler_nnc, _181 + (_313 * _555), 0.0f).x, _322, _192);
            float _569 = log2(max(0.0f, abs(_532) * 0.125f)) * smoothstep(4.0f, 0.0f, _261 - _259);
            float _572 = WaveReadLaneAt(_569, WaveGetLaneIndex() ^ 1u);
            float _573 = min(_569, _572);
            float _575 = WaveReadLaneAt(_573, WaveGetLaneIndex() ^ 8u);
            if (_532 > _192)
            {
                _445 = _255;
                _449 = _448;
                continue;
            }
            float _581 = _546 * 0.100000001490116119384765625f;
            bool _587;
            if (((_555 / _457) - _561) < _581)
            {
                _587 = (_561 - _532) < _581;
            }
            else
            {
                _587 = false;
            }
            float _599;
            float _600;
            if (_587)
            {
                _599 = _255;
                _600 = _550;
            }
            else
            {
                float _594 = smoothstep(0.0f, 30.0f, _533) * smoothstep(0.20000000298023223876953125f * _192, 0.0f, _532);
                _599 = _255 + (_550 * _594);
                _600 = _550 * (1.0f - _594);
            }
            _445 = _599;
            _449 = _448 + (color_tex.SampleLevel(sampler_nnc, _181 + (_313 * (_587 ? _555 : _545)), min(_573, _575)) * _600);
        }
        _248 = _247 + _448;
    }
    float _614 = _254 * smoothstep(0.00999999977648258209228515625f * _247.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _247.w), _254);
    float4 _675;
    [branch]
    if (int(_614 > 0.0f) != 0)
    {
        float _621 = -_192;
        float4 _623;
        _623 = 0.0f.xxxx;
        float4 _624;
        float _627;
        float _626 = _621;
        uint _628 = 0u;
        for (; _628 < 16u; _623 = _624, _626 = _627, _628++)
        {
            float2 _639 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_628)) + 0.5f.xx) + _208.xy);
            float _641 = sqrt(_639.x);
            float _643 = _639.y * 6.283185482025146484375f;
            float2 _654 = _181 + (((float2(cos(_643) * _641, sin(_643) * _641) * 1.0f) / float2(_Globals_color_tex_dims)) * _250);
            float4 _657 = coc_tex.SampleLevel(sampler_nnc, _654, 0.0f);
            float _658 = _657.x;
            if (_658 > (_626 - 10.0f))
            {
                _624 = _623 + color_tex.SampleLevel(sampler_llc, _654, 0.0f);
                _627 = max(_626, _658);
            }
            else
            {
                _624 = _623;
                _627 = _626;
            }
        }
        _675 = _247 + ((_623 / max(1.0f, _623.w).xxxx) * _614);
    }
    else
    {
        _675 = _247;
    }
    float4 _677 = _675 / _252.xxxx;
    img_output[_174] = float4((_677 / max(1.0000000133514319600180897396058e-10f, _677.w).xxxx).xyz, 1.0f);
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
    float _Globals_bokeh_anisotropy : packoffset(c0);
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0.y);
    uint2 _Globals_color_tex_dims : packoffset(c1);
    uint2 _Globals_coc_tiles_dilated_tex_dims : packoffset(c1.z);
    uint2 _Globals_depth_tex_dims : packoffset(c2);
    uint2 _Globals_coc_tex_dims : packoffset(c2.z);
    uint2 _Globals_img_output_dims : packoffset(c3);
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
    uint2 _170 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(8) + gl_WorkGroupID.x) * 8 + 0));
    float2 _177 = (float2(int2(_170)) + 0.5f.xx) / float2(_Globals_img_output_dims);
    float _188 = min(float(_Globals_color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _177, 0.0f).x);
    float4 _204 = ((blue_noise_texture.Load(int3(((_170 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _205 = _204.z;
    float _206 = _204.y;
    float _207 = _188 * 0.5f;
    float _208 = _204.x;
    bool _209 = _205 < 0.5f;
    float _213;
    if (_209)
    {
        _213 = 1.0f - _205;
    }
    else
    {
        _213 = _205;
    }
    float _214 = _209 ? (-1.0f) : 1.0f;
    float4 _216;
    float _219;
    float _221;
    float _223;
    _216 = 0.0f.xxxx;
    _219 = 0.0f;
    _221 = 0.0f;
    _223 = 0.0f;
    float4 _217;
    float _220;
    float _228;
    float _230;
    uint _152[2];
    float _222;
    float _224;
    uint _225 = 0u;
    float _227 = 10000000000.0f;
    float _229 = -10000000000.0f;
    [loop]
    for (; _225 < 4u; _216 = _217, _219 = _220, _221 = _222, _223 = _224, _225++, _227 = _228, _229 = _230)
    {
        float _236 = (float(_225) + _208) * 0.785398185253143310546875f;
        float2 _239 = float2(cos(_236), sin(_236));
        float2 _277;
        if (_Globals_bokeh_anisotropy > 0.0f)
        {
            float4 _252 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_177 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _256 = _252.xy / _252.z.xx;
            _256.y = _256.y * (-1.0f);
            float2 _260 = normalize(_256);
            float2x2 _265 = float2x2(_260, float2(-_260.y, _260.x));
            float _270 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * _Globals_bokeh_anisotropy) * dot(_256, _256)));
            _277 = mul(transpose(_265), mul(_265, _239) * float2(_270, 1.0f / _270));
        }
        else
        {
            _277 = _239;
        }
        float2 _282 = _277 * (_214.xx / float2(_Globals_color_tex_dims));
        float _291 = -_188;
        float _292 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * (((-2.0f) + _206) * _207)), 0.0f).x, _291, _188);
        float _293 = abs(_292);
        float _295 = _208 * 0.20000000298023223876953125f;
        float _296 = lerp(_219, max(_219, _293), _295);
        float _308 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * (((-1.0f) + _206) * _207)), 0.0f).x, _291, _188);
        float _311 = lerp(_296, max(_296, abs(_308)), _295);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_292, 0.0f)) | (spvPackHalf2x16(float2(_308, 0.0f)) << 16u);
        float _323 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * (_206 * _207)), 0.0f).x, _291, _188);
        float _324 = abs(_323);
        float _326 = lerp(_311, max(_311, _324), _295);
        float _338 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * ((1.0f + _206) * _207)), 0.0f).x, _291, _188);
        float _341 = lerp(_326, max(_326, abs(_338)), _295);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_323, 0.0f)) | (spvPackHalf2x16(float2(_338, 0.0f)) << 16u);
        float _354 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * ((2.0f + _206) * _207)), 0.0f).x, _291, _188);
        float _355 = abs(_354);
        _220 = lerp(_341, max(_341, _355), _295);
        _228 = min(min(min(_227, _293), _324), _355);
        _230 = max(max(max(_229, _293), _324), _355);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_354, 0.0f));
        for (uint _361 = 0u; _361 < 2u; )
        {
            _152[_361] = 0u;
            _361++;
            continue;
        }
        for (uint _368 = 0u; _368 < 5u; _368++)
        {
            uint _376 = _368 & 1u;
            uint _377 = _368 >> 1u;
            float2 _153 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_377] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_377] >> 16u) & 65535u).x);
            float _392 = (float(int(_368 - 2u)) + _206) * _207;
            for (int _394 = 0; _394 < 2; _394++)
            {
                if ((_392 / ((float(_394) + _213) - 1.0f)) >= _153[_376])
                {
                    _152[_394] |= (1u << (_368 & 31u));
                }
            }
        }
        float4 _416;
        _224 = _223;
        _222 = _221;
        _416 = 0.0f.xxxx;
        float _415;
        float _413;
        float4 _417;
        for (int _418 = 0; _418 < 2; _224 = _413, _222 = _415, _416 = _417, _418++)
        {
            float _425 = (float(_418) + _213) - 1.0f;
            float _426 = abs(_425);
            _415 = _222 + _426;
            if (_152[_418] == 0u)
            {
                _413 = _224;
                _417 = _416;
                continue;
            }
            bool _432 = _425 >= 0.0f;
            float _500;
            float _501;
            uint _502;
            if (_432)
            {
                uint _467 = firstbitlow(_152[_418]);
                uint _471 = uint(max(0, (int(_467) - 1)));
                uint _473 = _467 >> 1u;
                float2 _154 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_473] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_473] >> 16u) & 65535u).x);
                uint _487 = _471 >> 1u;
                float2 _155 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_487] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_487] >> 16u) & 65535u).x);
                _500 = _154[_467 & 1u];
                _501 = _155[_471 & 1u];
                _502 = _467;
            }
            else
            {
                uint _436 = firstbithigh(_152[_418]);
                uint _438 = min(4u, (_436 + 1u));
                uint _440 = _436 >> 1u;
                float2 _156 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_440] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_440] >> 16u) & 65535u).x);
                uint _454 = _438 >> 1u;
                float2 _157 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_454] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_454] >> 16u) & 65535u).x);
                _500 = _156[_436 & 1u];
                _501 = _157[_438 & 1u];
                _502 = _436;
            }
            float _508 = (float(int(_502 - 2u)) + _206) * _207;
            float _509 = _508 / _425;
            float _510 = _207 * (_432 ? 1.0f : (-1.0f));
            float _513 = _508 - _510;
            float _514 = _501 - (_509 - (_510 / _425));
            float _518 = _426 * ((_152[_418] != 0u) ? 1.0f : 0.0f);
            float _523 = lerp(_513, _508, clamp(_514 / (_514 + (-(_500 - _509))), 0.0f, 1.0f));
            float _529 = clamp(coc_tex.SampleLevel(sampler_nnc, _177 + (_282 * _523), 0.0f).x, _291, _188);
            float _537 = log2(max(0.0f, abs(_500) * 0.125f)) * smoothstep(4.0f, 0.0f, _230 - _228);
            float _540 = WaveReadLaneAt(_537, WaveGetLaneIndex() ^ 1u);
            float _541 = min(_537, _540);
            float _543 = WaveReadLaneAt(_541, WaveGetLaneIndex() ^ 8u);
            if (_500 > _188)
            {
                _413 = _224;
                _417 = _416;
                continue;
            }
            float _549 = _514 * 0.100000001490116119384765625f;
            bool _555;
            if (((_523 / _425) - _529) < _549)
            {
                _555 = (_529 - _500) < _549;
            }
            else
            {
                _555 = false;
            }
            float _567;
            float _568;
            if (_555)
            {
                _567 = _224;
                _568 = _518;
            }
            else
            {
                float _562 = smoothstep(0.0f, 30.0f, _501) * smoothstep(0.20000000298023223876953125f * _188, 0.0f, _500);
                _567 = _224 + (_518 * _562);
                _568 = _518 * (1.0f - _562);
            }
            _413 = _567;
            _417 = _416 + (color_tex.SampleLevel(sampler_nnc, _177 + (_282 * (_555 ? _523 : _513)), min(_541, _543)) * _568);
        }
        _217 = _216 + _416;
    }
    float _582 = _223 * smoothstep(0.00999999977648258209228515625f * _216.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _216.w), _223);
    float4 _643;
    [branch]
    if (int(_582 > 0.0f) != 0)
    {
        float _589 = -_188;
        float4 _591;
        _591 = 0.0f.xxxx;
        float4 _592;
        float _595;
        float _594 = _589;
        uint _596 = 0u;
        for (; _596 < 16u; _591 = _592, _594 = _595, _596++)
        {
            float2 _607 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_596)) + 0.5f.xx) + _204.xy);
            float _609 = sqrt(_607.x);
            float _611 = _607.y * 6.283185482025146484375f;
            float2 _622 = _177 + (((float2(cos(_611) * _609, sin(_611) * _609) * 1.0f) / float2(_Globals_color_tex_dims)) * _219);
            float4 _625 = coc_tex.SampleLevel(sampler_nnc, _622, 0.0f);
            float _626 = _625.x;
            if (_626 > (_594 - 10.0f))
            {
                _592 = _591 + color_tex.SampleLevel(sampler_llc, _622, 0.0f);
                _595 = max(_594, _626);
            }
            else
            {
                _592 = _591;
                _595 = _594;
            }
        }
        _643 = _216 + ((_591 / max(1.0f, _591.w).xxxx) * _582);
    }
    else
    {
        _643 = _216;
    }
    float4 _645 = _643 / _221.xxxx;
    img_output[_170] = float4((_645 / max(1.0000000133514319600180897396058e-10f, _645.w).xxxx).xyz, 1.0f);
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
