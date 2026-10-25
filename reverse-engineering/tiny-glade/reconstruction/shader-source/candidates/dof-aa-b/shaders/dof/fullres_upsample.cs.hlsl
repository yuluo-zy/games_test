// ==SKELETON==
// SOURCE PATH : shaders/dof/fullres_upsample.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_fullres_upsample.cs.hlsl_cs_main_0.bin
//     file_sha256  : b2deee990e6df11974d6a79993c02658fb209fdce2ce9f6ff1dfc3c4525cf734
//     pass/stage   : _dof_fullres_upsample (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W51 (Inference)
//     names        : halfres_tex, spatial_resolve_offsets, img_output, DofTileOffsets, tile_offsets_buffer, tile_coords_buffer, cs_main
//
// ==RECONSTRUCTED== shaders/dof/fullres_upsample.cs.hlsl
// Evidence: _dof_fullres_upsample.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 115).
//   bindings: halfres_tex t15, spatial_resolve_offsets b16 (int4[512]),
//   img_output u17, tile_offsets_buffer t18 (StructuredBuffer<DofTileOffsets>),
//   tile_coords_buffer t19 (StructuredBuffer<uint2>); LocalSize [8,8,1], entry cs_main.
//   Per-tile full-res resolve: a 2x2 quadrant selects 16 spatial-resolve offsets;
//   8 half-res texels are gathered around pixel/2 and averaged (upsample_halfres.inc),
//   then alpha-normalized (max(1e-10, w)) and written.  Status: O (structure from
//   disassembly; helper call boundary Inference).

#include "inc/common.hlsl"
#include "tiles.inc.hlsl"
#include "upsample_halfres.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"

Texture2D<float4> halfres_tex : register(t15, space0);
cbuffer spatial_resolve_offsets : register(b16, space0)
{
    int4 spatial_resolve_offsets_array[512];
};
RWTexture2D<float4> img_output : register(u17, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t18, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint3 gl_WorkGroupID : SV_GroupID)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[3] + gl_WorkGroupID.x]);
    uint2 quadrant = pixel & uint2(1u, 1u);
    uint quadrant_index = quadrant.x + (quadrant.y * 2u);
    float4 upsample = upsample_halfres(halfres_tex, spatial_resolve_offsets_array, pixel, quadrant_index);
    img_output[pixel] = upsample / max(1e-10f, upsample.w).xxxx;
}
