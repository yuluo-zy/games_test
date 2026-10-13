// ==RECONSTRUCTED== shaders/water/ice_refr_resolve.cs.hlsl
// SOURCE PATH : shaders/water/ice_refr_resolve.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [8,8,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — refraction resolve: reads refr_hit_tex (uint4) packed
//   normal/depth, reconstructs world position, spirals 8 taps over refr_hit_tex
//   projecting world-space offset samples, weights by normal-aligned fresnel-ish
//   term (0.99437 quadratic in view normal z), writes normalized radiance to
//   refl_refr_radiance_out_tex.
// MERGE (water-ice review water/ice_refr_resolve.cs.hlsl): A adopted wholesale
//   (include closure matches the bin OpString list exactly; behavior equivalent).

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
#include "inc/color/srgb.hlsl"
#include "ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 refr_hit_tex_dims;
    uint2 refr_hit_radiance_tex_dims;
    uint2 refl_refr_radiance_out_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<uint4> refr_hit_tex : register(t15, space0);
Texture2D<float4> refr_hit_radiance_tex : register(t16, space0);
// r11g11b10f storage format (target OpTypeImage fmt 8). NOTE: target also carries
// StorageImageWriteWithoutFormat; the installed dxc emits only
// StorageImageExtendedFormats for an explicit-format storage image — caps residual.
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);

// Tangent-space coordinate swap (x, y, z) -> (x, -z, y) applied to view dirs;
// matches the float3x3( (1,0,0), (0,0,1), (0,-1,0) ) row-matrix in the bins.
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// Reconstruct the third component of a unit normal stored as two bytes (fast
// normalize via the magic-constant sqrt(1-dot) trick).
float normal_z_from_xy(float2 xy)
{
    return asfloat(532487669u + (asint(max(0.0f, 1.0f - dot(xy, xy))) >> 1));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    uint4 hit = refr_hit_tex.Load(int3(gtid.xy, 0u));
    if (0u == hit.x)
    {
        return;
    }

    float3 n_local = float3((float((hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            (float((hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                            0.0f);
    n_local.z = normal_z_from_xy(n_local.xy);

    float2 ndc = (((float2(int2(gtid.xy)) + 0.5f) / float2(depth_tex_dims)) - 0.5f) * float2(2.0f, -2.0f);
    float4 view_ray_vs4 = mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f));
    float3 view_ray = normalize(mul(view_constants[0].view_to_world, view_ray_vs4).xyz);
    float4 wp4 = mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, depth_tex.Load(int3(gtid.xy, 0u)).x, 1.0f)));
    float3 world_pos = wp4.xyz / wp4.w;

    float3 view_dir = swap_yz(-view_ray);
    float roughness_raylen = length(n_local * unpack_half2x16(hit.x & 0xFFFFu).x);
    float dist = length(mul(view_constants[0].world_to_view, float4(world_pos, 1.0f)).xyz);
    float vdotn = -normalize(view_ray_vs4.xyz).z;
    float blur = max(roughness_raylen,
        (((dist / vdotn) * view_constants[0].clip_to_view[1].y) * 0.20000000298023223876953125f)
            * smoothstep(0.0f, 0.0500000007450580596923828125f * dist, roughness_raylen));
    float proj_scale = (dist * vdotn) * view_constants[0].clip_to_view[1].y;

    float3 refl_dir = swap_yz(-normalize(lerp(float3(0.0f, 1.0f, 0.0f),
        reflect(view_ray, float3(0.0f, 1.0f, 0.0f)), 0.959011614322662353515625f)));
    float3 binorm = normalize(cross(float3(0.0f, 1.0f, 0.0f), refl_dir))
        * max(min((((blur / (blur + dist)) * dist) * vdotn) * 0.0684653222560882568359375f,
            0.100000001490116119384765625f * proj_scale),
            (4.0f / float(refl_refr_radiance_out_tex_dims.y)) * proj_scale);
    float3 tangent = cross(refl_dir, binorm);

    float golden = (float((frame_index * 59u) % 128u) * 1.324717998504638671875f)
        + (dot(float2(gtid.xy & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);

    float4 acc = 0.0f;
    float t = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        float angle = (float(i) + golden) * 2.3999631404876708984375f;
        float3 sample_pos = world_pos + ((binorm * cos(angle)) + (tangent * sin(angle))) * t;
        float4 s4 = mul(view_constants[0].view_to_sample,
            mul(view_constants[0].world_to_view, float4(sample_pos, 1.0f)));
        uint2 px = uint2(int2(floor((((s4.xyz / s4.w).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refr_hit_tex_dims))));
        uint4 sample_hit = refr_hit_tex.Load(int3(px, 0u));
        if (sample_hit.x != 0u)
        {
            float3 sn = float3((float((sample_hit.x >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               (float((sample_hit.x >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                               0.0f);
            sn.z = normal_z_from_xy(sn.xy);
            float3 v = normalize(view_dir + normalize((sn * unpack_half2x16(sample_hit.x & 0xFFFFu).x)
                + swap_yz(sample_pos - world_pos)));
            float weight = 3.1640625820728018879890441894531e-05f / pow((-0.9943749904632568359375f * (v.z * v.z)) + 1.0f, 2.0f);
            acc += float4(refr_hit_radiance_tex.Load(int3(px, 0u)).xyz, 1.0f) * weight;
        }
        t += 0.125f;
    }

    refl_refr_radiance_out_tex[gtid.xy] = (acc.xyz / max(9.9999998245167004418121337039338e-15f, acc.w)).xyz;
}
