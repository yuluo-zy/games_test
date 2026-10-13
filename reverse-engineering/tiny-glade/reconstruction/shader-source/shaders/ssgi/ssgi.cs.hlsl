// ==RECONSTRUCTED== shaders/ssgi/ssgi.cs.hlsl | F-D
// Evidence: _ssgi_ssgi.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23; SHA-256
//   66e61114e7e782b18c01a857168ac52b587b1a063635a084e2af6b5caebef57d).
//   bindings: sampler_nnc s2, $Globals b13 (depth_tex_dims/half_view_normal_tex_dims/
//   output_tex_dims), depth_tex t14, half_view_normal_tex t15, output_tex u16,
//   frame_constants b0 space1; LocalSize [8,8,1], entry cs_main.
//   Horizon-based GTAO (XeGTAO.hlsl) over the half-res view-normal + depth pair:
//   space-filling-curve pixel jitter, 2 slices x 3 golden-ratio taps, distance
//   fade on the reconstructed view-space samples, integrated into an AO scalar.
//   Denoise / view-space helpers live in ssgi/XeGTAO.hlsl and ssgi/vaShared.hlsl.
// Status: O (structure/constants from disassembly; bindings verbatim);
//         I (function/var names — inlined by dxc).

#include "inc/common.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"
#include "ssgi/XeGTAO.hlsl"
#include "ssgi/vaShared.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
RWTexture2D<half> output_tex : register(u16, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 output_res = float2(output_tex_dims);
    float2 uv = pixel_to_uv(int2(dispatchThreadID.xy), output_res);

    // Space-filling-curve pixel index (bit-insertion + quadrant swap) selects a
    // per-pixel, per-frame GTAO jitter rotation.
    uint pixel_index = 0u;
    uint sfc_x = dispatchThreadID.x;
    uint sfc_y = dispatchThreadID.y;
    for (uint mask = 32u; mask > 0u; mask /= 2u)
    {
        uint xb = (sfc_x & mask) > 0u ? 1u : 0u;
        uint yb = (sfc_y & mask) > 0u ? 1u : 0u;
        pixel_index += (mask * mask) * ((3u * xb) ^ yb);
        if (yb == 0u)
        {
            if (xb == 1u)
            {
                uint prev_y = sfc_y;
                sfc_y = 63u - sfc_x;
                sfc_x = 63u - prev_y;
            }
        }
    }

    float2 jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(pixel_index + (288u * (frame_index % 64u)))) + 0.5f);
    float jitter_x = jitter.x;
    float jitter_y = jitter.y;

    // Center view-space position of this pixel.
    float4 view_homo = mul(view_constants[0].sample_to_view, float4(uv_to_sample(uv) * float2(2.0f, -2.0f), depth_tex.Load(uint3(dispatchThreadID.xy, 0u)).x, 1.0f));
    float view_z = (view_homo.xyz / view_homo.w).z;
    float focal = (-0.5f / view_z) * view_constants[0].view_to_clip[1][1];

    float2 inv_res = 1.0f / output_res;
    float2 view_to_sample_inv = float2(1.0f / view_constants[0].view_to_sample[0][0], 1.0f / view_constants[0].view_to_sample[1][1]);
    float2 clip_pix_scale = view_to_sample_inv * float2(2.0f, -2.0f);
    float2 clip_pix_bias = view_to_sample_inv * float2(-1.0f, 1.0f);

    float3 view_normal = normalize(half_view_normal_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz * float3(1.0f, 1.0f, -1.0f));

    float2 pixel_center_uv = (float2(dispatchThreadID.xy) + 0.5f) * inv_res;

    float4 gathered = depth_tex.GatherRed(sampler_nnc, float2(dispatchThreadID.xy) * inv_res);
    for (uint g = 0u; g < 4u; g++)
    {
        gathered[g] = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], gathered[g]);
    }

    float view_depth = gathered.y * 0.99919998645782470703125f;

    float3 view_pos;
    view_pos.xy = ((clip_pix_scale * pixel_center_uv) + clip_pix_bias) * view_depth;
    view_pos.z = view_depth;
    float3 view_dir = normalize(-view_pos);

    float radius_pixels = (-80.0f) * focal;
    float distance_fade = ((0.11249999701976776123046875f / focal) / (0.012500000186264514923095703125f / focal)) + 1.0f;

    float ao_radius = (0.125f / focal) / ((float2(view_depth, view_depth) * (clip_pix_scale * inv_res)).x);
    float radius_dist = 1.2999999523162841796875f / ao_radius;

    float ao = clamp((10.0f - ao_radius) * 0.00999999977648258209228515625f, 0.0f, 1.0f) * 0.5f;

    // Shared view-space sample position (rebuilt per tap).
    float3 sample_pos;

    for (float slice = 0.0f; slice < 1.0f; slice += 1.0f)
    {
        float angle = (slice + jitter_x) * 3.1415927410125732421875f;
        float cos_angle = cos(angle);
        float sin_angle = sin(angle);

        float2 slice_dir = float2(cos_angle, -sin_angle) * ao_radius;
        float3 slice_dir3 = float3(cos_angle, sin_angle, 0.0f);
        float3 h_proj = slice_dir3 - (view_dir * dot(slice_dir3, view_dir));
        float3 h_perp = normalize(cross(h_proj, view_dir));
        float3 n_proj = view_normal - (h_perp * dot(view_normal, h_perp));
        float n_len = length(n_proj);
        float ndv = clamp(dot(n_proj, view_dir) / n_len, 0.0f, 1.0f);
        float abs_ndv = abs(ndv);
        float sign_factor = float(int(sign(dot(h_proj, n_proj))));
        float horizon0 = sign_factor * XeGTAO_Internal_FastACos(abs_ndv);

        float cos_h2 = cos(horizon0 + 1.57079637050628662109375f);
        float cos_h1 = cos(horizon0 - 1.57079637050628662109375f);
        float h1 = cos_h1;
        float h2 = cos_h2;

        [unroll]
        for (float tap = 0.0f; tap < 3.0f; tap += 1.0f)
        {
            float2 sample_offset = slice_dir * (pow((tap + frac(jitter_y + ((slice + (tap * 3.0f)) * 0.61803400516510009765625f))) * 0.3333333432674407958984375f, 2.0f) + radius_dist);
            float mip_level = clamp(log2(length(sample_offset)), 0.0f, 0.0f);
            float2 rounded = round(sample_offset) * inv_res;

            float2 sample_uv_a = pixel_center_uv + rounded;
            float view_z_a = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_a, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_a) + clip_pix_bias) * view_z_a;
            sample_pos.z = view_z_a;
            float3 delta_a = sample_pos - view_pos;
            float len_a = length(delta_a);

            float2 sample_uv_b = pixel_center_uv - rounded;
            float view_z_b = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_b, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_b) + clip_pix_bias) * view_z_b;
            sample_pos.z = view_z_b;
            float3 delta_b = sample_pos - view_pos;
            float len_b = length(delta_b);

            h1 = max(h1, lerp(cos_h1, dot(delta_b / len_b, view_dir), clamp((len_b * radius_pixels) + distance_fade, 0.0f, 1.0f)));
            h2 = max(h2, lerp(cos_h2, dot(delta_a / len_a, view_dir), clamp((len_a * radius_pixels) + distance_fade, 0.0f, 1.0f)));
        }

        // horizon0 + HALF_PI accumulator (h2) feeds the *2 term; horizon0 - HALF_PI (h1) feeds the *-2 term.
        float abs_h1 = abs(h1);
        float acos_h1 = XeGTAO_Internal_FastACos(abs_h1);
        float horizon_angle_neg = (h1 >= 0.0f) ? acos_h1 : (3.14159297943115234375f - acos_h1);

        float abs_h2 = abs(h2);
        float acos_h2 = XeGTAO_Internal_FastACos(abs_h2);
        float horizon_angle_pos = (h2 >= 0.0f) ? acos_h2 : (3.14159297943115234375f - acos_h2);

        float half_w2 = horizon_angle_neg * (-2.0f);
        float sin_horizon0 = sin(horizon0);
        float half_w1 = 2.0f * horizon_angle_pos;

        ao += (lerp(n_len, 1.0f, 0.0500000007450580596923828125f) * (0.25f * (((ndv + (half_w2 * sin_horizon0)) - cos(half_w2 - horizon0)) + ((ndv + (half_w1 * sin_horizon0)) - cos(half_w1 - horizon0)))));
    }

    output_tex[dispatchThreadID.xy] = max(0.02999999932944774627685546875f, pow(ao, 1.0f));
}
