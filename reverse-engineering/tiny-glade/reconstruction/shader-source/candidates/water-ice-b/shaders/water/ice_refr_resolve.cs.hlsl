// ==RECONSTRUCTED== shaders/water/ice_refr_resolve.cs.hlsl
// Evidence: _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (depth_tex_dims, refr_hit_tex_dims, refr_hit_radiance_tex_dims,
//   refl_refr_radiance_out_tex_dims), depth_tex t14, refr_hit_tex t15 (RGBA8-packed normal+scale),
//   refr_hit_radiance_tex t16, refl_refr_radiance_out_tex u17, frame_constants set1/b0.
//   LocalSize [8,8,1], entry cs_main.
//   spirv-cross + disassembly: refraction resolve. Rejects empty hit cells (packed normal word 0),
//   decodes the RG8 octahedral-ish normal + half-float cone scale from the hit, fires 8 cosine-ish
//   cone rays around the specular direction with an ice fixed point spread (0.959), accumulates
//   refr_hit_radiance with a fresnel-ish weight, and writes the normalized accumulated radiance.
// Status: O (algorithm + bindings from SPIR-V).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "water/ice_shared.inc.hlsl"

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
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> refl_refr_radiance_out_tex : register(u17, space0);

// Reconstruct sqrt(max(0, d)) with the fixed-point magic (observed in the disassembly).
float ice_sqrt_approx(float d)
{
    return asfloat(532487669 + (asint(max(0.0f, d)) >> 1));
}

// Decode a packed hit: low bytes -> xy of a unit normal (z reconstructed), low half-word -> scale.
float3 decode_refr_normal(uint packed)
{
    float3 n = float3((float((packed >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                      (float((packed >> 16u) & 255u) * 0.007843137718737125396728515625f) - 1.0f,
                      0.0f);
    n.z = ice_sqrt_approx(1.0f - dot(n.xy, n.xy));
    return n;
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;
    uint packed = refr_hit_tex.Load(int3(px, 0)).x;
    if (packed == 0u)
    {
        return;
    }

    // World position of the pixel.
    float2 clip = ((float2(int2(px)) + 0.5f) / float2(depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float4 wpos = mul(view_constants[0].view_to_world,
                      mul(view_constants[0].sample_to_view,
                          float4(clip, depth_tex.Load(int3(px, 0)).x, 1.0f)));
    float3 pos = wpos.xyz / wpos.w;
    float4 vpos = mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f));
    float3 view_dir = normalize(normalize(mul(view_constants[0].view_to_world, vpos).xyz));

    float3 n = decode_refr_normal(packed);
    float3 tangent_basis = mul(-view_dir, float3x3(float3(1.0f, -0.0f, 0.0f),
                                                   float3(-0.0f, 0.0f, 1.0f),
                                                   float3(-0.0f, -1.0f, 0.0f)));
    float cone_len = length(n * unpack_half(packed));
    float view_dist = length(mul(view_constants[0].world_to_view, float4(pos, 1.0f)).xyz);
    float view_nz = -normalize(vpos.xyz).z;
    float spread = max(cone_len, ((view_dist / view_nz) * view_constants[0].clip_to_view[1].y * 0.2f)
        * smoothstep(0.0f, 0.05f * view_dist, cone_len));
    float pixel_footprint = (view_dist * view_nz) * view_constants[0].clip_to_view[1].y;

    // Fixed ice spread / fresnel constant 0.959 (tan of the refraction cone).
    float3 spec = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f), reflect(view_dir, float3(0.0f, 1.0f, 0.0f)),
                                          0.959011614322662353515625f.xxx)), float3(0.0f, 1.0f, 0.0f));
    float3 axis = normalize(cross(float3(0.0f, 1.0f, 0.0f), spec)) * max(min(
        ((spread / (spread + view_dist)) * view_dist * view_nz) * 0.0684653222560882568359375f,
        0.1f * pixel_footprint), (4.0f / float(refl_refr_radiance_out_tex_dims.y)) * pixel_footprint);
    float3 ortho = cross(spec, axis);

    float seed = (float((frame_index * 59u) % 128u) * 1.324717998504638671875f)
        + (dot(float2(px & uint2(1u, 1u)), float2(1.0f, 2.0f)) * 0.654508531093597412109375f);

    float4 acc = 0.0f;
    for (int i = 0; i < 8; ++i)
    {
        float ang = (float(i) + seed) * 2.3999631404876708984375f;
        float3 sample_pos = pos + ((axis * cos(ang)) + (ortho * sin(ang))) * (float(i) * 0.125f);
        float4 sp = mul(view_constants[0].view_to_sample,
                        mul(view_constants[0].world_to_view, float4(sample_pos, 1.0f)));
        uint2 hit_px = uint2(int2(floor((((sp.xyz / sp.w.xxx).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refr_hit_tex_dims))));
        uint hit = refr_hit_tex.Load(int3(hit_px, 0)).x;
        if (hit != 0u)
        {
            float3 hn = decode_refr_normal(hit);
            float3 dir = normalize(tangent_basis + normalize((hn * unpack_half(hit))
                + mul(sample_pos - pos, float3x3(float3(1.0f, -0.0f, 0.0f),
                                                 float3(-0.0f, 0.0f, 1.0f),
                                                 float3(-0.0f, -1.0f, 0.0f)))));
            float fres = ((dir.z * dir.z) * (-0.9943749904632568359375f)) + 1.0f;
            acc += float4(refr_hit_radiance_tex.Load(int3(hit_px, 0)).xyz, 1.0f)
                * (3.1640625820728018879890441894531e-05f / (fres * fres));
        }
    }

    refl_refr_radiance_out_tex[px] = acc.xyz / max(9.9999998245167004418121337039338e-15f, acc.w);
}
