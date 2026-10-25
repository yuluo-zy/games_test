// ======================================================================
// inc/math.hlsl —— 通用数学辅助公共头文件
// ======================================================================
// 用途:安全归一化、平方长度、截断与小型几何工具;可用 HLSL 内建时使用内建。
// 来源:跨所有 bin 的 OpLine 归属(大量 OpExtInst/sqrt、OpDot、OpFMul、OpFDiv、
//       VectorShuffle —— RT/GI 消费者如 _gi_trace_gi_candidates、_water_ice_deferred_rt)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/math.hlsl | evidence: OpLine attribution across all bins (heavy OpExtInst/sqrt, OpDot, OpFMul, OpFDiv, VectorShuffle — RT/GI consumers like _gi_trace_gi_candidates, _water_ice_deferred_rt); include-graph edges math.hlsl -> bicubic.hlsl, math.hlsl -> quad_mesh.hlsl, math.hlsl -> unjitter_taa.hlsl | status: I (function names/selection Inference — helpers inlined; the operation mix is consistent with a general math utility set)
//
// Shared math helpers (safe normalization, squared lengths, clamping and
// small geometric utilities). Uses HLSL intrinsics where available.

#ifndef TINYGLADE_INC_MATH_HLSL
#define TINYGLADE_INC_MATH_HLSL

// 截断到 [0,1]。
float clamp01(float v)
{
    return clamp(v, 0.0f, 1.0f);
}

float3 clamp01(float3 v)
{
    return clamp(v, 0.0f, 1.0f);
}

// 长度平方(省去开方)。
float length_squared(float3 v)
{
    return dot(v, v);
}

// 归一化,并防止除零。
float3 safe_normalize(float3 v)
{
    float len_sq = dot(v, v);
    float len = sqrt(len_sq);
    return len > 1.0e-6f ? v / len : float3(0.0f, 0.0f, 0.0f);
}

// 带小下限的安全倒数。
float safe_rcp(float v)
{
    return 1.0f / max(abs(v), 1.0e-6f);
}

// 安全倒数,保留接近零值的符号。
float rcp_keep_sign(float v)
{
    return sign(v) * safe_rcp(v);
}

#endif // TINYGLADE_INC_MATH_HLSL
