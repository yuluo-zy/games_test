// ======================================================================
// inc/samplers.hlsl —— 全局静态采样器集合
// ======================================================================
// 用途:所有着色器共享的采样器;每个着色器只绑定实际使用的采样器。
// 命名约定:三个字母 = 最小滤波 / 最大滤波 / 环绕方式:
//   L=线性, N=最近邻, C=钳位, R=重复, B=边界。
//   sampler_shadow_linear 是比较采样器,用于阴影贴图比较。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/samplers.hlsl | evidence: catalog.json + raw-per-bin.jsonl across all 404 bins — sampler binding names/sets (sampler_lnc s0, sampler_llr s1, sampler_nnc s2, sampler_llc s3, sampler_llb s4, sampler_default s9, sampler_mipmap s10, sampler_shadow_nearest s11, sampler_shadow_linear s12, all space0); only used samplers appear per bin (dxc drops unused declarations); include-graph edge common.hlsl -> samplers.hlsl and samplers.hlsl -> debug_utils.hlsl | status: O (names/sets/bindings verbatim); I (filter/wrap state — sampler state lives CPU-side, not in SPIR-V; encoded from the L/N/C/R/B name convention)
//
// Static sampler set shared by all shaders. Each shader binds only the samplers
// it actually uses. Name convention: three letters = min-filter / mag-filter /
// wrap: L=linear, N=nearest, C=clamp, R=repeat, B=border. Shadow samplers are
// used for shadow-map comparisons (sampler_shadow_linear is a comparison sampler).

#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#include "debug_utils.hlsl"

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif // TINYGLADE_INC_SAMPLERS_HLSL
