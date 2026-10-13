// ==RECONSTRUCTED== shaders/dof/multires.inc.hlsl | B (dof-aa review, 2026-10-13): real f16x2 pack/unpack helpers matching the target masking form.
//
// The ring-gather consumers inline a half2 pack/unpack (f32tof16 x2 | <<16 and
// the reverse) used to store the CoC ring taps in the groupshared
// ring_samples table.  Consumers call these with the masking form (& 0xFFFF) to
// match the target's OpBitwiseAnd count.

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
