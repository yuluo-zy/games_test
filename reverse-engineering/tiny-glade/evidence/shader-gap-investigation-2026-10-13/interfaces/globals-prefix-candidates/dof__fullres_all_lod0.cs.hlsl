// PATCHED: modules ain_6f1dd5dd8a66e104.bin:cs_main, ain_72a2a95d73e37edc.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)) || (defined(TGV_72a2a95d73e37edc) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/fullres_all_lod0.cs.hlsl | MERGE (dof-aa review, 2026-10-13): B base (loop structure) + A RWTexture2D<half4> img_output (Rgba16f). No 8/11 near-branch guard (correct for lod0). 6f1d noise scheme only (72a2 residual).
//   Full-res bokeh pass at LOD0 scale: 9-tap ring, 4 bins, no half-res upsample
//   blend (tile list offsets[1]).

#include "inc/common.hlsl"
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
#include "dof/multires.inc.hlsl"

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
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);

groupshared uint dof_ring_samples[64][5];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[1] + gl_WorkGroupID.x]);
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);

    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.25f;
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub = use_low ? (0.5f - sub) : sub2;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[4];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u; ring_bits[3] = 0u;
    float coc_min = 1e10f;
    float coc_max = -1e10f;

    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(view_constants[0].clip_to_view, float4((pixel_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.7f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        float lerp_gain = dither.x * 0.2f;

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc8)), lerp_gain);
        coc_min = min(min(min(min(min(coc_min, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8));
        coc_max = max(max(max(max(max(coc_max, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8));
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f));

        for (uint i = 0u; i < 4u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 9u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 4u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 4; ++slot)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.5f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float slot_depth = 0.0f;
        for (int slot = 0; slot < 4; slot_weight = depth_weight, slot_depth = slot_depth, slot_color = slot_color, ++slot)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.5f) - 1.0f;
            float abs_frac = abs(slot_frac);
            depth_weight = slot_depth + abs_frac;
            if (ring_bits[slot] == 0u)
            {
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc, hi_coc;
            uint tap_idx;
            if (positive)
            {
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = a[low_bit & 1u];
                hi_coc = b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(8u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = a[high_bit & 1u];
                hi_coc = b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 4u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * float(ring_bits[slot] != 0u);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            occlusion = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                continue;
            }
            float edge_eps = hi_edge * 0.1f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float4 ring_color;
            float ring_weight;
            // No 8/11 near-branch guard for lod0 (target): near -> slot_span/sample_weight.
            if (close_to_edge)
            {
                ring_color = slot_color;
                ring_weight = slot_span;
            }
            else
            {
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                ring_color = slot_color + (slot_span * depth_gate);
                ring_weight = slot_span * (1.0f - depth_gate);
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    // Background scatter (16 samples) when the foreground contribution is low.
    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = -coc_limit;
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 16u; ++s)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float bg_coc = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f).x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum += color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc = max(max_bg_coc, bg_coc);
            }
        }
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w)) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    float4 result = final_color / depth_weight;
    img_output[pixel] = float4((result / max(1e-10f, result.w)).xyz, 1.0f);
}

#endif

#if defined(TGV_6f1dd5dd8a66e104) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

