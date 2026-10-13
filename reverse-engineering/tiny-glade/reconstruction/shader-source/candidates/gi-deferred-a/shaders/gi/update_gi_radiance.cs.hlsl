// ==RECONSTRUCTED== shaders/gi/update_gi_radiance.cs.hlsl | status: O (structure from disassembly)
// Evidence: _gi_update_gi_radiance.cs.hlsl_cs_main_0.bin (LocalSize 8x8, entry cs_main).
//   bindings: depth_tex t26, candidate_radiance_tex t33, candidate_hit_tex t34
//   (SampledImage), restir_radiance_out_tex u35 / restir_ray_dir_out_tex u36
//   (RWTexture2D<float4>), restir_reservoir_out_tex u37 (RWTexture2D<uint2>),
//   frame_constants set1/b0; caps {Shader, RuntimeDescriptorArray, StorageImageExtendedFormats}.
//   For each half-res texel: copy the candidate radiance into the ReSTIR radiance
//   buffer, pack the candidate ray dir with the full-res depth (sub-pixel jittered)
//   into the ray-dir buffer, and seed a fresh reservoir {payload = pixel id,
//   weights = (1,1)}.
//   Include residual: original closure ~38 files vs candidate ~14 (see decisions.md).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/rt.hlsl"
#include "gi_settings.inc.hlsl"
#include "restir.inc.hlsl"

Texture2D<float4> depth_tex : register(t26, space0);
Texture2D<float4> candidate_radiance_tex : register(t33, space0);
Texture2D<float4> candidate_hit_tex : register(t34, space0);
RWTexture2D<float4> restir_radiance_out_tex : register(u35, space0);
RWTexture2D<float4> restir_ray_dir_out_tex : register(u36, space0);
RWTexture2D<uint2> restir_reservoir_out_tex : register(u37, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;

    restir_radiance_out_tex[px] = candidate_radiance_tex.Load(int3(px, 0));

    // Candidate ray direction plus the full-res depth at the matching sub-pixel.
    float4 hit = candidate_hit_tex.Load(int3(px, 0));
    float depth = depth_tex.Load(int3((px * 2u) + hi_px_subpixels[frame_index & 3u], 0)).x;
    restir_ray_dir_out_tex[px] = float4(hit.xyz, depth);

    // Fresh reservoir: payload = packed pixel id, weights = (1, 1).
    uint weights = pack_restir_weights(float2(1.0f, 0.0f));
    restir_reservoir_out_tex[px] = uint2(pack_restir_payload(px), weights | (weights << 16u));
}
