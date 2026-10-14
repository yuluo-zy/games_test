// ============================================================
// inc/common.hlsl —— 伞形头文件
// 用途:几乎所有着色器都 include 它,统一引入共享采样器集合。
//   额外共享助手应放在各自专属头文件里。
// 入口:无(纯头文件)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/common.hlsl | evidence: include-graph direct edge common.hlsl -> samplers.hlsl (observed in 252 consumer mains); embedded in 402 bins as the umbrella header; OpLine attribution shows small FMul code in some ps (possible shared color helper, not pinned) | status: O (includes samplers.hlsl); U (any additional helper bodies not pinned by evidence — left out rather than invented)
//
// Umbrella header included by nearly every shader. Pulls in the shared
// sampler set; additional shared helpers belong in their specific headers.

#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#include "samplers.hlsl"

#endif // TINYGLADE_INC_COMMON_HLSL
