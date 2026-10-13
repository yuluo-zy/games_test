// ==SKELETON==
// SOURCE PATH : shaders/dof/upsample_halfres.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 3 bins):
//   _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin,
//   _dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin,
//   _dof_fullres_upsample.cs.hlsl_cs_main_0.bin
//
// ==RECONSTRUCTED== shaders/dof/upsample_halfres.inc.hlsl
// Evidence: fullres/fullres_upsample bins inline an 8-texel half-res gather:
//   fullres_pixel/2 + spatial_resolve_offsets[quadrant*16+i].xy, averaged *1/8,
//   then the caller renormalizes alpha.  Function name/signature are Inference
//   (inlined in consumers); the body reproduces the observed inlined opcodes.
// Status: I (name/signature), O (inlined body from disassembly).

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
