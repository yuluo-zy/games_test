// ==RECONSTRUCTED== shaders/gi/restir.inc.hlsl | status: O (reservoir texel packing from
// _gi_update/_gi_eval disassembly); I (helper names)
// MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (packing helpers;
// B kept the file U with only two unused constants — both fold to identical literals).
// Evidence: the GI ReSTIR chain stores one reservoir per half-res texel as a uint2:
//   x = packed payload pixel id (px | py<<16), y = two packed half16 weights. The
//   reservoir's unbiased weight W is read back as the high half of y (eval reads
//   restir_reservoir_tex.y >> 16). update initializes y = packHalf2x16(1,0)<<16|packHalf2x16(1,0).
// Include-only file (4 gi bins), no outgoing includes.

#ifndef TINYGLADE_GI_RESTIR_INC_HLSL
#define TINYGLADE_GI_RESTIR_INC_HLSL

// Pack a pixel id into a 32-bit payload (low 16 x, high 16 y).
uint pack_restir_payload(uint2 pixel)
{
    return pixel.x | (pixel.y << 16u);
}

// Unpack a payload pixel id.
uint2 unpack_restir_payload(uint payload)
{
    return uint2(payload & 0xFFFFu, payload >> 16u);
}

// Pack two float weights into one uint (two half16 slots).
uint pack_restir_weights(float2 w)
{
    return f32tof16(w.x) | (f32tof16(w.y) << 16u);
}

// Unpack two half16 weights.
float2 unpack_restir_weights(uint packed)
{
    return float2(f16tof32(packed & 0xFFFFu), f16tof32(packed >> 16u));
}

#endif // TINYGLADE_GI_RESTIR_INC_HLSL
