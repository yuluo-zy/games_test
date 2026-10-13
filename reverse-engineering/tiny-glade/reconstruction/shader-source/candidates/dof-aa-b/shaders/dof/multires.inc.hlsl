// ==SKELETON==
// SOURCE PATH : shaders/dof/multires.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 10 bins):
//   _dof_fullres.cs.hlsl_cs_main_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin,
//   _dof_fullres_all_lod0.cs.hlsl_cs_main_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin,
//   _dof_fullres_all_lod1.cs.hlsl_cs_main_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin,
//   _dof_halfres.cs.hlsl_cs_halfres{,_all}_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin
//
// ==RECONSTRUCTED== shaders/dof/multires.inc.hlsl
// Evidence: embedded before the shared inc chain in the fullres/halfres bins
// (pure helpers: no sampler/texture references), matching a math-only include.
// The ring-gather consumers inline a half2 pack/unpack (f32tof16 x2 | <<16 and
// the reverse) used to store the 17 CoC ring taps in the groupshared
// ring_samples[64][9] table.  Names/body Inference (inlined).  Status: I.

#ifndef TINYGLADE_DOF_MULTIRES_INC_HLSL
#define TINYGLADE_DOF_MULTIRES_INC_HLSL

// Pack two halves into one uint (f16x2), as observed in the ring-gather stores:
// ring_samples[lane][i] = pack_half2(float2(coc_a, 0)) | (pack_half2(float2(coc_b, 0)) << 16).
uint dof_pack_half2(float2 v)
{
    uint2 packed = f32tof16(v);
    return packed.x | (packed.y << 16u);
}

// Unpack the low/upper half of a packed uint; the consumers read .x (the CoC).
float2 dof_unpack_half2(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

#endif // TINYGLADE_DOF_MULTIRES_INC_HLSL
