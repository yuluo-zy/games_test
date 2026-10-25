#pragma once
// ======================================================================
// inc/morton.hlsl —— 莫顿码(Z 阶曲线)编码/解码
// ======================================================================
// 来源:_dof_analyze.cs.hlsl_cs_main_0 反编译的 XOR-swap 解码
//       (0x55555555 -> 0x33333333 -> 0x0F0F0F0F -> 0x00FF00FF -> 0xFFFF);
//       include 图 19 个 bin(dof/gi/grass/rdi/tgsr)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/morton.hlsl | evidence: _dof_analyze.cs.hlsl_cs_main_0 decompiled XOR-swap decode (0x55555555 -> 0x33333333 -> 0x0F0F0F0F -> 0x00FF00FF -> 0xFFFF); include graph 19 bins (dof/gi/grass/rdi/tgsr) | status: I
//
// Observation (from _dof_analyze inlined use-site, tiling a 8x8 workgroup by local index):
//   x = m & 0x55555555u; x = (x ^ (x>>1)) & 0x33333333u; x = (x ^ (x>>2)) & 0x0F0F0F0Fu;
//   x = (x ^ (x>>4)) & 0x00FF00FFu; x = (x ^ (x>>8)) & 0xFFFFu;  (same for y from m>>1)
// Encode is the standard inverse (no direct opcode use observed); marked Inference.

// 把 16 位 x/y 交错编码为 32 位莫顿码(标准编码,无直接 opcode 观测)。
uint morton_encode(uint2 p)
{
    uint x = p.x & 0xFFFFu;
    x = (x | (x << 8u)) & 0x00FF00FFu;
    x = (x | (x << 4u)) & 0x0F0F0F0Fu;
    x = (x | (x << 2u)) & 0x33333333u;
    x = (x | (x << 1u)) & 0x55555555u;
    uint y = p.y & 0xFFFFu;
    y = (y | (y << 8u)) & 0x00FF00FFu;
    y = (y | (y << 4u)) & 0x0F0F0F0Fu;
    y = (y | (y << 2u)) & 0x33333333u;
    y = (y | (y << 1u)) & 0x55555555u;
    return x | (y << 1u);
}

// 莫顿码解码为 x/y(XOR-swap 展开,来自 _dof_analyze 内联点)。
uint2 morton_decode(uint m)
{
    uint x = m & 0x55555555u;
    x = (x ^ (x >> 1u)) & 0x33333333u;
    x = (x ^ (x >> 2u)) & 0x0F0F0F0Fu;
    x = (x ^ (x >> 4u)) & 0x00FF00FFu;
    x = (x ^ (x >> 8u)) & 0xFFFFu;
    uint y = (m >> 1u) & 0x55555555u;
    y = (y ^ (y >> 1u)) & 0x33333333u;
    y = (y ^ (y >> 2u)) & 0x0F0F0F0Fu;
    y = (y ^ (y >> 4u)) & 0x00FF00FFu;
    y = (y ^ (y >> 8u)) & 0xFFFFu;
    return uint2(x, y);
}
