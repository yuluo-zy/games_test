// ==RECONSTRUCTED== shaders/dof/upsample_halfres.inc.hlsl | B (dof-aa review, 2026-10-13): helper content; the include edge exists in the 3 consumer bins so a live helper is more faithful.
//
// fullres/fullres_upsample bins inline an 8-texel half-res gather:
//   fullres_pixel/2 + spatial_resolve_offsets[quadrant*16+i].xy, averaged *1/8,
//   then the caller renormalizes alpha.  Function name/signature are Inference
//   (inlined in consumers); the body reproduces the observed inlined opcodes.

#ifndef TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
#define TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL

// Gather 8 half-res texels around the 2x2 full-res quadrant and average.
float4 upsample_halfres(Texture2D<float4> src_tex,
                        int4 resolve_offsets[512],
                        uint2 fullres_pixel,
                        uint quadrant)
{
    float4 sum = 0.0f;
    for (uint i = 0u; i < 8u; ++i)
    {
        sum += src_tex[(fullres_pixel / uint2(2u, 2u))
            + uint2(resolve_offsets[(quadrant * 16u) + i].xy)];
    }
    return sum * 0.125f.xxxx;
}

#endif // TINYGLADE_DOF_UPSAMPLE_HALFRES_INC_HLSL