groupshared uint ring_samples[64][5];

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
    uint2 _179 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(4) + gl_WorkGroupID.x) * 8 + 0));
    float2 _186 = (float2(int2(_179)) + 0.5f.xx) / float2(img_output_dims);
    float _197 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _186, 0.0f).x);
    float4 _213 = ((blue_noise_texture.Load(int3(((_179 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint2 _214 = uint2(7u, 7u) + _179;
    float _226 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_214.x)) + (0.005837149918079376220703125f * float(_214.y)))) - 0.5f) * 0.5f;
    float _227 = 0.5f + _226;
    float _228 = _213.y;
    float _229 = _197 * 0.25f;
    float _234 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _244 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_179.x) + _234)) + (0.005837149918079376220703125f * (float(_179.y) + _234))));
    bool _245 = _227 < 0.5f;
    float _249;
    if (_245)
    {
        _249 = 0.5f - _226;
    }
    else
    {
        _249 = _227;
    }
    float _250 = _245 ? (-1.0f) : 1.0f;
    float4 _252;
    float _255;
    float _257;
    float _259;
    _252 = 0.0f.xxxx;
    _255 = 0.0f;
    _257 = 0.0f;
    _259 = 0.0f;
    float4 _253;
    float _256;
    float _264;
    float _266;
    uint _161[4];
    float _258;
    float _260;
    uint _261 = 0u;
    float _263 = 10000000000.0f;
    float _265 = -10000000000.0f;
    [loop]
    for (; _261 < 2u; _252 = _253, _255 = _256, _257 = _258, _259 = _260, _261++, _263 = _264, _265 = _266)
    {
        float _272 = (float(_261) + _244) * 1.57079637050628662109375f;
        float2 _275 = float2(cos(_272), sin(_272));
        float2 _313;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _288 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_186 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _292 = _288.xy / _288.z.xx;
            _292.y = _292.y * (-1.0f);
            float2 _296 = normalize(_292);
            float2x2 _301 = float2x2(_296, float2(-_296.y, _296.x));
            float _306 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_292, _292)));
            _313 = mul(transpose(_301), mul(_301, _275) * float2(_306, 1.0f / _306));
        }
        else
        {
            _313 = _275;
        }
        float2 _318 = _313 * (_250.xx / float2(color_tex_dims));
        float _327 = -_197;
        float _328 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * (((-4.0f) + _228) * _229)), 0.0f).x, _327, _197);
        float _329 = abs(_328);
        float _332 = _213.x * 0.20000000298023223876953125f;
        float _333 = lerp(_255, max(_255, _329), _332);
        float _345 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * (((-3.0f) + _228) * _229)), 0.0f).x, _327, _197);
        float _348 = lerp(_333, max(_333, abs(_345)), _332);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_328, 0.0f)) | (spvPackHalf2x16(float2(_345, 0.0f)) << 16u);
        float _361 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * (((-2.0f) + _228) * _229)), 0.0f).x, _327, _197);
        float _362 = abs(_361);
        float _364 = lerp(_348, max(_348, _362), _332);
        float _376 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * (((-1.0f) + _228) * _229)), 0.0f).x, _327, _197);
        float _379 = lerp(_364, max(_364, abs(_376)), _332);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_361, 0.0f)) | (spvPackHalf2x16(float2(_376, 0.0f)) << 16u);
        float _391 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * (_228 * _229)), 0.0f).x, _327, _197);
        float _392 = abs(_391);
        float _394 = lerp(_379, max(_379, _392), _332);
        float _406 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * ((1.0f + _228) * _229)), 0.0f).x, _327, _197);
        float _409 = lerp(_394, max(_394, abs(_406)), _332);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_391, 0.0f)) | (spvPackHalf2x16(float2(_406, 0.0f)) << 16u);
        float _422 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * ((2.0f + _228) * _229)), 0.0f).x, _327, _197);
        float _423 = abs(_422);
        float _425 = lerp(_409, max(_409, _423), _332);
        float _437 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * ((3.0f + _228) * _229)), 0.0f).x, _327, _197);
        float _440 = lerp(_425, max(_425, abs(_437)), _332);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_422, 0.0f)) | (spvPackHalf2x16(float2(_437, 0.0f)) << 16u);
        float _453 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * ((4.0f + _228) * _229)), 0.0f).x, _327, _197);
        float _454 = abs(_453);
        _256 = lerp(_440, max(_440, _454), _332);
        _264 = min(min(min(min(min(_263, _329), _362), _392), _423), _454);
        _266 = max(max(max(max(max(_265, _329), _362), _392), _423), _454);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_453, 0.0f));
        for (uint _460 = 0u; _460 < 4u; )
        {
            _161[_460] = 0u;
            _460++;
            continue;
        }
        for (uint _467 = 0u; _467 < 9u; _467++)
        {
            uint _475 = _467 & 1u;
            uint _476 = _467 >> 1u;
            float2 _162 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_476] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_476] >> 16u) & 65535u).x);
            float _491 = (float(int(_467 - 4u)) + _228) * _229;
            for (int _493 = 0; _493 < 4; _493++)
            {
                if ((_491 / (((float(_493) + _249) * 0.5f) - 1.0f)) >= _162[_475])
                {
                    _161[_493] |= (1u << (_467 & 31u));
                }
            }
        }
        float4 _516;
        _260 = _259;
        _258 = _257;
        _516 = 0.0f.xxxx;
        float _515;
        float _513;
        float4 _517;
        for (int _518 = 0; _518 < 4; _260 = _513, _258 = _515, _516 = _517, _518++)
        {
            float _526 = ((float(_518) + _249) * 0.5f) - 1.0f;
            float _527 = abs(_526);
            _515 = _258 + _527;
            if (_161[_518] == 0u)
            {
                _513 = _260;
                _517 = _516;
                continue;
            }
            bool _533 = _526 >= 0.0f;
            float _601;
            float _602;
            uint _603;
            if (_533)
            {
                uint _568 = firstbitlow(_161[_518]);
                uint _572 = uint(max(0, (int(_568) - 1)));
                uint _574 = _568 >> 1u;
                float2 _163 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_574] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_574] >> 16u) & 65535u).x);
                uint _588 = _572 >> 1u;
                float2 _164 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_588] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_588] >> 16u) & 65535u).x);
                _601 = _163[_568 & 1u];
                _602 = _164[_572 & 1u];
                _603 = _568;
            }
            else
            {
                uint _537 = firstbithigh(_161[_518]);
                uint _539 = min(8u, (_537 + 1u));
                uint _541 = _537 >> 1u;
                float2 _165 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_541] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_541] >> 16u) & 65535u).x);
                uint _555 = _539 >> 1u;
                float2 _166 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_555] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_555] >> 16u) & 65535u).x);
                _601 = _165[_537 & 1u];
                _602 = _166[_539 & 1u];
                _603 = _537;
            }
            float _609 = (float(int(_603 - 4u)) + _228) * _229;
            float _610 = _609 / _526;
            float _611 = _229 * (_533 ? 1.0f : (-1.0f));
            float _614 = _609 - _611;
            float _615 = _602 - (_610 - (_611 / _526));
            float _619 = _527 * ((_161[_518] != 0u) ? 1.0f : 0.0f);
            float _624 = lerp(_614, _609, clamp(_615 / (_615 + (-(_601 - _610))), 0.0f, 1.0f));
            float _630 = clamp(coc_tex.SampleLevel(sampler_nnc, _186 + (_318 * _624), 0.0f).x, _327, _197);
            float _638 = log2(max(0.0f, abs(_601) * 0.125f)) * smoothstep(4.0f, 0.0f, _266 - _264);
            float _641 = WaveReadLaneAt(_638, WaveGetLaneIndex() ^ 1u);
            float _642 = min(_638, _641);
            float _644 = WaveReadLaneAt(_642, WaveGetLaneIndex() ^ 8u);
            if (_601 > _197)
            {
                _513 = _260;
                _517 = _516;
                continue;
            }
            float _650 = _615 * 0.100000001490116119384765625f;
            bool _656;
            if (((_624 / _526) - _630) < _650)
            {
                _656 = (_630 - _601) < _650;
            }
            else
            {
                _656 = false;
            }
            float _668;
            float _669;
            if (_656)
            {
                _668 = _260;
                _669 = _619;
            }
            else
            {
                float _663 = smoothstep(0.0f, 30.0f, _602) * smoothstep(0.20000000298023223876953125f * _197, 0.0f, _601);
                _668 = _260 + (_619 * _663);
                _669 = _619 * (1.0f - _663);
            }
            _513 = _668;
            _517 = _516 + (color_tex.SampleLevel(sampler_nnc, _186 + (_318 * (_656 ? _624 : _614)), min(_642, _644)) * _669);
        }
        _253 = _252 + _516;
    }
    float _683 = _259 * smoothstep(0.00999999977648258209228515625f * _252.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _252.w), _259);
    float4 _744;
    [branch]
    if (int(_683 > 0.0f) != 0)
    {
        float _690 = -_197;
        float4 _692;
        _692 = 0.0f.xxxx;
        float4 _693;
        float _696;
        float _695 = _690;
        uint _697 = 0u;
        for (; _697 < 16u; _692 = _693, _695 = _696, _697++)
        {
            float2 _708 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_697)) + 0.5f.xx) + _213.xy);
            float _710 = sqrt(_708.x);
            float _712 = _708.y * 6.283185482025146484375f;
            float2 _723 = _186 + (((float2(cos(_712) * _710, sin(_712) * _710) * 1.0f) / float2(color_tex_dims)) * _255);
            float4 _726 = coc_tex.SampleLevel(sampler_nnc, _723, 0.0f);
            float _727 = _726.x;
            if (_727 > (_695 - 10.0f))
            {
                _693 = _692 + color_tex.SampleLevel(sampler_llc, _723, 0.0f);
                _696 = max(_695, _727);
            }
            else
            {
                _693 = _692;
                _696 = _695;
            }
        }
        _744 = _252 + ((_692 / max(1.0f, _692.w).xxxx) * _683);
    }
    else
    {
        _744 = _252;
    }
    float4 _746 = _744 / _257.xxxx;
    img_output[_179] = float4((_746 / max(1.0000000133514319600180897396058e-10f, _746.w).xxxx).xyz, 1.0f);
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
// lifted from _dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

