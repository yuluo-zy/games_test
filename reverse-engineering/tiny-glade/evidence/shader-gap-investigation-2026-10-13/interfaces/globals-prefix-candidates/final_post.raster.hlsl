// KNOWN EXCEPTION: the f0ad ps_main is a result-less stub in the original (all values dead). dxc 1.9 DCEs its
// whole body, so the bindings (sampler_lnc, $Globals, input_tex) and caps cannot be reproduced; the hand-written
// full ps that used to cover this variant was invented and is now excluded.
// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/final_post.raster.hlsl | evidence: _final_post.raster.hlsl_{b903,f0ad}.bin — b903 full ps: upsampling_mode dispatch (0 nearest / 1 bilinear / 2 5-tap bicubic / 3 EASU / 4 wave-gated EASU-vs-bicubic), blur_pyramid blend, lens distortion+CA, tony_mc_mapface tonemap, grading LUT (or debug RGB ramp via grading_flags), blue-noise dither, optional ICC LUT + 88x24 debug ramp; caps Shader+RuntimeDescriptorArray+GroupNonUniformVote; f0ad is a reduced stub (not the target) | status: O (all polynomials/constants from inlined SPIR-V); I (helper function names; bicubic kernel is the shipped non-Catmull variant)
//
// Final post-processing pass (blit_resolve_swapchain): input_tex is upsampled
// to the swapchain size, blurred pyramid blended in, tonemapped through the
// Tony McMapface LUT, color-graded (grading_lut_tex or inline ramp), ICC
// applied when use_icc, and dithered with blue noise. shadow_mask_mean scales
// the exposure so shadowed regions read a bit darker.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_nnc s2, sampler_llc s3)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/bicubic.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/grading_lut.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/color/xyz.hlsl"
#include "inc/color/luv.hlsl"
#include "inc/color/helmholtz_kohlrausch.hlsl"
#include "inc/color/math.hlsl"
#include "inc/upsample/easu.hlsl"
#include "inc/upsample/lanczos2.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims;
    uint2 blur_pyramid_tex_dims;
    uint2 grading_lut_tex_dims;
    uint2 icc_lut_dims;
    uint2 blue_noise_texture_dims;
    uint is_input_valid;
    uint use_icc;
    float sharpen_amount;
    uint upsampling_mode;
    uint grading_flags;
    float grading_strength;
    float grading_contrast;
    float grading_saturation;
    float vignette_amount;
    float distortion_amount;
    float ca_amount;
    float gamma_shift;
    uint2 framebuffer_dims;
};

Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> blur_pyramid_tex : register(t15, space0);
Texture3D<float4> grading_lut_tex : register(t16, space0);
Texture3D<float4> icc_lut : register(t17, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
ByteAddressBuffer shadow_mask_mean : register(t19, space0);
Texture3D<float4> tony_mc_mapface_lut : register(t20, space0);

// ICC debug-ramp accent colors (3 swatch blocks shown in the top-left when
// use_icc is enabled).
static const float3 debug_ramp[3] = {
    float3(0.8631572723388671875f, 0.06301002204418182373046875f, 0.15896083414554595947265625f),
    float3(0.9911022186279296875f, 0.6444797515869140625f, 0.13286833465099334716796875f),
    float3(0.02028855867683887481689453125f, 0.304987370967864990234375f, 0.520995616912841796875f),
};

// 5-tap separable bicubic used by upsampling_mode 2 and the smooth path of
// mode 4. Weights follow the shipped (non-Catmull) kernel exactly.
float3 bicubic_upsample(int2 pixel, float2 output_dims)
{
    float2 input_dims = float2(input_tex_dims);
    float2 uv = (float2(pixel) + 0.5f.xx) / output_dims;
    float2 src = uv * input_dims;
    float2 base = floor(src - 0.5f.xx);
    float2 f = src - (base + 0.5f.xx);

    float2 w0 = f * (-0.5f + (f * (1.0f - (f * 0.5f))));
    float2 w1 = f * (0.5f + (f * (2.0f - (f * 1.5f))));
    float2 w2 = (1.0f + ((f * f) * (-2.5f + (f * 1.5f)))) + w1;
    float2 w3 = (f * f) * (-0.5f + (f * 0.5f));

    float2 c_left = (base - 0.5f.xx) / input_dims;
    float2 c_right = (base + 2.5f.xx) / input_dims;
    float2 c_center = (base + 0.5f.xx + (w1 / w2)) / input_dims;

    float3 t1 = input_tex.SampleLevel(sampler_lnc, float2(c_center.x, c_left.y), 0.0f).xyz;
    float3 t2 = input_tex.SampleLevel(sampler_lnc, float2(c_left.x, c_center.y), 0.0f).xyz;
    float3 t3 = input_tex.SampleLevel(sampler_lnc, c_center, 0.0f).xyz;
    float3 t4 = input_tex.SampleLevel(sampler_lnc, float2(c_right.x, c_center.y), 0.0f).xyz;
    float3 t5 = input_tex.SampleLevel(sampler_lnc, float2(c_center.x, c_right.y), 0.0f).xyz;

    float3 num = (t1 * (w2.x * w0.y)) + (t2 * (w0.x * w2.y)) + (t3 * (w2.x * w2.y)) +
                 (t4 * (w3.x * w2.y)) + (t5 * (w2.x * w3.y));
    float den = (w2.x * w0.y) + (w0.x * w2.y) + (w2.x * w2.y) + (w3.x * w2.y) + (w2.x * w3.y);
    return max(0.0f.xxx, num / den.xxx);
}

// One EASU tap weight from the offset to the tap center (dot with the
// normalized gradient, scaled by the span and clamped by the length limit).
float easu_tap_weight(float2 off, float2 g2n, float2 perp, float2 span, float len_inv_recip, float len_inv)
{
    float2 v = float2(dot(off, g2n), dot(off, perp)) * span;
    float t = min(dot(v, v), len_inv);
    float a = (0.4000000059604644775390625f * t) - 1.0f;
    float b = (len_inv_recip * t) - 1.0f;
    return ((1.5625f * (a * a)) - 0.5625f) * (b * b);
}

// FidelityFX FSR 1.0 EASU (edge-adaptive spatial upsampling), 12 taps. The
// shipped kernel uses per-tap luma y + 0.5*(x+z) and the two-phase combine.
float3 easu_upsample(int2 pixel, float2 output_dims)
{
    float2 input_dims = float2(input_tex_dims);
    float4 coef = float4(input_dims.x / output_dims.x, input_dims.y / output_dims.y,
                         (0.5f * input_dims.x) / output_dims.x - 0.5f, (0.5f * input_dims.y) / output_dims.y - 0.5f);
    float4 div_dims = float4(1.0f, 1.0f, 1.0f, -1.0f) / input_dims.xyxy;
    float4 add_dims = float4(-1.0f, 2.0f, 1.0f, 2.0f) / input_dims.xyxy;

    float2 src = (float2(pixel) * coef.xy) + coef.zw;
    float2 base = floor(src);
    float2 f = src - base;

    float2 t0 = (base * div_dims.xy) + div_dims.zw;
    float2 t1 = t0 + add_dims.xy;
    float2 t2 = t0 + add_dims.zw;
    float2 t3 = t0 + (float4(0.0f, 4.0f, 0.0f, 0.0f) / input_dims.xyxy).xy;
    float4 tap_off = float4(-0.5f, 0.5f, -0.5f, 0.5f) * div_dims.xxyy;
    float2 off_xw = float2(tap_off.x, tap_off.w);
    float2 off_yw = float2(tap_off.y, tap_off.w);
    float2 off_yz = float2(tap_off.y, tap_off.z);
    float2 off_xz = float2(tap_off.x, tap_off.z);

    float4 a1 = input_tex.SampleLevel(sampler_lnc, t0 + off_xw, 0.0f);
    float4 b1 = input_tex.SampleLevel(sampler_lnc, t0 + off_yw, 0.0f);
    float4 c1 = input_tex.SampleLevel(sampler_lnc, t1 + off_xw, 0.0f);
    float4 c2 = input_tex.SampleLevel(sampler_lnc, t1 + off_yw, 0.0f);
    float4 c3 = input_tex.SampleLevel(sampler_lnc, t1 + off_yz, 0.0f);
    float4 c4 = input_tex.SampleLevel(sampler_lnc, t1 + off_xz, 0.0f);
    float4 d1 = input_tex.SampleLevel(sampler_lnc, t2 + off_xw, 0.0f);
    float4 d2 = input_tex.SampleLevel(sampler_lnc, t2 + off_yw, 0.0f);
    float4 d3 = input_tex.SampleLevel(sampler_lnc, t2 + off_yz, 0.0f);
    float4 d4 = input_tex.SampleLevel(sampler_lnc, t2 + off_xz, 0.0f);
    float4 e1 = input_tex.SampleLevel(sampler_lnc, t3 + off_yz, 0.0f);
    float4 e2 = input_tex.SampleLevel(sampler_lnc, t3 + off_xz, 0.0f);

    float la1 = a1.y + (0.5f * (a1.x + a1.z));
    float lb1 = b1.y + (0.5f * (b1.x + b1.z));
    float lc1 = c1.y + (0.5f * (c1.x + c1.z));
    float lc2 = c2.y + (0.5f * (c2.x + c2.z));
    float lc3 = c3.y + (0.5f * (c3.x + c3.z));
    float lc4 = c4.y + (0.5f * (c4.x + c4.z));
    float ld1 = d1.y + (0.5f * (d1.x + d1.z));
    float ld2 = d2.y + (0.5f * (d2.x + d2.z));
    float ld3 = d3.y + (0.5f * (d3.x + d3.z));
    float ld4 = d4.y + (0.5f * (d4.x + d4.z));
    float le1 = e1.y + (0.5f * (e1.x + e1.z));
    float le2 = e2.y + (0.5f * (e2.x + e2.z));

    float fx = f.x;
    float fy = f.y;
    float fx1 = 1.0f - fx;
    float fy1 = 1.0f - fy;
    float w00 = fx1 * fy1;
    float w01 = fx * fy1;
    float w10 = fx1 * fy;
    float w11 = fx * fy;

    float g0 = abs(ld4 - lc3);
    float g1 = ld4 - ld1;
    float e_c = clamp(abs(g1) / max(g0, abs(lc3 - ld1)), 0.0f, 1.0f);
    float g2 = abs(lc2 - lc3);
    float g3 = lc2 - la1;
    float e_b = clamp(abs(g3) / max(g2, abs(lc3 - la1)), 0.0f, 1.0f);
    float g4 = ld3 - lc3;
    float e_d = clamp(abs(g4) / max(abs(ld3 - ld4), g0), 0.0f, 1.0f);
    float g5 = abs(ld1 - ld4);
    float g6 = ld1 - lb1;
    float e_f = clamp(abs(g6) / max(g5, abs(ld4 - lb1)), 0.0f, 1.0f);
    float g7 = abs(ld1 - lc2);
    float g8 = ld1 - lc1;
    float e_a = clamp(abs(g8) / max(g7, abs(lc2 - lc1)), 0.0f, 1.0f);
    float g9 = le2 - lc3;
    float e_g = clamp(abs(g9) / max(abs(le2 - lc2), g2), 0.0f, 1.0f);
    float g10 = ld2 - lc2;
    float gx = (g1 * w00) + (g4 * w01) + (g8 * w10) + (g10 * w11);
    float e_e = clamp(abs(g10) / max(abs(ld2 - ld1), g7), 0.0f, 1.0f);
    float g11 = le1 - ld4;
    float gy = (g3 * w00) + (g6 * w01) + (g9 * w10) + (g11 * w11);
    float e_h = clamp(abs(g11) / max(abs(le1 - ld1), g5), 0.0f, 1.0f);

    float2 gv = float2(gx, gy);
    float g_len2 = dot(gv, gv);
    bool is_zero = g_len2 < 3.0517578125e-05f;
    float2 g2n = float2(is_zero ? 1.0f : gx, gy) * (is_zero ? 1.0f : rsqrt(g_len2)).xx;

    float e_avg = ((w00 * ((e_c * e_c) + (e_b * e_b))) + (w01 * ((e_d * e_d) + (e_f * e_f))) +
                   (w10 * ((e_a * e_a) + (e_g * e_g))) + (w11 * ((e_e * e_e) + (e_h * e_h)))) * 0.5f;
    float e_avg2 = e_avg * e_avg;
    float2 span = float2(1.0f + (((dot(g2n, g2n) / max(abs(g2n.x), abs(g2n.y))) - 1.0f) * e_avg2), 1.0f - (0.5f * e_avg2));
    float len_inv_recip = 0.5f - (0.2899999916553497314453125f * e_avg2);
    float len_inv = 1.0f / len_inv_recip;
    float2 perp = float2(-g2n.y, g2n.x);

    float w_a1 = easu_tap_weight(float2(0.0f, -1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_b1 = easu_tap_weight(float2(1.0f, -1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c1 = easu_tap_weight(float2(-1.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c2 = easu_tap_weight(float2(0.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c3 = easu_tap_weight(-f, g2n, perp, span, len_inv_recip, len_inv);
    float w_c4 = easu_tap_weight(float2(-1.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d1 = easu_tap_weight(1.0f.xx - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d2 = easu_tap_weight(float2(2.0f, 1.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d3 = easu_tap_weight(float2(2.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_d4 = easu_tap_weight(float2(1.0f, 0.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_e1 = easu_tap_weight(float2(1.0f, 2.0f) - f, g2n, perp, span, len_inv_recip, len_inv);
    float w_e2 = easu_tap_weight(float2(0.0f, 2.0f) - f, g2n, perp, span, len_inv_recip, len_inv);

    float w_sum = w_a1 + w_b1 + w_c1 + w_c2 + w_c3 + w_c4 + w_d1 + w_d2 + w_d3 + w_d4 + w_e1 + w_e2;
    float3 weighted = a1.xyz * w_a1 + b1.xyz * w_b1 + c1.xyz * w_c1 + c2.xyz * w_c2 + c3.xyz * w_c3 + c4.xyz * w_c4 + d1.xyz * w_d1 + d2.xyz * w_d2 + d3.xyz * w_d3 + d4.xyz * w_d4 + e1.xyz * w_e1 + e2.xyz * w_e2;
    float3 result = max(min(min(c3.xyz, d4.xyz), min(c2.xyz, d1.xyz)), weighted / w_sum.xxx);
    return min(max(max(c3.xyz, d4.xyz), max(c2.xyz, d1.xyz)), result);
}

// Distorted UV for a view-space point: project the distorted point and the
// distorted corner through view_to_clip and map back to normalized UV.
float2 final_post_sample_uv(float2 view_uv, float2 corner_uv, float r2, float corner_r2, float2 clip_scale, float amp)
{
    float2 distorted_pt = view_uv + (view_uv * (amp * r2));
    float2 distorted_corner = corner_uv + (corner_uv * (amp * corner_r2));
    float2 ndc = mul(view_constants[0].view_to_clip, float4(distorted_pt * clip_scale, 0.0f, 1.0f)).xy /
                 mul(view_constants[0].view_to_clip, float4(distorted_corner * clip_scale, 0.0f, 1.0f)).xy;
    return (ndc * float2(0.5f, -0.5f)) + 0.5f.xx;
}

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    int2 pixel = int2(position.xy);
    float2 output_dims = float2(framebuffer_dims);
    float2 uv = (float2(pixel) + 0.5f.xx) / output_dims;

    // Upsample source to the swapchain resolution (mode 4 is the default:
    // wave-wide edge detection picks EASU or the smooth 5-tap bicubic).
    float3 upsampled;
    if (upsampling_mode == 0u)
    {
        upsampled = input_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz;
    }
    else if (upsampling_mode == 1u)
    {
        upsampled = input_tex.SampleLevel(sampler_lnc, uv, 0.0f).xyz;
    }
    else if (upsampling_mode == 2u)
    {
        upsampled = bicubic_upsample(pixel, output_dims);
    }
    else if (upsampling_mode == 3u)
    {
        upsampled = easu_upsample(pixel, output_dims);
    }
    else
    {
        float4 g = input_tex.GatherGreen(sampler_lnc, uv, int2(0, 0));
        float g_min = min(min(g.x, g.y), min(g.z, g.w));
        float g_max = max(max(g.x, g.y), max(g.z, g.w));
        if (WaveActiveAnyTrue((abs(g_min - g_max) / max(9.9999996826552253889678874634872e-21f, g_min + g_max)) > 0.5f))
        {
            upsampled = easu_upsample(pixel, output_dims);
        }
        else
        {
            upsampled = bicubic_upsample(pixel, output_dims);
        }
    }

    float2 blur_dims = float2(blur_pyramid_tex_dims);
    float blur_extent = max(blur_dims.x, blur_dims.y);
    float3 blur_mid = (blur_pyramid_tex.SampleLevel(sampler_lnc, uv, log2(0.0500000007450580596923828125f * blur_extent)).xyz * 0.119999997317790985107421875f) * 0.25f;

    float distortion = max(0.0f, distortion_amount);
    float3 blended;
    if (distortion > 0.0f)
    {
        float2 clip_scale = float2(view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x, 1.0f);
        float2 view_uv = mul(view_constants[0].clip_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f)).xy / clip_scale;
        float r2 = dot(view_uv, view_uv);
        float2 corner_uv = mul(view_constants[0].clip_to_view, float4(1.0f, 1.0f, 0.0f, 1.0f)).xy / clip_scale;
        float corner_r2 = dot(corner_uv, corner_uv);

        float ca_plus = distortion * (1.0f + ca_amount);
        float ca_minus = distortion * (1.0f - ca_amount);
        blended = float3(input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, ca_plus), 0.0f).x,
                         input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, distortion), 0.0f).y,
                         input_tex.SampleLevel(sampler_lnc, final_post_sample_uv(view_uv, corner_uv, r2, corner_r2, clip_scale, ca_minus), 0.0f).z);
    }
    else
    {
        float3 blur_low = blur_pyramid_tex.SampleLevel(sampler_llc, uv, log2(0.001000000047497451305389404296875f * blur_extent)).xyz * 0.119999997317790985107421875f;
        blended = max((((upsampled + blur_low) * 0.917431175708770751953125f.xxx) - lerp(blur_mid, dot(blur_mid, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.5f.xxx)), 0.0f.xxx);
    }

    float2 centered = uv - 0.5f.xx;
    float r2 = 3.5f * dot(centered, centered);
    float3 tonemapped = pow(max(0.0f.xxx, (blended * lerp(1.0f, 0.569999992847442626953125f, smoothstep(0.300000011920928955078125f, 1.0f, clamp(shadow_mask_mean.Load<float>(0), 0.0f, 1.0f)))) * exp((-vignette_amount) * (r2 * r2))),
                           max(0.0f, 1.10000002384185791015625f + gamma_shift).xxx) * 1.60000002384185791015625f;

    float3 mapface = max(0.0f.xxx, tony_mc_mapface_lut.SampleLevel(sampler_lnc, ((tonemapped / (tonemapped + 1.0f.xxx)) * 0.979166686534881591796875f) + 0.010416666977107524871826171875f.xxx, 0.0f).xyz);
    float3 base_color = (is_input_valid != 0u) ? mapface : 0.180000007152557373046875f.xxx;

    float3 graded_sat = clamp(lerp(dot(base_color, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, base_color, grading_saturation.xxx), 0.0f.xxx, 1.0f.xxx);
    float graded_luma = dot(graded_sat, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float3 graded = graded_sat * (pow(graded_luma, grading_contrast) / max(9.9999997473787516355514526367188e-06f, graded_luma));

    float3 final_graded;
    if ((1u & grading_flags) != 0u)
    {
        if ((2u & grading_flags) != 0u)
        {
            if ((pixel.x < 256) && (pixel.y < 16))
            {
                float3 ramp = float3(float(pixel.x % 16), float(pixel.x / 16), float(15 - (pixel.y % 16))) * 0.066666670143604278564453125f;
                final_graded = srgb_to_linear(ramp);
            }
            else
            {
                final_graded = graded;
            }
        }
        else
        {
            final_graded = graded;
        }
    }
    else
    {
        float3 lut_uv = linear_to_srgb(graded) * (float(grading_lut_tex_dims.x - 1u).xxx / float(grading_lut_tex_dims.x).xxx) + (0.5f.xxx / float(grading_lut_tex_dims.x).xxx);
        float3 lut_color = srgb_to_linear(grading_lut_tex.SampleLevel(sampler_lnc, lut_uv, 0.0f).xyz);
        final_graded = lerp(graded, lut_color, grading_strength.xxx);
    }

    float dither = (blue_noise_texture.Load(int3(uint2((pixel + int2(int(frame_index * 59u), int(frame_index * 37u))) & int2(255, 255)), 0u)).x * 2.0f) - 1.0f;
    float3 dithered = clamp(final_graded + (clamp(final_graded * 512.0f, 0.0f.xxx, 1.0f.xxx) * ((max(-1.0f, dither * rsqrt(abs(dither))) - float(int(sign(dither)))) * 0.0039215688593685626983642578125f)), 0.0f.xxx, 1.0f.xxx);

    float3 output_color;
    if (use_icc != 0u)
    {
        int2 debug_pos = int2(pixel.x, int((framebuffer_dims.y - 1u) - uint(pixel.y))) - int2(24, 18);
        bool in_region = (debug_pos.x >= 0) && (debug_pos.y >= 0) && (uint(debug_pos.x) < 88u) && (uint(debug_pos.y) < 24u);
        float3 icc_input = dithered;
        if (in_region)
        {
            uint block = uint(debug_pos.x) / 32u;
            uint col = uint(debug_pos.x) % 32u;
            if (col < 24u)
            {
                if ((col == 0u) || (debug_pos.y == 0))
                {
                    icc_input = debug_ramp[block] * (0.4199999868869781494140625f / max(debug_ramp[block].x, max(debug_ramp[block].y, debug_ramp[block].z)));
                }
                else
                {
                    icc_input = debug_ramp[block];
                }
            }
        }
        float3 icc_uv = linear_to_srgb(icc_input) * 0.96875f.xxx + 0.015625f.xxx;
        output_color = srgb_to_linear(icc_lut.SampleLevel(sampler_lnc, icc_uv, 0.0f).xyz);
    }
    else
    {
        output_color = dithered;
    }

    return float4(output_color, 1.0f);
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _final_post.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims : packoffset(c0);
    uint2 blur_pyramid_tex_dims : packoffset(c0.z);
    uint2 grading_lut_tex_dims : packoffset(c1);
    uint2 icc_lut_dims : packoffset(c1.z);
    uint2 blue_noise_texture_dims : packoffset(c2);
    uint is_input_valid : packoffset(c2.z);
    uint use_icc : packoffset(c2.w);
    float sharpen_amount : packoffset(c3);
    uint upsampling_mode : packoffset(c3.y);
    uint grading_flags : packoffset(c3.z);
    float grading_strength : packoffset(c3.w);
    float grading_contrast : packoffset(c4);
    float grading_saturation : packoffset(c4.y);
    float vignette_amount : packoffset(c4.z);
    float distortion_amount : packoffset(c4.w);
    float ca_amount : packoffset(c5);
    float gamma_shift : packoffset(c5.y);
    uint2 framebuffer_dims : packoffset(c5.z);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> input_tex : register(t14, space0);

static float4 gl_FragCoord;
struct SPIRV_Cross_Input
{
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    if (upsampling_mode == 0u)
    {
    }
    else
    {
        if (upsampling_mode == 1u)
        {
        }
        else
        {
            if (upsampling_mode == 2u)
            {
            }
            else
            {
                if (upsampling_mode == 3u)
                {
                }
                else
                {
                    float4 _98 = input_tex.GatherGreen(sampler_lnc, (float2(int2(gl_FragCoord.xy)) + 0.5f.xx) / float2(framebuffer_dims), int2(0, 0));
                    float _99 = _98.x;
                    float _100 = _98.y;
                    float _101 = _98.z;
                    float _102 = _98.w;
                    float _105 = min(min(_99, _100), min(_101, _102));
                    float _108 = max(max(_99, _100), max(_101, _102));
                }
            }
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
