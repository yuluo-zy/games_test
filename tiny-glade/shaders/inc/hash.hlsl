// ============================================================
// inc/hash.hlsl —— 整数/浮点哈希辅助
// 用途:为逐实例/逐纹素的确定性随机提供哈希与均匀随机数。
//   `hash` 是经典 5 步整数哈希;`rand01` 把哈希位变造成 [0,1) 均匀浮点;
//   `hash_angle` 用黄金比序列把索引映射为伪随机角度。
//   消费方:250 个 bin(如 _billboard_forest vs_main 内联)。
// 入口:无(纯头文件;依赖 math_const.hlsl)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/hash.hlsl | evidence: _billboard_forest raster vs_main inlined use (uint hash: x+(x<<10), x^(x>>6), x+(x<<3), x^(x>>11), x+(x<<15) applied over instance seed; rand01: asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0; angle: frac(0.9 + float(x) * 1.618034f) * 6.283185482025146484375f) — matches include-graph 250 consumer bins | status: O (algorithm/value patterns from inlined SPIR-V); I (function names — helpers are inlined, names not embedded)
//
// Integer + float hash helpers used for deterministic per-instance / per-texel
// randomness. `hash` is the classic 5-step integer hash; `rand01` bitcasts a
// hash to a uniform float in [0,1); `hash_angle` maps an index to a pseudo-random
// angle via golden-ratio sequence.

#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

#include "math_const.hlsl"

// 5 步整数哈希(雪崩)。直接用于实例/像素索引。
uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}

// 从哈希 uint 得到 [0,1) 均匀浮点(尾数位变造技巧)。
float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

// 从索引得到 [0, 2π) 的伪随机角度(黄金比哈希)。
float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}

// 从浮点种子得到 [0,1) 伪随机浮点。
float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif // TINYGLADE_INC_HASH_HLSL