groupshared uint ring_samples[64][5];

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
    uint2 _175 = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer.Load<uint2>((tile_offsets_buffer.Load<uint>(4) + gl_WorkGroupID.x) * 8 + 0));
    float2 _182 = (float2(int2(_175)) + 0.5f.xx) / float2(img_output_dims);
    float _193 = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, _182, 0.0f).x);
    float4 _209 = ((blue_noise_texture.Load(int3(((_175 + uint2(13u, 13u)) + uint2(int2(int(frame_constants_frame_index * 59u), int(frame_constants_frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float _210 = _209.z;
    float _211 = _209.y;
    float _212 = _193 * 0.25f;
    float _213 = _209.x;
    bool _214 = _210 < 0.5f;
    float _218;
    if (_214)
    {
        _218 = 1.0f - _210;
    }
    else
    {
        _218 = _210;
    }
    float _219 = _214 ? (-1.0f) : 1.0f;
    float4 _221;
    float _224;
    float _226;
    float _228;
    _221 = 0.0f.xxxx;
    _224 = 0.0f;
    _226 = 0.0f;
    _228 = 0.0f;
    float4 _222;
    float _225;
    float _233;
    float _235;
    uint _157[4];
    float _227;
    float _229;
    uint _230 = 0u;
    float _232 = 10000000000.0f;
    float _234 = -10000000000.0f;
    [loop]
    for (; _230 < 4u; _221 = _222, _224 = _225, _226 = _227, _228 = _229, _230++, _232 = _233, _234 = _235)
    {
        float _241 = (float(_230) + _213) * 0.785398185253143310546875f;
        float2 _244 = float2(cos(_241), sin(_241));
        float2 _282;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 _257 = mul(frame_constants_view_constants[0u].clip_to_view, float4((_182 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 _261 = _257.xy / _257.z.xx;
            _261.y = _261.y * (-1.0f);
            float2 _265 = normalize(_261);
            float2x2 _270 = float2x2(_265, float2(-_265.y, _265.x));
            float _275 = lerp(0.699999988079071044921875f, 1.0f, exp(((-4.0f) * bokeh_anisotropy) * dot(_261, _261)));
            _282 = mul(transpose(_270), mul(_270, _244) * float2(_275, 1.0f / _275));
        }
        else
        {
            _282 = _244;
        }
        float2 _287 = _282 * (_219.xx / float2(color_tex_dims));
        float _296 = -_193;
        float _297 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * (((-4.0f) + _211) * _212)), 0.0f).x, _296, _193);
        float _298 = abs(_297);
        float _300 = _213 * 0.20000000298023223876953125f;
        float _301 = lerp(_224, max(_224, _298), _300);
        float _313 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * (((-3.0f) + _211) * _212)), 0.0f).x, _296, _193);
        float _316 = lerp(_301, max(_301, abs(_313)), _300);
        ring_samples[gl_LocalInvocationIndex][0u] = spvPackHalf2x16(float2(_297, 0.0f)) | (spvPackHalf2x16(float2(_313, 0.0f)) << 16u);
        float _329 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * (((-2.0f) + _211) * _212)), 0.0f).x, _296, _193);
        float _330 = abs(_329);
        float _332 = lerp(_316, max(_316, _330), _300);
        float _344 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * (((-1.0f) + _211) * _212)), 0.0f).x, _296, _193);
        float _347 = lerp(_332, max(_332, abs(_344)), _300);
        ring_samples[gl_LocalInvocationIndex][1u] = spvPackHalf2x16(float2(_329, 0.0f)) | (spvPackHalf2x16(float2(_344, 0.0f)) << 16u);
        float _359 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * (_211 * _212)), 0.0f).x, _296, _193);
        float _360 = abs(_359);
        float _362 = lerp(_347, max(_347, _360), _300);
        float _374 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * ((1.0f + _211) * _212)), 0.0f).x, _296, _193);
        float _377 = lerp(_362, max(_362, abs(_374)), _300);
        ring_samples[gl_LocalInvocationIndex][2u] = spvPackHalf2x16(float2(_359, 0.0f)) | (spvPackHalf2x16(float2(_374, 0.0f)) << 16u);
        float _390 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * ((2.0f + _211) * _212)), 0.0f).x, _296, _193);
        float _391 = abs(_390);
        float _393 = lerp(_377, max(_377, _391), _300);
        float _405 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * ((3.0f + _211) * _212)), 0.0f).x, _296, _193);
        float _408 = lerp(_393, max(_393, abs(_405)), _300);
        ring_samples[gl_LocalInvocationIndex][3u] = spvPackHalf2x16(float2(_390, 0.0f)) | (spvPackHalf2x16(float2(_405, 0.0f)) << 16u);
        float _421 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * ((4.0f + _211) * _212)), 0.0f).x, _296, _193);
        float _422 = abs(_421);
        _225 = lerp(_408, max(_408, _422), _300);
        _233 = min(min(min(min(min(_232, _298), _330), _360), _391), _422);
        _235 = max(max(max(max(max(_234, _298), _330), _360), _391), _422);
        ring_samples[gl_LocalInvocationIndex][4u] = spvPackHalf2x16(float2(_421, 0.0f));
        for (uint _428 = 0u; _428 < 4u; )
        {
            _157[_428] = 0u;
            _428++;
            continue;
        }
        for (uint _435 = 0u; _435 < 9u; _435++)
        {
            uint _443 = _435 & 1u;
            uint _444 = _435 >> 1u;
            float2 _158 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_444] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_444] >> 16u) & 65535u).x);
            float _459 = (float(int(_435 - 4u)) + _211) * _212;
            for (int _461 = 0; _461 < 4; _461++)
            {
                if ((_459 / (((float(_461) + _218) * 0.5f) - 1.0f)) >= _158[_443])
                {
                    _157[_461] |= (1u << (_435 & 31u));
                }
            }
        }
        float4 _484;
        _229 = _228;
        _227 = _226;
        _484 = 0.0f.xxxx;
        float _483;
        float _481;
        float4 _485;
        for (int _486 = 0; _486 < 4; _229 = _481, _227 = _483, _484 = _485, _486++)
        {
            float _494 = ((float(_486) + _218) * 0.5f) - 1.0f;
            float _495 = abs(_494);
            _483 = _227 + _495;
            if (_157[_486] == 0u)
            {
                _481 = _229;
                _485 = _484;
                continue;
            }
            bool _501 = _494 >= 0.0f;
            float _569;
            float _570;
            uint _571;
            if (_501)
            {
                uint _536 = firstbitlow(_157[_486]);
                uint _540 = uint(max(0, (int(_536) - 1)));
                uint _542 = _536 >> 1u;
                float2 _159 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_542] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_542] >> 16u) & 65535u).x);
                uint _556 = _540 >> 1u;
                float2 _160 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_556] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_556] >> 16u) & 65535u).x);
                _569 = _159[_536 & 1u];
                _570 = _160[_540 & 1u];
                _571 = _536;
            }
            else
            {
                uint _505 = firstbithigh(_157[_486]);
                uint _507 = min(8u, (_505 + 1u));
                uint _509 = _505 >> 1u;
                float2 _161 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_509] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_509] >> 16u) & 65535u).x);
                uint _523 = _507 >> 1u;
                float2 _162 = float2(spvUnpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_523] & 65535u).x, spvUnpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_523] >> 16u) & 65535u).x);
                _569 = _161[_505 & 1u];
                _570 = _162[_507 & 1u];
                _571 = _505;
            }
            float _577 = (float(int(_571 - 4u)) + _211) * _212;
            float _578 = _577 / _494;
            float _579 = _212 * (_501 ? 1.0f : (-1.0f));
            float _582 = _577 - _579;
            float _583 = _570 - (_578 - (_579 / _494));
            float _587 = _495 * ((_157[_486] != 0u) ? 1.0f : 0.0f);
            float _592 = lerp(_582, _577, clamp(_583 / (_583 + (-(_569 - _578))), 0.0f, 1.0f));
            float _598 = clamp(coc_tex.SampleLevel(sampler_nnc, _182 + (_287 * _592), 0.0f).x, _296, _193);
            float _606 = log2(max(0.0f, abs(_569) * 0.125f)) * smoothstep(4.0f, 0.0f, _235 - _233);
            float _609 = WaveReadLaneAt(_606, WaveGetLaneIndex() ^ 1u);
            float _610 = min(_606, _609);
            float _612 = WaveReadLaneAt(_610, WaveGetLaneIndex() ^ 8u);
            if (_569 > _193)
            {
                _481 = _229;
                _485 = _484;
                continue;
            }
            float _618 = _583 * 0.100000001490116119384765625f;
            bool _624;
            if (((_592 / _494) - _598) < _618)
            {
                _624 = (_598 - _569) < _618;
            }
            else
            {
                _624 = false;
            }
            float _636;
            float _637;
            if (_624)
            {
                _636 = _229;
                _637 = _587;
            }
            else
            {
                float _631 = smoothstep(0.0f, 30.0f, _570) * smoothstep(0.20000000298023223876953125f * _193, 0.0f, _569);
                _636 = _229 + (_587 * _631);
                _637 = _587 * (1.0f - _631);
            }
            _481 = _636;
            _485 = _484 + (color_tex.SampleLevel(sampler_nnc, _182 + (_287 * (_624 ? _592 : _582)), min(_610, _612)) * _637);
        }
        _222 = _221 + _484;
    }
    float _651 = _228 * smoothstep(0.00999999977648258209228515625f * _221.w, max(9.9999997473787516355514526367188e-06f, 0.20000000298023223876953125f * _221.w), _228);
    float4 _712;
    [branch]
    if (int(_651 > 0.0f) != 0)
    {
        float _658 = -_193;
        float4 _660;
        _660 = 0.0f.xxxx;
        float4 _661;
        float _664;
        float _663 = _658;
        uint _665 = 0u;
        for (; _665 < 16u; _660 = _661, _663 = _664, _665++)
        {
            float2 _676 = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_665)) + 0.5f.xx) + _209.xy);
            float _678 = sqrt(_676.x);
            float _680 = _676.y * 6.283185482025146484375f;
            float2 _691 = _182 + (((float2(cos(_680) * _678, sin(_680) * _678) * 1.0f) / float2(color_tex_dims)) * _224);
            float4 _694 = coc_tex.SampleLevel(sampler_nnc, _691, 0.0f);
            float _695 = _694.x;
            if (_695 > (_663 - 10.0f))
            {
                _661 = _660 + color_tex.SampleLevel(sampler_llc, _691, 0.0f);
                _664 = max(_663, _695);
            }
            else
            {
                _661 = _660;
                _664 = _663;
            }
        }
        _712 = _221 + ((_660 / max(1.0f, _660.w).xxxx) * _651);
    }
    else
    {
        _712 = _221;
    }
    float4 _714 = _712 / _226.xxxx;
    img_output[_175] = float4((_714 / max(1.0000000133514319600180897396058e-10f, _714.w).xxxx).xyz, 1.0f);
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
