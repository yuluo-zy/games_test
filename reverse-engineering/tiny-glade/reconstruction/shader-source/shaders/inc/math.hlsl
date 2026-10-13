// ==RECONSTRUCTED== shaders/inc/math.hlsl | evidence: OpLine attribution across all bins (heavy OpExtInst/sqrt, OpDot, OpFMul, OpFDiv, VectorShuffle — RT/GI consumers like _gi_trace_gi_candidates, _water_ice_deferred_rt); include-graph edges math.hlsl -> bicubic.hlsl, math.hlsl -> quad_mesh.hlsl, math.hlsl -> unjitter_taa.hlsl | status: I (function names/selection Inference — helpers inlined; the operation mix is consistent with a general math utility set)
//
// Shared math helpers (safe normalization, squared lengths, clamping and
// small geometric utilities). Uses HLSL intrinsics where available.

#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL

// Clamp a value to [0,1].
float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}

// Square of the length (avoids the sqrt).
float length_squared(float3 v)
{
    return dot(v, v);
}

// Normalize, guarding against division by zero.
float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}

// Safe reciprocal with a small floor.
float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}

// Safe reciprocal, preserving sign of a near-zero value.
float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif // TINYGLADE_INC_MATH_HLSL
