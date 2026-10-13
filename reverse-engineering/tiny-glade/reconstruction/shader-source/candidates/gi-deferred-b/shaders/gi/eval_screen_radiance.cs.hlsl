// ==RECONSTRUCTED== shaders/gi/eval_screen_radiance.cs.hlsl (candidate gi-deferred-b)
// Evidence: _gi_eval_screen_radiance.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: $Globals b13 {8 tex dims}, blue_noise_texture t14, half_depth_tex t15,
//     half_view_normal_tex t16, ssgi_tex t17, restir_radiance_tex t18, restir_ray_dir_tex t19,
//     restir_reservoir_tex t20, output_tex u21, frame_constants set1/b0. LocalSize [8,8,1].
//   spirv-cross: 8-spiral half-res GI screen-radiance evaluation: reconstruct world
//   normal, sample candidate rays/radiance at snapped half-res positions weighted by
//   depth/ssgi/ray-dir agreement, project the radiance onto SH band 1 (Y00/Y1x..) and
//   accumulate three SH-vector slabs. Reservoir high-half of .y is the radiance weight.
// Status: O (structure from disassembly); I (blue_noise() helper reuse, seed 71+frame).

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/hash.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/halfres.hlsl"
#include "inc/math.hlsl"
#include "inc/sh.hlsl"
#include "restir.inc.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 ssgi_tex_dims;
    uint2 restir_radiance_tex_dims;
    uint2 restir_ray_dir_tex_dims;
    uint2 restir_reservoir_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> half_depth_tex : register(t15, space0);
Texture2D<float4> half_view_normal_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t17, space0);
Texture2D<float4> restir_radiance_tex : register(t18, space0);
Texture2D<float4> restir_ray_dir_tex : register(t19, space0);
Texture2D<uint4> restir_reservoir_tex : register(t20, space0);
RWTexture2D<float4> output_tex : register(u21, space0);

static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

float2 unpack_half2x16(uint v)
{
    return f16tof32(uint2(v & 0xffffu, v >> 16u));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    float4 bn = blue_noise(xy, 71u + frame_index);

    float depth = half_depth_tex.Load(int3(xy, 0u)).x;
    uint sub = frame_index & 3u;
    float ssgi = ssgi_tex.Load(int3((xy * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x;
    float3 world_n = mul(view_constants[0u].view_to_world, float4(half_view_normal_tex.Load(int3(xy, 0u)).xyz, 0.0f)).xyz;
    float normal_z = max(0.300000011920928955078125f, half_view_normal_tex.Load(int3(xy, 0u)).z);

    float4 acc[3];
    acc[0] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[1] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[2] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float total = 0.0f;

    for (uint i = 0u; i < 8u; ++i) {
        float angle = (float(i) + bn.x) * 2.3999631404876708984375f;
        uint2 sp = uint2((float2(xy) + 0.5f.xx) + (((float2(cos(angle), sin(angle)) * sqrt(float(i) + bn.y)) * 0.3535533845424652099609375f) * 8.0f) * 2.0f);
        sp -= (sp % uint2(2u, 2u));

        float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(sp, 0u)).x / depth)))))
                * exp2((-20.0f) * abs(ssgi_tex.Load(int3((sp * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x - ssgi))
                * float(int(dot(world_n, restir_ray_dir_tex.Load(int3(sp, 0u)).xyz) > 0.0f));

        float rad_w = max(0.0f, unpack_half2x16((restir_reservoir_tex.Load(int3(sp, 0u)).y >> 16u) & 0xffffu).x);
        float3 rad = restir_radiance_tex.Load(int3(sp, 0u)).xyz * rad_w * w;
        float4 sh = float4(0.2820949852466583251953125f, restir_ray_dir_tex.Load(int3(sp, 0u)).xyz * 0.48860299587249755859375f) * 4.0f;
        acc[0] += sh * rad.x;
        acc[1] += sh * rad.y;
        acc[2] += sh * rad.z;
        total += w;
    }

    float factor = 0.5f / max(9.9999997473787516355514526367188e-06f, total);
    acc[0] *= factor;
    acc[1] *= factor;
    acc[2] *= factor;

    for (uint c = 0u; c < 3u; ++c) {
        output_tex[xy + uint2(c * half_depth_tex_dims.x, 0u)] = acc[c];
    }
}
