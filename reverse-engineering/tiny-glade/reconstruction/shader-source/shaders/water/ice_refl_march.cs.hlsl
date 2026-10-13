// PATCHED: modules ch.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/ice_refl_march.cs.hlsl
// SOURCE PATH : shaders/water/ice_refl_march.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refl_march.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [64,1,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — ice reflection march: per-reflected-pixel ray setup,
//   Hilbert-curve-ordered blue-noise reflection sampling (8 tries), half-res
//   depth march with bracketed refinement, history write or ray emission into
//   the software-RT ray buffer (morton-count bookkeeping), refl_hit/radiance
//   writes.
// MERGE (water-ice review water/ice_refl_march.cs.hlsl): A base adopted wholesale;
//   folded in the B/bin accept-gate polarity fix — the bin accepts the sample when
//   `reflect(basis, v).z < 0` (B matches), whereas A inverted it (retried on
//   reflect.z<0). The Hilbert index + retry-on-bad structure are A's (match the bin).

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
// inc/blue_noise.hlsl declares blue_noise_texture at t14; the water/ice RT family
// binds it at t15 (consumers embed no blue_noise.hlsl include). Declared locally.
#include "inc/hammersley.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/rt.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
// terrain_heightmap.inc omitted (reconstructed inc references a consumer-declared texture
// that this shader never binds; original embedded it but carries no binding).
#include "inc/material_eval.inc.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/history_valid.hlsl"
#include "ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 ice_roughness_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
};

