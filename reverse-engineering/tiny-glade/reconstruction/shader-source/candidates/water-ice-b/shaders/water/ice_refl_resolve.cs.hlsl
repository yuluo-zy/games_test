// ==RECONSTRUCTED== shaders/water/ice_refl_resolve.cs.hlsl
// Evidence: _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (13 members: dims + history_valid), sampler_lnc s0, blue_noise_texture
//   t15, depth_tex t16, ice_roughness_tex t17, refl_hit_tex t18 (sampled), refl_hit_radiance_tex t19
//   (sampled), color_tex u20 (RWTexture2D<float3> r11g11b10f), refl_refr_radiance_history_tex t21,
//   refl_refr_radiance_out_tex u22 (RWTexture2D<float3>, unknown-format -> WriteWithoutFormat),
//   ray_len_history_tex t23, ray_len_out_tex u24 (RWTexture2D<float2> rg16f), frame_constants set1/b0.
//   LocalSize [8,8,1], entry cs_main.
//   spirv-cross + disassembly: reflection resolve. Rejects rough=1 cells, cone-spreads a fixed
//   number of rays (blue-noise + per-pixel corner), gathers refl_hit_radiance weighted by a ray-cone
//   LOD / roughness similarity, blends TAA history (with a clip-space validity gate) and quantizes
//   through an 8-bit-exponent frexp, accumulating into color_tex and writing the ray length.
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
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> color_tex : register(u20, space0);
Texture2D<float4> refl_refr_radiance_history_tex : register(t21, space0);
RWTexture2D<float3> refl_refr_radiance_out_tex : register(u22, space0);
Texture2D<float4> ray_len_history_tex : register(t23, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> ray_len_out_tex : register(u24, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;
    if (ice_roughness_tex.Load(int3(px, 0)).x == 1.0f)
    {
        refl_refr_radiance_out_tex[px] = 0.0f;
        return;
    }

    float2 uv = (float2(int2(px)) + 0.5f) / float2(depth_tex_dims);
    uint2 bn_off = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index & 255u))) + 0.5f.xx) * 256.0f;
    float4 bn = ((blue_noise_texture.Load(int3((px + bn_off) % uint2(256u, 256u), 0u)) * 255.0f) * (1.0f / 256.0f)) + (1.0f / 512.0f);

    float2 clip = (uv - 0.5f) * float2(2.0f, -2.0f);
    float4 view_ray = mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f));
    float4 wpos = mul(view_constants[0].view_to_world,
                      mul(view_constants[0].sample_to_view, float4(clip, depth_tex.Load(int3(px, 0)).x, 1.0f)));
    float3 pos = (wpos.xyz / wpos.w.xxx);
    float4 hit = refl_hit_tex.Load(int3(px / uint2(2u, 2u), 0u));
    float rough = ice_roughness_tex.Load(int3(px, 0)).x;
    float rough2 = max(0.000600000028498470783233642578125f, rough * rough);

    float3 view_dir = normalize(normalize(mul(view_constants[0].view_to_world, view_ray).xyz));
    float3 basis = mul(-view_dir, float3x3(float3(1.0f, -0.0f, 0.0f),
                                           float3(-0.0f, 0.0f, 1.0f),
                                           float3(-0.0f, -1.0f, 0.0f)));
    float3 basis_z = basis;
    if (basis_z.z < 0.0f)
    {
        basis_z.z *= -0.25f;
        basis_z = normalize(basis_z);
    }

    uint corner = ((px.x & 1u) | ((px.y & 1u) * 2u)) & 3u;
    float view_dist = length(mul(view_constants[0].world_to_view, float4(pos, 1.0f)).xyz);
    float view_nz = -normalize(view_ray.xyz).z;
    float pixels_per = 16.0f / max(9.9999997473787516355514526367188e-06f, view_dist);

    float4 reproj = mul(view_constants[0].clip_to_prev_clip,
                        mul(view_constants[0].view_to_clip,
                            mul(view_constants[0].clip_to_view, float4(clip, depth_tex.Load(int3(px, 0)).x, 1.0f))));
    float2 reproj_uv = ((reproj.xy / reproj.w.xx) * float2(0.5f, -0.5f)) + 0.5f;
    float2 clamp_uv = clamp(reproj_uv, 0.0f, 1.0f);
    float valid = float(all(bool2(clamp_uv.x == reproj_uv.x, clamp_uv.y == reproj_uv.y)));
    float2 hist_uv = uv + float4(reproj_uv - ((float4(clip, 0.0f, 1.0f).xy * float2(0.5f, -0.5f)) + 0.5f), valid, 1.0f).xy;

    float ray_len = exp2(-clamp(pixels_per * hit.x, 0.0f, 100.0f));
    float lods = max(0.0f, (-0.0625f * view_dist) * log2(1.0000000031710768509710513471353e-30f
        + lerp(exp2(-clamp(pixels_per * ray_len_history_tex.SampleLevel(sampler_lnc, hist_uv, 0.0f).y, 0.0f, 100.0f)),
                ray_len, 0.100000001490116119384765625f)));
    float spread = max(lods, ((view_dist / view_nz) * view_constants[0].clip_to_view[1].y * 0.20000000298023223876953125f)
        * smoothstep(0.0f, 0.0500000007450580596923828125f * view_dist, lods));
    float footprint = (view_dist * view_nz) * view_constants[0].clip_to_view[1].y;

    float one_m = 1.0f - rough2;
    float3 mip = (one_m * (sqrt(one_m) + rough2)).xxx;
    float3 spec = reflect(-normalize(lerp(float3(0.0f, 1.0f, 0.0f), reflect(view_dir, float3(0.0f, 1.0f, 0.0f)), mip)),
                          float3(0.0f, 1.0f, 0.0f));
    float3 axis = normalize(cross(float3(0.0f, 1.0f, 0.0f), spec)) * max(min(
        ((spread / (spread + view_dist)) * view_dist * view_nz) * (sqrt(rough2) * 0.25f),
        0.100000001490116119384765625f * footprint), (4.0f / float(color_tex_dims.y)) * footprint);
    float3 ortho = cross(spec, axis);

    float seed = float((frame_index * 59u) % 128u) * 1.324717998504638671875f;

    float4 acc = 0.0f;
    float ray_acc = 0.0f;
    for (int i = 0; i < 8; ++i)
    {
        float ang = ((float(i) + seed) * 2.3999631404876708984375f) + (float(corner) * 1.57079637050628662109375f);
        float4 sp = mul(view_constants[0].view_to_sample,
                        mul(view_constants[0].world_to_view,
                            float4(pos + (((axis * cos(ang)) + (ortho * sin(ang))) * (pow(float(i) + bn.y, 1.0f) * 0.125f)), 1.0f)));
        uint2 p2 = uint2(int2(floor((((sp.xyz / sp.w.xxx).xy * float2(0.5f, -0.5f)) + 0.5f) * float2(refl_hit_tex_dims))));
        float4 h2 = refl_hit_tex.Load(int3(p2, 0u));
        if (h2.y != 0.0f)
        {
            float w = (9.9999999747524270787835121154785e-07f
                * exp((-1.0f) * abs((4.0f / (1.0f + (lods * lods))) - (4.0f / (1.0f + (h2.x * h2.x))))))
                * (9.9999999747524270787835121154785e-07f + clamp(1.0f - (8.0f * abs(hit.y - h2.y)), 0.0f, 1.0f));
            acc += float4(refl_hit_radiance_tex.Load(int3(p2, 0u)).xyz, 1.0f) * w;
            ray_acc += ray_len * w;
        }
    }

    float sum = max(9.9999998245167004418121337039338e-15f, acc.w);
    float3 avg = acc.xyz / sum;
    float3 m = normalize(lerp(float3(0.0f, 0.0f, 1.0f), reflect(-basis_z, float3(0.0f, 0.0f, 1.0f)), mip));
    float3 out_rad = ((avg * lerp(0.100000001490116119384765625f, 1.0f,
        exp2((-1.0f) - (8.656169891357421875f * dot(m, basis_z + m))))) * (1.0f + rough2))
        + refl_refr_radiance_out_tex[px].xyz;

    float3 final;
    if (history_valid != 0u && frame_index != 0u)
    {
        float3 hist = refl_refr_radiance_history_tex.SampleLevel(sampler_lnc, hist_uv, 0.0f).xyz;
        final = all(bool3(hist.x > 0.0f, hist.y > 0.0f, hist.z > 0.0f))
            ? lerp(hist, out_rad, lerp(1.0f, 0.25f, valid).xxx) : out_rad;
    }
    else
    {
        final = out_rad;
    }

    refl_refr_radiance_out_tex[px] = ice_quantize_radiance(final);
    color_tex[px] = color_tex[px].xyz + final;
    ray_len_out_tex[px] = float2(max(0.0f, (-0.0625f * view_dist) * log2(1.0000000031710768509710513471353e-30f + (ray_acc / sum))), lods);
}
