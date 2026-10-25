// ==RECONSTRUCTED== shaders/water/ice_shared.inc.hlsl | evidence: raw-per-bin.jsonl debug_names
// "FrexpStructType" on _water_ice_refl_resolve.cs.hlsl (the only ice-family consumer that needs a
// frexp mantissa/exponent quantize); embedded source list for the 6 ice bins
// (_ice_plane raster x2, _water_ice_deferred_rt, _water_ice_refl_march, _water_ice_refl_resolve,
// _water_ice_refr_resolve) includes this file but NOT inc/curve.hlsl -> the ice family defines its
// own FrexpStructType here (mirrors the adjudicated curve.hlsl form). status: O (struct), O/I
// (helper names inferred; inlined in SPIR-V).
//
// Ice-surface shared declarations. Kept intentionally small: the exact original body is inlined in
// consumers, so this only needs to supply the types/helpers those consumers reference.

#ifndef TINYGLADE_WATER_ICE_SHARED_INC_HLSL
#define TINYGLADE_WATER_ICE_SHARED_INC_HLSL

// frexp pair used by ice_refl_resolve to widen accumulated radiance into 8-bit-exponent range.
struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

#define frexp ice_frexp

float3 ice_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

// Quantize a radiance value by rounding its exponent up to 8 bits (the ice resolve consumers
// reconstruct history as x + exp2(exponent - float3(8,8,7))).
float3 ice_quantize_radiance(float3 x)
{
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}

// Low 16 bits of a packed hit word are a half-float scale (ice refl/refr rays pack a per-hit
// cone scale); the consumers unpack only the x component.
float unpack_half(uint packed)
{
    return f16tof32(packed & 0xFFFFu);
}

#endif // TINYGLADE_WATER_ICE_SHARED_INC_HLSL
