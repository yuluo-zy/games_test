#pragma once
// ============================================================
// inc/curve.hlsl —— frexp 包装与曲线量化
// 用途:自定义 C 风格 frexp(float3 尾数 + int3 指数),供时间积累着色器
//   量化历史值:x + exp2(指数 - (8,8,7))。
//   消费方:_taa、_tgsr。
// 入口:无(纯头文件)
// 工具链说明:dxc 1.10 内置 frexp 的 SPIR-V lowering 有 bug,故自定义为
//   incb_frexp,并用宏把 `frexp` 词法改写指到它,避免命中坏内建。
// ============================================================
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

// C 风格 frexp 的结果结构:尾数与指数。
struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

// C 风格 frexp:返回尾数(∈[0.5,1)),并填充 out 指数。
float3 incb_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    // 把尾数归一化到 [0.5, 1)(C 风格 frexp)。
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    // 零 / 非正规 / 无穷 / NaN:尾数 = x,指数 = 0。
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

// 曲线量化:x + exp2(指数 - (8,8,7)),与 taa/tgsr 内联模式一致。
float3 curve(float3 x)
{
    // 注意:直接把 f.exponent 作为 frexp 的 out 参数会触发 dxc 1.10 SPIR-V
    // 生成器 bug(对非 int 向量做 OpConvertSToF);用本地 out 参数可避开,
    // 同时产生与观察到的内联 taa/tgsr 模式等价的 SPIR-V。
    FrexpStructType f;
    int3 e;
    f.mantissa = frexp(x, e);
    f.exponent = e;
    return x + exp2(float3(f.exponent) - float3(8.0f, 8.0f, 7.0f));
}
