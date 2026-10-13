#pragma once
// ==RECONSTRUCTED== shaders/inc/curve.hlsl | evidence: _taa.cs.hlsl_cs_main_0 and _tgsr.cs.hlsl_cs_main_* inlined FrexpStructType (float3 mantissa, int3 exponent) + frexp(x, out exponent) then x + exp2(exponent - float3(8,8,7)); raw-per-bin.jsonl debug_names "FrexpStructType" (6 bins); include graph 11 bins | status: U
//
// Observation (identical in _taa and _tgsr): FrexpStructType is a float3/int3 mantissa/exponent pair;
// frexp is C-style (x == mantissa * 2^exponent, mantissa in [0.5,1)), returning the mantissa and
// filling the out exponent.  Consumers then quantize history as x + exp2(exponent - (8,8,7)).
// The name "curve" suggests a wrapping quantization function; its exact body is Inference --
// reconstructed to match the observed inlined x + exp2(exponent - (8,8,7)) pattern.

// NOTE (toolchain): dxc v1.10.2605.37 ships a native `frexp` intrinsic whose SPIR-V lowering is
// broken (OpConvertSToF on a non-int input -> "generated SPIR-V is invalid").  The original game
// predates that intrinsic and defines a custom `frexp` here.  To keep consumer source-level calls
// `frexp(x, out exponent)` resolving to this implementation (not the broken builtin), the function
// is defined as `incb_frexp` and the macro below rewrites `frexp` tokens to it for the rest of the TU.

#define frexp incb_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 incb_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    // Normalize mantissa to [0.5, 1) (C-style frexp).
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    // Zero / subnormal / inf / nan: mantissa = x, exponent = 0.
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

float3 curve(float3 x)
{
    // Note: passing f.exponent directly as the out-param of frexp triggers a dxc 1.10 SPIR-V
    // generator bug (OpConvertSToF on a non-int vector); a local out-param avoids it while
    // producing equivalent SPIR-V to the observed inlined taa/tgsr pattern.
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}
