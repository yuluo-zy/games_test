// ==RECONSTRUCTED== shaders/gi/eval_screen_radiance.cs.hlsl | status: O (structure from disassembly)
// Evidence: _gi_eval_screen_radiance.cs.hlsl_cs_main_0.bin (LocalSize 8x8, entry cs_main).
//   bindings: $Globals b13 (8 dims), blue_noise_texture t14, half_depth_tex t15,
//   half_view_normal_tex t16, ssgi_tex t17, restir_radiance_tex t18,
//   restir_ray_dir_tex t19, restir_reservoir_tex t20, output_tex u21,
//   frame_constants set1/b0; caps {Shader, RuntimeDescriptorArray}.
//   Evaluates the screen-space radiance at each half-res texel: 8 golden-angle
//   spiral samples read the ReSTIR radiance/ray-dir/reservoir buffers, weight each
//   sample by depth/normal/SSGI agreement and the reservoir W weight, project onto
//   a band-0/1 SH basis, and store 3 SH texels (RGB channels) side by side.
//   Include residual: original closure ~21 files vs candidate ~14 (see decisions.md).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/halfres.hlsl"
#include "gi_settings.inc.hlsl"
#include "restir.inc.hlsl"

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

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> half_depth_tex : register(t15, space0);
Texture2D<float4> half_view_normal_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t17, space0);
Texture2D<float4> restir_radiance_tex : register(t18, space0);
Texture2D<float4> restir_ray_dir_tex : register(t19, space0);
Texture2D<uint2> restir_reservoir_tex : register(t20, space0);
RWTexture2D<float4> output_tex : register(u21, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;

    // Rotating blue-noise sample (256x256 tile, 2D golden-ratio offsets).
    uint2 noise_px = (px + uint2(13u, 13u) +
        uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(71u + frame_index)) + 0.5f) * 256.0f)) %
        uint2(256u, 256u);
    float4 noise = ((blue_noise_texture.Load(int3(noise_px, 0)) * 255.0f) * 0.00390625f) + 0.001953125f;

    float depth = half_depth_tex.Load(int3(px, 0)).x;
    uint subp = frame_index & 3u;
    float ssgi = ssgi_tex.Load(int3((px * 2u) + hi_px_subpixels[subp], 0)).x;
    float3 view_normal = half_view_normal_tex.Load(int3(px, 0)).xyz;
    float3 world_normal = mul(view_constants[0].view_to_world, float4(view_normal, 0.0f)).xyz;
    float normal_z = max(0.3f, half_view_normal_tex.Load(int3(px, 0)).z);

    float4 sh_acc[3] = { float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f) };
    float weight_sum = 0.0f;

    for (uint i = 0u; i < 8u; ++i) {
        float fi = float(i);
        float angle = (fi + noise.x) * 2.3999631404876708984375f;
        uint2 sample_px = uint2(float2(px) + 0.5f + (float2(cos(angle), sin(angle)) * sqrt(fi + noise.y) * 0.3535533845424652099609375f * 8.0f * 2.0f));
        uint2 even_px = sample_px - (sample_px % uint2(2u, 2u));

        float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(even_px, 0)).x / depth))))) *
                  exp2((-20.0f) * abs(ssgi_tex.Load(int3((even_px * 2u) + hi_px_subpixels[subp], 0)).x - ssgi)) *
                  float(dot(world_normal, restir_ray_dir_tex.Load(int3(even_px, 0)).xyz) > 0.0f);

        float W = f16tof32((restir_reservoir_tex.Load(int3(even_px, 0)).y >> 16u) & 0xFFFFu);
        float3 radiance = restir_radiance_tex.Load(int3(even_px, 0)).xyz * max(0.0f, W) * w;
        float4 sh_basis = float4(0.2820949852466583251953125f, restir_ray_dir_tex.Load(int3(even_px, 0)).xyz * 0.48860299587249755859375f) * 4.0f;

        sh_acc[0] += sh_basis * radiance.x;
        sh_acc[1] += sh_basis * radiance.y;
        sh_acc[2] += sh_basis * radiance.z;
        weight_sum += w;
    }

    float inv = 0.5f / max(1e-5f, weight_sum);
    sh_acc[0] *= inv;
    sh_acc[1] *= inv;
    sh_acc[2] *= inv;

    [unroll]
    for (uint c = 0u; c < 3u; ++c) {
        output_tex[px + uint2(c * half_depth_tex_dims.x, 0u)] = sh_acc[c];
    }
}
