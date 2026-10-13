// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
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

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/blue_noise.hlsl"
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
            if (rejected || reflect(view_dir, v).z < 0.0f)
            {
                k++;
                continue;
            }
            refl_dir = v;
            break;
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
