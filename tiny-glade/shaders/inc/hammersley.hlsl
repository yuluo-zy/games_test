#pragma once
// ============================================================
// inc/hammersley.hlsl —— Hammersley 低差异序列
// 用途:生成 (i/N, 范德科彭特逆) 的均匀采样点。
//   消费方:_lut_brdf_fg(用 2π 缩放做 BRDF 积分采样),77 个 bin。
// 入口:无(纯头文件)
// 关键算法:radicalInverse_VdC 位反转 + 2^-32 缩放
// ============================================================
// ==RECONSTRUCTED== shaders/inc/hammersley.hlsl | evidence: _lut_brdf_fg.cs.hlsl_cs_main_0 decompiled bit-reversal + float(bits)*2.3283064e-10 (2^-32) + i/N pattern; include graph 77 consumer bins | status: I
//
// Observation (from _lut_brdf_fg inlined use-site):
//   u1 = float(i)/float(N)  (loop used sqrt(float(i)*0.0009765625) with N=1024)
//   u2 = radicalInverse_VdC(i)  (bit-reversal then float(bits)*2^-32); callers scale by 2*PI
// Function names not embedded (dxc inlines helpers); "hammersley"/"radicalInverse_VdC" are the
// de-facto conventions matched to the observed opcode pattern.  Function names = Inference.

// 基数 2 的范德科彭特逆,[0,1)。
float radicalInverse_VdC(uint bits)
{
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return float(bits) * 2.3283064365386963e-10f;
}

// Hammersley 采样点:i ∈ [0,N)。
float2 hammersley(uint i, uint N)
{
    return float2(float(i) / float(N), radicalInverse_VdC(i));
}
