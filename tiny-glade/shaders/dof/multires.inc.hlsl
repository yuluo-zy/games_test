// ==RECONSTRUCTED== shaders/dof/multires.inc.hlsl | B (dof-aa review, 2026-10-13): real f16x2 pack/unpack helpers matching the target masking form.
//
// The ring-gather consumers inline a half2 pack/unpack (f32tof16 x2 | <<16 and
// the reverse) used to store the CoC ring taps in the groupshared
// ring_samples table.  Consumers call these with the masking form (& 0xFFFF) to
// match the target's OpBitwiseAnd count.
//
// === 中文说明 ===
// 半精度 f16x2 打包/解包辅助函数。环形采集的消费者用它把 CoC 环采样存储到
//   groupshared ring_samples 表;调用时使用 (& 0xFFFF) 掩码形式以匹配目标的
//   OpBitwiseAnd 数量。本文件不声明可改名符号(被多个 DoF pass include)。

#ifndef TINYGLADE_DOF_MULTIRES_INC_HLSL
#define TINYGLADE_DOF_MULTIRES_INC_HLSL

// 把两个 half 打包进一个 uint(f16x2),与环采集存储形式一致:
//   ring_samples[lane][i] = pack_half2(float2(coc_a, 0)) | (pack_half2(float2(coc_b, 0)) << 16)。
uint dof_pack_half2(float2 v)
{
    uint2 packed = f32tof16(v);
    return packed.x | (packed.y << 16u);
}

// 解包一个打包 uint 的低/高 half;消费者读 .x 通道(即 CoC)。
float2 dof_unpack_half2(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

#endif // TINYGLADE_DOF_MULTIRES_INC_HLSL
