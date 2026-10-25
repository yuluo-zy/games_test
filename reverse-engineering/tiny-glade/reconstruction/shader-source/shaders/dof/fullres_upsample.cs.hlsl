// ==RECONSTRUCTED== shaders/dof/fullres_upsample.cs.hlsl | B (dof-aa review, 2026-10-13: tie, slight B; calls the upsample_halfres() helper from upsample_halfres.inc) + Rgba16f fix: img_output RWTexture2D<half4> (target format operand Rgba16f).
//   Full-res reconstruction of the half-res DoF output: per-tile 8-tap spatial resolve using
//   the offsets table, alpha-normalized.

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"
#include "dof/upsample_halfres.inc.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"

Texture2D<float4> halfres_tex : register(t15, space0);
cbuffer spatial_resolve_offsets : register(b16, space0)
{
    int4 spatial_resolve_offsets_array[512];
};
RWTexture2D<half4> img_output : register(u17, space0);
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
