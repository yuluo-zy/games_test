#pragma once
// ============================================================
// inc/fbm_noise.hlsl —— 分形布朗运动(FBM)噪声
// 用途:perlin() 的 2 倍频分形,带固定的 36.87° 旋转矩阵与频率 1.9。
//   消费方:_paths_path_mask_visual 等 14 个 bin。
// 入口:无(纯头文件;依赖 perlin.hlsl)
// 关键算法:2 倍频循环 amp*=0.55,pos 乘 0.8/-0.6/0.6/0.8 旋转矩阵
// ============================================================
// ==RECONSTRUCTED== shaders/inc/fbm_noise.hlsl | evidence: _paths_path_mask_visual inlined 2-octave loop (amp *= 0.55, pos = mul(float2x2(0.8,-0.6,0.6,0.8), pos * 1.9), value += amp * (-1 + 2*perlin(pos))); include order: fbm_noise follows perlin.hlsl (descendant) -> fbm_noise includes perlin.hlsl; 14 consumer bins | status: I
//
// Observation: 2-octave fractal of perlin() with a fixed 36.87-deg rotation matrix
// (cos=0.8, sin=0.6) and frequency 1.9.  Function name not embedded -> "fbm" is Inference.

#include "perlin.hlsl"

// 双倍频 FBM 噪声。
float fbm(float2 p)
{
    const float2x2 rot = float2x2(0.8f, -0.6f, 0.6f, 0.8f);
    float value = 0.0f;
    float amp = 0.5f;
    float2 pos = p;
    [unroll]
    for (int i = 0; i < 2; i++)
    {
        value += amp * (-1.0f + 2.0f * perlin(pos));
        amp *= 0.55f;
        pos = mul(rot, pos * 1.9f);
    }
    return value;
}
