// ==RECONSTRUCTED== shaders/gi/update_gi_radiance.cs.hlsl | status: O (structure from disassembly)
// MERGE (gi-deferred review 2026-10-13): content adopted from candidate B (near-exact include
// closure). hi_px_subpixels now comes from gi_settings.inc.hlsl (A's placement, pinned by the
// bin debug names) instead of a local copy. terrain_heightmap.inc.hlsl added for closure
// parity; this pass binds no terrain texture so TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL is set.
// Evidence: _gi_update_gi_radiance.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: depth_tex t26, candidate_radiance_tex t33, candidate_hit_tex t34,
//     restir_radiance_out_tex u35, restir_ray_dir_out_tex u36, restir_reservoir_out_tex u37,
//     frame_constants set1/b0. LocalSize [8,8,1], entry cs_main.
//   spirv-cross: copies the traced candidate radiance/hit to the ReSTIR outputs and
//     seeds the reservoir with a default payload; ray dir out = hit.xyz + jittered depth.

#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/hash.hlsl"
#include "inc/consts.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "gi/restir.inc.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/rt.hlsl"
#include "inc/utils.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/history_valid.hlsl"

Texture2D<float4> depth_tex : register(t26, space0);
Texture2D<float4> candidate_radiance_tex : register(t33, space0);
Texture2D<float4> candidate_hit_tex : register(t34, space0);
RWTexture2D<float4> restir_radiance_out_tex : register(u35, space0);
RWTexture2D<float4> restir_ray_dir_out_tex : register(u36, space0);
RWTexture2D<uint2> restir_reservoir_out_tex : register(u37, space0);

uint pack_half2x16(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    restir_radiance_out_tex[xy] = candidate_radiance_tex.Load(int3(xy, 0u));
    restir_ray_dir_out_tex[xy] = float4(candidate_hit_tex.Load(int3(xy, 0u)).xyz, depth_tex.Load(int3((xy * uint2(2u, 2u)) + hi_px_subpixels[frame_index & 3u], 0u)).x);

    uint packed = pack_half2x16(float2(1.0f, 0.0f));
    restir_reservoir_out_tex[xy] = uint2(xy.x | (xy.y << 16u), packed | (packed << 16u));
}
