// ==RECONSTRUCTED== shaders/ssgi/XeGTAO.hlsl | F-D
// 依据:_ssgi_ssgi.cs.hlsl_cs_main_0.bin(内嵌源路径 shaders/ssgi/XeGTAO.hlsl;
// 内联使用点:快速 acos 地平角重建
//   `((-0.15658299624919891357421875f * x) + 1.57079601287841796875f)
//   * asfloat(532487669 + (asint(1.0f - x) >> 1))`
// 在每个地平切片出现一次、积分阶段出现两次,与经典 XeGTAO 角度恢复一致)。
// 状态:O (算法/常量取自内联 SPIR-V);I (函数名——内联)。
//
// 基于地平的环境光遮蔽数学(Intel GTAO 血统),由 SSGI pass 共享。
// 所有辅助均为 static 并内联进消费方;本头文件自包含(不 include 任何内容)。

#ifndef TINYGLADE_SSGI_XEGTAO_HLSL
#define TINYGLADE_SSGI_XEGTAO_HLSL

// |x| <= 1 时 acos(x) 的快速近似,由线性修正 + 整数位技巧
// (asfloat(0x1FBD1A55... + (asint(1-x) >> 1)))构成。调用方传入 |x|;
// 符号处理在调用点完成。
float XeGTAO_Internal_FastACos(float abs_x)
{
    return ((-0.15658299624919891357421875f * abs_x) + 1.57079601287841796875f)
        * asfloat(532487669 + (asint(1.0f - abs_x) >> 1));
}

// 投影法线的加权长度,用于缩放某切片对 AO 的贡献:
// 掠射法线时贡献强,朝向法线时饱和趋向 1.0。
float XeGTAO_Internal_ProjectedNormalLen(float projected_normal_len)
{
    return lerp(projected_normal_len, 1.0f, 0.0500000007450580596923828125f);
}

#endif // TINYGLADE_SSGI_XEGTAO_HLSL