ByteAddressBuffer ice_refl_count_buffer : register(t25, space0);
ByteAddressBuffer ice_refl_px_buffer : register(t26, space0);
RWByteAddressBuffer __alias_ice_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_ice_rt_ray_buffer : register(u34, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
Texture2D<float4> ice_roughness_tex : register(t30, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> refl_hit_tex : register(u31, space0);
[[vk::image_format("r32ui")]] RWTexture2D<uint> refl_hit_radiance_tex : register(u32, space0);

// Tangent-space coordinate swap (x, y, z) -> (x, -z, y); see ice_refr_resolve.
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// Hilbert-curve index of a half-res pixel (x, y) within a 64x64 block.
uint hilbert_index(uint x, uint y)
{
    uint acc = 0u;
    uint xi = x;
    uint yi = y;
    for (uint s = 32u; s > 0u; s /= 2u)
    {
        uint bx = uint((xi & s) > 0u);
        uint by = uint((yi & s) > 0u);
        acc += (s * s) * ((3u * bx) ^ by);
        if (by == 0u)
        {
            if (bx == 1u)
            {
                xi = 63u - xi;
                yi = 63u - yi;
            }
        }
    }
    return acc;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (gtid.x >= ice_refl_count_buffer.Load<uint>(12))
    {
        return;
    }

    uint px = ice_refl_px_buffer.Load<uint>(gtid.x * 4u + 0u);
    uint px_x = px & 65535u;
    uint px_y = (px >> 16u) & 65535u;
    uint2 pixel = uint2(px_x, px_y);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(pixel)) + 0.5f) / half_dims) - 0.5f) * float2(2.0f, -2.0f);

    float3 view_ray = normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam4.xyz / cam4.w;
    float3 ground = cam_pos + (view_ray * ((-cam_pos.y) / view_ray.y));
    float4 g4 = mul(view_constants[0].view_to_sample, mul(view_constants[0].world_to_view, float4(ground, 1.0f)));
    float4 pos_vs = float4(ndc, (g4.xyz / g4.w).z, 1.0f);

    uint frame_mod = frame_index % 64u;
    float frame_rand = 5.588237762451171875f * float(frame_mod);
    float h = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px_y) + frame_rand))));

    float4 rough4 = ice_roughness_tex.Load(int3(pixel * uint2(2u, 2u), 0u));
    float rough = rough4.x;
    float rough2 = max(0.000600000028498470783233642578125f, rough * rough);

    float3 view_dir = swap_yz(-view_ray);

    // Reflection-direction importance sampling (up to 8 tries to stay above horizon).
    float3 refl_dir = 0.0f;
    uint k = 0u;
    for (;;)
    {
        if (k < 8u)
        {
            uint curve = hilbert_index(pixel.x, pixel.y) + (288u * frame_mod);
            float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(curve)) + 0.5f);

            float3 v = 0.0f;
            bool rejected = true;
            while (true)
            {
                float3 z = normalize(float3(rough2 * view_dir.x, rough2 * view_dir.y, view_dir.z));
                float3 binorm = (z.z < 0.99989998340606689453125f) ? normalize(cross(float3(0.0f, 0.0f, 1.0f), z)) : float3(1.0f, 0.0f, 0.0f);
                float r = sqrt(bn.x);
                float theta = bn.y * 3.1415927410125732421875f;
                float c1 = r * cos(theta);
                float s1 = r * sin(theta);
                float hh = 0.5f * (1.0f + z.z);
                float c2 = (1.0f - hh) * sqrt(1.0f - (c1 * c1)) + (hh * s1);
                float3 dir = (binorm * c1) + (cross(z, binorm) * c2) + (z * sqrt(max(0.0f, (1.0f - (c1 * c1)) - (c2 * c2))));
                float3 dir2 = normalize(float3(rough2 * dir.x, rough2 * dir.y, max(0.0f, dir.z)));

                bool bad = (dir2.z <= 9.9999997473787516355514526367188e-06f)
                    || (reflect(-view_dir, dir2).z <= 9.9999997473787516355514526367188e-06f);
                if (bad || view_dir.z <= 9.9999997473787516355514526367188e-06f)
                {
                    v = 0.0f;
                    rejected = true;
                    break;
                }
                v = dir2;
                rejected = false;
                break;
            }
            // Bin accept-gate polarity: accept when reflect(basis, v).z < 0
            // (a bad sample has v=0 -> reflect.z=0 -> retry below).
            if (!rejected && reflect(view_dir, v).z < 0.0f)
            {
                refl_dir = v;
                break;
            }
            k++;
            continue;
        }
        else
        {
            refl_dir = float3(0.0f, 1.0f, 0.0f);
            break;
        }
    }

    float3 refl = -reflect(-view_ray, swap_yz(refl_dir));
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }

    // March the reflected ray against the half-res depth buffer.
    float3 march_start = pos_vs.xyz;
    float4 ref_end = pos_vs + mul(view_constants[0].view_to_clip, mul(view_constants[0].world_to_view, float4(refl, 0.0f)));
    float3 ref_dir = (ref_end / (float((ref_end.w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(ref_end.w)))).xyz - march_start;
    bool3 neg = ref_dir < 0.0f;
    float3 t0 = (float3(neg.x ? 1.0f : -1.0f, neg.y ? 1.0f : -1.0f, neg.z ? 1.0f : -1.0f) - march_start) / ref_dir;
    float3 start = march_start + (ref_dir * max(0.0f, max(t0.x, t0.y)));
    float3 r2 = ref_dir - (start - march_start);
    bool3 pos3 = r2 >= 0.0f;
    float3 t1 = (float3(pos3.x ? 1.0f : -1.0f, pos3.y ? 1.0f : -1.0f, pos3.z ? 1.0f : -1.0f) - start) / r2;
    float3 end = start + (r2 * min(min(t1.x, t1.y), t1.z));

    float2 start_uv = (start.xy * float2(0.5f, -0.5f)) + 0.5f;
    float2 end_uv = (end.xy * float2(0.5f, -0.5f)) + 0.5f;

    bool hit = false;
    float hit_t = 0.0f;
    float2 hit_uv = 0.0f;

    uint steps = uint(int(max(2u, min(16u, uint(int(floor(length((end_uv - start_uv) * half_dims))))))));
    float step_count = (end.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
    float inv_steps = 1.0f / step_count;

    float prev_e = 0.0f;
    float prev_hit = 0.0f;
    float prev_diff = 0.0f;
    float prev_depth = 0.0f;
    float prev_ray = 0.0f;
    bool prev_under = false;
    float prev_t = 0.0f;

    if (steps > 0u)
    {
        float t_first = lerp(0.0f, 1.0f, pow(h * inv_steps, 2.0f));
        float3 sp = start + (r2 * t_first);
        float inv_z = 1.0f / sp.z;
        float d = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diff = (1.00000202655792236328125f / d) - inv_z;
        bool under = (diff < 0.0f);
        prev_e = under ? 0.0f : diff;
        prev_hit = under ? diff : 0.0f;
        prev_diff = under ? inv_z - (1.0f / d) : 0.0f;
        prev_depth = under ? view_constants[0].clip_to_view[3].z : 0.0f;
        prev_ray = under ? t_first : 0.0f;
        prev_under = under;
        prev_t = 0.0f;

        for (uint i = 1u; i < steps; ++i)
        {
            float tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + h) * inv_steps, 2.0f)));
            float3 sp2 = start + (r2 * tt);
            float inv_z2 = 1.0f / sp2.z;
            float d2 = half_depth_tex.SampleLevel(sampler_nnc, (sp2.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diff2 = (1.00000202655792236328125f / d2) - inv_z2;
            bool under2 = (diff2 < 0.0f);
            if (under2)
            {
                prev_hit = diff2;
                prev_diff = inv_z2 - (1.0f / d2);
                prev_depth = view_constants[0].clip_to_view[3].z;
                prev_ray = tt;
                prev_under = true;
                break;
            }
            prev_e = diff2;
            prev_under = under2;
            prev_t = tt;
        }
    }

    if (prev_under)
    {
        // Refine with a few bisection steps.
        float lo = prev_e;
        float hi = prev_ray;
        float d_lo = prev_hit;
        float d_hi = prev_diff;
        float ray_hi = prev_depth;
        float ray_lo = prev_under ? prev_t : 0.0f;
        float t_hi = prev_ray;
        for (uint j = 0u; j < 3u; ++j)
        {
            float mid = lo + t_hi;
            mid *= 0.5f;
            float3 sm = start + (r2 * mid);
            float inv_zm = 1.0f / sm.z;
            float dm = half_depth_tex.SampleLevel(sampler_nnc, (sm.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diffm = (1.00000202655792236328125f / dm) - inv_zm;
            bool under_m = (diffm < 0.0f);
            lo = under_m ? lo : mid;
            d_lo = under_m ? d_lo : diffm;
            d_hi = under_m ? diffm : d_hi;
            ray_hi = under_m ? ray_hi : (inv_zm - (1.0f / dm));
            t_hi = under_m ? mid : t_hi;
        }
        float interp = lerp(lo, t_hi, d_lo / (d_lo + (-d_hi)));
        float3 fp = start + (r2 * interp);
        float inv_zp = 1.0f / fp.z;
        float dp = half_depth_tex.SampleLevel(sampler_nnc, (fp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diffp = (1.00000202655792236328125f / dp) - inv_zp;
        bool converged = (abs(diffp) < (d_lo * 0.89999997615814208984375f));
        hit = converged;
        hit_t = converged ? interp : t_hi;
        hit_uv = converged ? lerp(start_uv, end_uv, interp) : lerp(start_uv, end_uv, prev_e);
    }
    else
    {
        hit = false;
        hit_t = prev_e;
        hit_uv = lerp(start_uv, end_uv, prev_e);
    }

    float dist;
    float3 history_color;
    if (hit)
    {
        float3 history;
        if (history_valid != 0u && frame_index != 0u)
        {
            history = taa_history_tex.SampleLevel(sampler_lnc,
                reprojection_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).xy + hit_uv, 0.0f).xyz;
        }
        else
        {
            history = 0.0f;
        }
        float4 wp = mul(view_constants[0].view_to_world, mul(view_constants[0].clip_to_view,
            float4(lerp(start, end, hit_t), 1.0f)));
        dist = distance(wp.xyz / wp.w, ground);
        history_color = history;
    }
    else
    {
        float4 wp = mul(view_constants[0].view_to_world,
            mul(view_constants[0].sample_to_view, float4(lerp(start, end, hit_t), 1.0f)));
        float3 wp_xyz = wp.xyz / wp.w;
        uint slot;
        __alias_ice_rt_count_buffer.InterlockedAdd(12, 1u, slot);
        float3 origin = wp_xyz - cam_pos;
        __alias_ice_rt_ray_buffer.Store<uint4>(slot * 16u + 0u,
            uint4(px_x + (px_y * 65536u),
                pack_half2x16(float2(origin.x, 0.0f)) | (pack_half2x16(float2(refl.x, 0.0f)) << 16u),
                pack_half2x16(float2(origin.y, 0.0f)) | (pack_half2x16(float2(refl.y, 0.0f)) << 16u),
                pack_half2x16(float2(origin.z, 0.0f)) | (pack_half2x16(float2(refl.z, 0.0f)) << 16u)));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_rt_count_buffer.InterlockedAdd(0, 1u, group);
        }
        dist = distance(wp_xyz, ground);
        history_color = 0.0f;
    }

    refl_hit_tex[pixel] = float2(dist, rough2);
    float3 clamped = clamp(history_color, 0.0f, 65408.0f);
    float scale = asfloat((asuint(max(max(1.52587890625e-05f, clamped.x), max(clamped.y, clamped.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(clamped + scale);
    refl_hit_radiance_tex[pixel] = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_ice_refl_march.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float _159;

cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_terrain_lighting_texture_dims : packoffset(c0);
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0.z);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_brdf_fg_lut_dims : packoffset(c2);
    uint _Globals_rt_bvh_node_count : packoffset(c2.z);
    uint _Globals_rt_triangle_count : packoffset(c2.w);
    uint _Globals_history_valid : packoffset(c3);
    uint2 _Globals_half_depth_tex_dims : packoffset(c3.y);
    uint2 _Globals_taa_history_tex_dims : packoffset(c4);
    uint2 _Globals_reprojection_tex_dims : packoffset(c4.z);
    uint2 _Globals_ice_roughness_tex_dims : packoffset(c5);
    uint2 _Globals_refl_hit_tex_dims : packoffset(c5.z);
    uint2 _Globals_refl_hit_radiance_tex_dims : packoffset(c6);
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

ByteAddressBuffer ice_refl_count_buffer : register(t25, space0);
ByteAddressBuffer ice_refl_px_buffer : register(t26, space0);
RWByteAddressBuffer __alias_ice_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_ice_rt_ray_buffer : register(u34, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
Texture2D<float4> ice_roughness_tex : register(t30, space0);
RWTexture2D<float2> refl_hit_tex : register(u31, space0);
RWTexture2D<uint> refl_hit_radiance_tex : register(u32, space0);

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
        if (gl_GlobalInvocationID.x >= ice_refl_count_buffer.Load<uint>(12))
        {
            break;
        }
        uint _187 = ice_refl_px_buffer.Load<uint>(gl_GlobalInvocationID.x * 4 + 0);
        uint _188 = _187 & 65535u;
        uint _190 = (_187 >> 16u) & 65535u;
        uint2 _191 = uint2(_188, _190);
        float2 _196 = float2(_Globals_half_depth_tex_dims);
        float2 _205 = (((float2(int2(_191)) + 0.5f.xx) / _196) - 0.5f.xx) * float2(2.0f, -2.0f);
        float _206 = _205.x;
        float3 _212 = normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(_206, _205.y, 0.0f, 1.0f))).xyz);
        float3 _213 = -_212;
        float4 _214 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 _218 = _214.xyz / _214.w.xxx;
        float3 _224 = _218 + (_212 * ((-_218).y / _212.y));
        float4 _234 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_224, 1.0f)));
        float4 _240 = float4(_206, _205.y, (_234.xyz / _234.w.xxx).z, 1.0f);
        uint _243 = frame_constants_frame_index % 64u;
        float _246 = 5.588237762451171875f * float(_243);
        float _255 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_188) + _246)) + (0.005837149918079376220703125f * (float(_190) + _246))));
        float4 _257 = ice_roughness_tex.Load(int3(_191 * uint2(2u, 2u), 0u));
        float _258 = _257.x;
        float _260 = max(0.000600000028498470783233642578125f, _258 * _258);
        float3 _261 = mul(_213, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
        float3 _383;
        uint _263 = 0u;
        for (;;)
        {
            if (_263 < 8u)
            {
                uint _275;
                _275 = 0u;
                uint _276;
                uint _271;
                uint _274;
                for (uint _270 = _190, _273 = _188, _277 = 32u; _277 > 0u; _270 = _271, _273 = _274, _275 = _276, _277 /= 2u)
                {
                    uint _284 = uint((_273 & _277) > 0u);
                    uint _287 = uint((_270 & _277) > 0u);
                    _276 = _275 + ((_277 * _277) * ((3u * _284) ^ _287));
                    if (_287 == 0u)
                    {
                        uint _300;
                        uint _301;
                        if (_284 == 1u)
                        {
                            _300 = 63u - _270;
                            _301 = 63u - _273;
                        }
                        else
                        {
                            _300 = _270;
                            _301 = _273;
                        }
                        _271 = _301;
                        _274 = _300;
                    }
                    else
                    {
                        _271 = _270;
                        _274 = _273;
                    }
                }
                float2 _307 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_275 + (288u * _243))) + 0.5f.xx);
                float3 _377;
                do
                {
                    float _315 = _261.z;
                    float3 _317 = normalize(float3(_260 * _261.x, _260 * _261.y, _315));
                    float _318 = _317.z;
                    float3 _325;
                    if (_318 < 0.99989998340606689453125f)
                    {
                        _325 = normalize(cross(float3(0.0f, 0.0f, 1.0f), _317));
                    }
                    else
                    {
                        _325 = float3(1.0f, 0.0f, 0.0f);
                    }
                    float _328 = sqrt(_307.x);
                    float _329 = _307.y * 3.1415927410125732421875f;
                    float _331 = _328 * cos(_329);
                    float _335 = 0.5f * (1.0f + _318);
                    float _338 = 1.0f - (_331 * _331);
                    float _342 = ((1.0f - _335) * sqrt(_338)) + (_335 * (_328 * sin(_329)));
                    float3 _351 = ((_325 * _331) + (cross(_317, _325) * _342)) + (_317 * sqrt(max(0.0f, _338 - (_342 * _342))));
                    float3 _359 = normalize(float3(_260 * _351.x, _260 * _351.y, max(0.0f, _351.z)));
                    bool _369;
                    if (!(_359.z <= 9.9999997473787516355514526367188e-06f))
                    {
                        _369 = reflect(-_261, _359).z <= 9.9999997473787516355514526367188e-06f;
                    }
                    else
                    {
                        _369 = true;
                    }
                    bool _374;
                    if (!_369)
                    {
                        _374 = _315 <= 9.9999997473787516355514526367188e-06f;
                    }
                    else
                    {
                        _374 = true;
                    }
                    if (_374)
                    {
                        _377 = 0.0f.xxx;
                        break;
                    }
                    _377 = _359;
                    break;
                } while(false);
                if (reflect(_261, _377).z < 0.0f)
                {
                    _383 = _377;
                    break;
                }
                _263++;
                continue;
            }
            else
            {
                _383 = float3(0.0f, 1.0f, 0.0f);
                break;
            }
        }
        float3 _386 = -reflect(_213, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), _383));
        float _387 = _386.y;
        float3 _393;
        if (_387 < 0.0f)
        {
            float3 _392 = _386;
            _392.y = -_387;
            _393 = _392;
        }
        else
        {
            _393 = _386;
        }
        float4x4 _456;
        float3 _395 = _240.xyz;
        float4 _404 = _240 + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(_393, 0.0f)));
        float _405 = _404.w;
        float3 _414 = (_404 / (float((_405 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(_405))).xxxx).xyz;
        float3 _415 = _414 - _395;
        bool3 _416 = bool3(_415.x < 0.0f.xxx.x, _415.y < 0.0f.xxx.y, _415.z < 0.0f.xxx.z);
        float3 _419 = (float3(_416.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _416.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _416.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _395) / _415;
        float3 _425 = _395 + (_415 * max(0.0f, max(_419.x, _419.y)));
        float3 _426 = _414 - _425;
        bool3 _427 = bool3(_426.x >= 0.0f.xxx.x, _426.y >= 0.0f.xxx.y, _426.z >= 0.0f.xxx.z);
        float3 _430 = (float3(_427.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, _427.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, _427.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - _425) / _426;
        float3 _437 = _425 + (_426 * min(min(_430.x, _430.y), _430.z));
        bool _638;
        float _639;
        float2 _640;
        do
        {
            float2 _442 = (_425.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float2 _445 = (_437.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            _456 = frame_constants_view_constants[0u].clip_to_view;
            uint _458 = uint(int(max(2u, min(16u, uint(int(floor(length((_445 - _442) * _196))))))));
            float _551;
            float _619;
            float _620;
            float _621;
            float _622;
            bool _623;
            do
            {
                float3 _463 = _437 - _425;
                float _471;
                if (_437.z > 9.5367431640625e-07f)
                {
                    _471 = float(_458);
                }
                else
                {
                    _471 = max(1.0f, float(_458) - 1.0f);
                }
                float _472 = 1.0f / _471;
                float _545;
                float _546;
                float _547;
                float _548;
                float _549;
                bool _550;
                if (_458 > 0u)
                {
                    float _478 = lerp(0.0f, 1.0f, pow(_255 * _472, 2.0f));
                    float3 _480 = _425 + (_463 * _478);
                    float _485 = 1.0f / _480.z;
                    float4 _488 = half_depth_tex.SampleLevel(sampler_nnc, (_480.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _489 = _488.x;
                    float _492 = (1.00000202655792236328125f / _489) - _485;
                    bool _495 = (_492 < 0.0f) ? true : false;
                    float _538;
                    float _539;
                    float _540;
                    float _541;
                    float _542;
                    bool _543;
                    float _544;
                    if (_495)
                    {
                        _538 = 0.0f;
                        _539 = _492;
                        _540 = _485 - (1.0f / _489);
                        _541 = _456[3].z;
                        _542 = _478;
                        _543 = _495;
                        _544 = 0.0f;
                    }
                    else
                    {
                        float _503;
                        float _507;
                        _503 = _492;
                        _507 = _478;
                        float _504;
                        bool _506;
                        float _508;
                        float _533;
                        float _534;
                        float _535;
                        float _536;
                        bool _537;
                        uint _500 = 1u;
                        bool _505 = _495;
                        for (;;)
                        {
                            if (_500 < _458)
                            {
                                _508 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_500) + _255) * _472, 2.0f)));
                                float3 _518 = _425 + (_463 * _508);
                                float _523 = 1.0f / _518.z;
                                float4 _525 = half_depth_tex.SampleLevel(sampler_nnc, (_518.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                                float _526 = _525.x;
                                _504 = (1.00000202655792236328125f / _526) - _523;
                                _506 = (_504 < 0.0f) ? true : false;
                                if (_506)
                                {
                                    _533 = _504;
                                    _534 = _523 - (1.0f / _526);
                                    _535 = _456[3].z;
                                    _536 = _508;
                                    _537 = _506;
                                    break;
                                }
                                _500++;
                                _503 = _504;
                                _505 = _506;
                                _507 = _508;
                                continue;
                            }
                            else
                            {
                                _533 = 0.0f;
                                _534 = 0.0f;
                                _535 = 0.0f;
                                _536 = 1.0f;
                                _537 = _505;
                                break;
                            }
                        }
                        _538 = _503;
                        _539 = _533;
                        _540 = _534;
                        _541 = _535;
                        _542 = _536;
                        _543 = _537;
                        _544 = _507;
                    }
                    _545 = _538;
                    _546 = _539;
                    _547 = _540;
                    _548 = _541;
                    _549 = _542;
                    _550 = _543;
                    _551 = _544;
                }
                else
                {
                    _545 = 0.0f;
                    _546 = 0.0f;
                    _547 = 0.0f;
                    _548 = 0.0f;
                    _549 = 1.0f;
                    _550 = false;
                    _551 = 0.0f;
                }
                if (_550)
                {
                    float _556;
                    float _559;
                    float _561;
                    float _563;
                    float _565;
                    float _567;
                    _556 = _551;
                    _559 = _545;
                    _561 = _546;
                    _563 = _547;
                    _565 = _548;
                    _567 = _549;
                    for (uint _569 = 0u; _569 < 3u; )
                    {
                        float _573 = _556 + _567;
                        float _574 = _573 * 0.5f;
                        float3 _576 = _425 + (_463 * _574);
                        float _581 = 1.0f / _576.z;
                        float4 _584 = half_depth_tex.SampleLevel(sampler_nnc, (_576.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                        float _585 = _584.x;
                        float _588 = (1.00000202655792236328125f / _585) - _581;
                        bool _591 = (_588 < 0.0f) ? true : false;
                        _556 = _591 ? _556 : _574;
                        _559 = _591 ? _559 : _588;
                        _561 = _591 ? _588 : _561;
                        _563 = _591 ? (_581 - (1.0f / _585)) : _563;
                        _565 = _591 ? _456[3].z : _565;
                        _567 = _591 ? _574 : _567;
                        _569++;
                        continue;
                    }
                    float _595 = lerp(_556, _567, _559 / (_559 + (-_561)));
                    float3 _597 = _425 + (_463 * _595);
                    float _602 = 1.0f / _597.z;
                    float4 _605 = half_depth_tex.SampleLevel(sampler_nnc, (_597.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float _606 = _605.x;
                    float _609 = (1.00000202655792236328125f / _606) - _602;
                    bool _614 = (abs(_609) < (_559 * 0.89999997615814208984375f)) ? true : false;
                    _619 = _614 ? _595 : _567;
                    _620 = _614 ? _609 : _561;
                    _621 = _614 ? _456[3].z : _565;
                    _622 = _614 ? (_602 - (1.0f / _606)) : _563;
                    _623 = true;
                    break;
                }
                else
                {
                    _619 = _551;
                    _620 = _159;
                    _621 = _159;
                    _622 = _159;
                    _623 = false;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            bool _631;
            if (_623)
            {
                bool _630;
                if (_622 < _621)
                {
                    _630 = _620 < _621;
                }
                else
                {
                    _630 = false;
                }
                _631 = _630;
            }
            else
            {
                _631 = false;
            }
            if (_631)
            {
                _638 = true;
                _639 = _619;
                _640 = lerp(_442, _445, _619.xx);
                break;
            }
            _638 = false;
            _639 = _551;
            _640 = lerp(_442, _445, _551.xx);
            break;
        } while(false);
        float4 _646 = float4(lerp(_425, _437, _639.xxx), 1.0f);
        float _722;
        float3 _723;
        if (_638)
        {
            bool _704;
            if (_Globals_history_valid != 0u)
            {
                _704 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _704 = false;
            }
            float3 _714;
            if (_704)
            {
                _714 = taa_history_tex.SampleLevel(sampler_lnc, reprojection_tex.SampleLevel(sampler_nnc, _640, 0.0f).xy + _640, 0.0f).xyz;
            }
            else
            {
                _714 = 0.0f.xxx;
            }
            float4 _716 = mul(frame_constants_view_constants[0u].view_to_world, mul(_456, _646));
            _722 = distance(_716.xyz / _716.w.xxx, _224);
            _723 = _714;
        }
        else
        {
            float4 _651 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, _646));
            float3 _655 = _651.xyz / _651.w.xxx;
            uint _657;
            __alias_ice_rt_count_buffer.InterlockedAdd(12, 1u, _657);
            float3 _658 = _655 - _218;
            __alias_ice_rt_ray_buffer.Store<uint4>(_657 * 16 + 0, uint4(_188 + (_190 * 65536u), spvPackHalf2x16(float2(_658.x, 0.0f)) | (spvPackHalf2x16(float2(_393.x, 0.0f)) << 16u), spvPackHalf2x16(float2(_658.y, 0.0f)) | (spvPackHalf2x16(float2(_393.y, 0.0f)) << 16u), spvPackHalf2x16(float2(_658.z, 0.0f)) | (spvPackHalf2x16(float2(_393.z, 0.0f)) << 16u)));
            if ((_657 & 63u) == 0u)
            {
                uint _689;
                __alias_ice_rt_count_buffer.InterlockedAdd(0, 1u, _689);
            }
            _722 = distance(_655, _224);
            _723 = 0.0f.xxx;
        }
        refl_hit_tex[_191] = float2(_722, _260);
        float3 _726 = clamp(_723, 0.0f.xxx, 65408.0f.xxx);
        float _736 = asfloat((asuint(max(max(1.52587890625e-05f, _726.x), max(_726.y, _726.z))) + 125845504u) & 2139095040u);
        uint3 _739 = asuint(_726 + _736.xxx);
        refl_hit_radiance_tex[_191] = ((((asuint(_736) << 4u) + 268435456u) | (_739.z << 18u)) | (_739.y << 9u)) | (_739.x & 511u);
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
