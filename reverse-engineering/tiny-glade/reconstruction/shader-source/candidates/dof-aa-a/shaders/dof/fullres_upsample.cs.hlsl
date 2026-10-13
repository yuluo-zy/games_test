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
//     names        : halfres_tex, spatial_resolve_offsets, img_output, DofTileOffsets, tile_offsets_buffer, tile_coords_buffer, cs_main
// ==RECONSTRUCTED== shaders/dof/fullres_upsample.cs.hlsl | dof-aa-a | status: O (structure from SPIR-V)
//   Full-res reconstruction of the half-res DoF output: per-tile 8-tap spatial resolve using
//   the offsets table, alpha-normalized.

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"
#include "dof/upsample_halfres.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"

cbuffer spatial_resolve_offsets : register(b16, space0)
{
    int4 spatial_resolve_offsets_array[512];
};

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t18, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t19, space0);
Texture2D<float4> halfres_tex : register(t15, space0);
RWTexture2D<float4> img_output : register(u17, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    uint2 pixel = (dispatch_id.xy & uint2(7u, 7u))
        + uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[3] + group_id.x];
    uint2 subpixel = pixel & uint2(1u, 1u);
    uint subpixel_index = subpixel.x + (subpixel.y * 2u);

    float4 acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    [loop]
    for (uint i = 0u; i < 8u; ++i)
    {
        acc += halfres_tex.Load(int3((pixel / uint2(2u, 2u))
            + uint2(spatial_resolve_offsets_array[subpixel_index * 16u + i].xy), 0));
    }

    float4 resolved = acc * 0.125f;
    img_output[pixel] = resolved / max(1.000000013351432e-10f, resolved.w);
}
