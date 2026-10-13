// ==RECONSTRUCTED== shaders/water/ice_shared.inc.hlsl
// SOURCE PATH : shaders/water/ice_shared.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 6 bins):
//   _ice_plane.raster.hlsl_{b903e,f0adff}.bin, _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin,
//   _water_ice_refl_march.cs.hlsl_cs_main_0.bin, _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin,
//   _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
// Status: O (half-precision pack/unpack helpers from inlined use-sites in the ice
//   consumers: pack_half2x16 = f32tof16 | <<16, unpack_half2x16 = f16tof32(low/high));
//   O (FrexpStructType name from raw-per-bin debug_names); I (frexp helper body —
//   inlined in SPIR-V, mirrors the adjudicated curve.hlsl incb_frexp form)
// MERGE (water-ice review water/ice_shared.inc.hlsl, inc-fix #6): B's FrexpStructType
//   placement (the ice_refl_resolve bin carries the name and its closure has this file
//   but NOT inc/curve.hlsl) + the frexp workaround (dxc's native frexp SPIR-V lowering
//   is broken) + A's half pack/unpack helpers.
//
// 冰面反射/折射家族共用辅助函数。

#ifndef TINYGLADE_WATER_ICE_SHARED_HLSL
#define TINYGLADE_WATER_ICE_SHARED_HLSL

#include "inc/pack_unpack.hlsl"

// 把两个半精度数(各在 [0,1])打包进一个 uint(射线/切线编码)。
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}

// 从一个 uint 的低/高半字解出 half2。
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

// ice_refl_resolve 用 frexp 把累加的辐射度拓宽到 8 位指数范围。
// dxc v1.10.2605.37 的原生 `frexp` SPIR-V 降级是坏的,冰面家族自带实现
// (镜像已裁定的 inc/curve.hlsl 形式);宏把本 TU 里的 `frexp(x, out e)` 路由到这里。
#define frexp ice_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 ice_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    // 把尾数归一到 [0.5, 1)(C 风格 frexp)。
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    // 零 / 次正规 / 无穷 / NaN:尾数 = x,指数 = 0。
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

#endif // TINYGLADE_WATER_ICE_SHARED_HLSL
