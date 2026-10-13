// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water/ice_refl_resolve.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [8,8,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — ice reflection resolve: 8-tap spiral gather over
//   refl_hit_tex/refl_hit_radiance_tex with roughness/ray-length matched weights,
//   FrexpStructType quantization (curve.hlsl), history blend and ray-length output.

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/history_valid.hlsl"
#include "ice_shared.inc.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/curve.hlsl"
// NOTE: inc/blue_noise.hlsl declares blue_noise_texture at t14; the ice family
// binds it at t15 (its consumers embed no blue_noise.hlsl include + local decl).
// Declared locally below at t15; blue_noise.hlsl omitted (inc register issue).

cbuffer $Globals : register(b13, space0)
{
    uint2 brdf_fg_lut_dims;
    uint history_valid;
    uint2 blue_noise_texture_dims;
    uint2 depth_tex_dims;
    uint2 ice_roughness_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
    uint2 color_tex_dims;
    uint2 refl_refr_radiance_history_tex_dims;
    uint2 refl_refr_radiance_out_tex_dims;
    uint2 ray_len_history_tex_dims;
    uint2 ray_len_out_tex_dims;
    uint2 reprojection_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
Texture2D<float4> ice_roughness_tex : register(t17, space0);
Texture2D<float4> refl_hit_tex : register(t18, space0);
Texture2D<float4> refl_hit_radiance_tex : register(t19, space0);
RWTexture2D<float3> color_tex : register(u20, space0);
Texture2D<float4> refl_refr_radiance_history_tex : register(t21, space0);
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u22, space0);
Texture2D<float4> ray_len_history_tex : register(t23, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> ray_len_out_tex : register(u24, space0);

// Tangent-space coordinate swap (x, y, z) -> (x, -z, y); see ice_refr_resolve.
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (ice_roughness_tex.Load(int3(gtid.xy, 0u)).x == 1.0f)
    {
        refl_refr_radiance_out_tex[gtid.xy] = 0.0f;
        return;
    }

    float2 uv = (float2(int2(gtid.xy)) + 0.5f) / float2(depth_tex_dims);
    float4 bn = ((blue_noise_texture.Load(int3(
        (gtid.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
            * float(frame_index & 255u)) + 0.5f) * 256.0f)) % uint2(256u, 256u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    float2 ndc = (uv - 0.5f) * float2(2.0f, -2.0f);
    float4 ray_vs = mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f));
    float3 view_ray = normalize(mul(view_constants[0].view_to_world, ray_vs).xyz);
    float4 pos_vs = float4(ndc, depth_tex.Load(int3(gtid.xy, 0u)).x, 1.0f);
    float4 wp4 = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, pos_vs));
    float3 world_pos = wp4.xyz / wp4.w;

    float4 refl_hit = refl_hit_tex.Load(int3(gtid.xy / uint2(2u, 2u), 0u));
    float roughness = ice_roughness_tex.Load(int3(gtid.xy, 0u)).x;
    float rough2 = max(0.000600000028498470783233642578125f, roughness * roughness);

    float3 view_dir = swap_yz(-view_ray);
    float3 view_dir_fix;
    if (view_dir.z < 0.0f)
    {
        view_dir_fix = normalize(float3(view_dir.x, view_dir.y, view_dir.z * (-0.25f)));
    }
    else
    {
        view_dir_fix = view_dir;
    }

    uint quad_id = ((gtid.x & 1u) | ((gtid.y & 1u) * 2u)) & 3u;
    float dist = length(mul(view_constants[0].world_to_view, float4(world_pos, 1.0f)).xyz);
    float vdotn = -normalize(ray_vs.xyz).z;
    float inv_dist = max(9.9999997473787516355514526367188e-06f, dist);
    float ray_scale = 16.0f / inv_dist;

    float4 prev = mul(view_constants[0].clip_to_prev_clip,
        mul(view_constants[0].view_to_clip, mul(view_constants[0].clip_to_view, pos_vs)));
    float2 prev_uv = ((prev.xy / prev.w) * float2(0.5f, -0.5f)) + 0.5f;
    float2 prev_uv_clamped = clamp(prev_uv, 0.0f, 1.0f);
    float in_view = float(all(prev_uv_clamped == prev_uv));
    float2 reproj_uv = uv + (prev_uv - ((pos_vs.xy * float2(0.5f, -0.5f)) + 0.5f));

    float cur_exp = exp2(-clamp(ray_scale * refl_hit.x, 0.0f, 100.0f));
    float ray_log = (-0.0625f) * inv_dist;
    float ray_len = max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f
        + lerp(exp2(-clamp(ray_scale * ray_len_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).y, 0.0f, 100.0f)),
            cur_exp, 0.100000001490116119384765625f)));
    float ray_len_f = max(ray_len,
        (((dist / vdotn) * view_constants[0].clip_to_view[1].y) * 0.20000000298023223876953125f)
            * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, ray_len));
    float proj_scale = (dist * vdotn) * view_constants[0].clip_to_view[1].y;

    float3 roughness3 = ((1.0f - rough2) * (sqrt(1.0f - rough2) + rough2)).xxx;
    float3 refl_dir = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f),
        reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), roughness3)), float3(0.0f, 1.0f, 0.0f));
    float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir))
        * max(min((((ray_len_f / (ray_len_f + dist)) * dist) * vdotn) * (sqrt(rough2) * 0.25f),
            0.100000001490116119384765625f * proj_scale),
            (4.0f / float(color_tex_dims.y)) * proj_scale);
    float3 tangent = cross(refl_dir, binorm);

    float golden = float((frame_index * 59u) % 128u) * 1.324717998504638671875f;

    float4 acc = 0.0f;
    float acc_w = 0.0f;
    float t = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        float angle = ((float(i) + golden) * 2.3999631404876708984375f) + (float(quad_id) * 1.57079637050628662109375f);
        float4 s4 = mul(view_constants[0].view_to_sample,
            mul(view_constants[0].world_to_view, float4(
                world_pos + ((binorm * cos(angle)) + (tangent * sin(angle))) * (pow(t + bn.y, 1.0f) * 0.125f),
                1.0f)));
        uint2 px = uint2(int2(floor((((s4.xyz / s4.w).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refl_hit_tex_dims))));
        float4 rh = refl_hit_tex.Load(int3(px, 0u));
        if (rh.y != 0.0f)
        {
            float w = (9.9999999747524270787835121154785e-07f
                * exp((-1.0f) * abs((4.0f / (1.0f + (ray_len * ray_len))) - (4.0f / (1.0f + (rh.x * rh.x))))))
                * (9.9999999747524270787835121154785e-07f + clamp(1.0f - (8.0f * abs(refl_hit.y - rh.y)), 0.0f, 1.0f));
            acc += float4(refl_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * w;
            acc_w += cur_exp * w;
        }
        t += 1.0f;
    }

    float acc_norm = max(9.9999998245167004418121337039338e-15f, acc.w);
    float3 refl_dir2 = normalize(lerp(float3(0.0f, 0.0f, 1.0f),
        reflect(-view_dir_fix, float3(0.0f, 0.0f, 1.0f)), roughness3));
    float3 radiance = ((acc.xyz / acc_norm) * lerp(0.100000001490116119384765625f, 1.0f,
        exp2((-1.0f) - (8.656169891357421875f * dot(refl_dir2, view_dir_fix + refl_dir2)))))
        * (1.0f + rough2)
        + refl_refr_radiance_out_tex[gtid.xy].xyz;

    float3 blended;
    if (history_valid != 0u && frame_index != 0u)
    {
        float3 hist = refl_refr_radiance_history_tex.SampleLevel(sampler_lnc, reproj_uv, 0.0f).xyz;
        if (all(hist > 0.0f))
        {
            blended = lerp(hist, radiance, lerp(1.0f, 0.25f, in_view));
        }
        else
        {
            blended = radiance;
        }
    }
    else
    {
        blended = radiance;
    }

    FrexpStructType f;
    f.mantissa = frexp(blended, f.exponent);
    float3 quant = blended + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
    refl_refr_radiance_out_tex[gtid.xy] = quant;
    color_tex[gtid.xy] = color_tex[gtid.xy].xyz + blended;
    ray_len_out_tex[gtid.xy] = float2(max(0.0f, ray_log * log2(1.0000000031710768509710513471353e-30f + (acc_w / acc_norm))), ray_len);
}
