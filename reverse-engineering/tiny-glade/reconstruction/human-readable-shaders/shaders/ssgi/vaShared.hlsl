// ==RECONSTRUCTED== shaders/ssgi/vaShared.hlsl | F-D
// 依据:_ssgi_ssgi.cs.hlsl_cs_main_0.bin(内嵌源路径 shaders/ssgi/vaShared.hlsl;
// 内联使用点:深度->视图 -Z 重建 `-1.0f / (depth * -clip_to_view[3][2])`
// 出现在 4 点深度聚合与每个切片采样点)。
// 状态:O (算法取自内联 SPIR-V);I (函数名——内联)。
//
// SSGI/GTAO pass 共享的视图空间重建辅助。所有辅助均为 static 并内联进消费方;
// 本头文件自包含(不 include 任何内容)。

#ifndef TINYGLADE_SSGI_VASHARED_HLSL
#define TINYGLADE_SSGI_VASHARED_HLSL

// 把 [0,1] 深度纹素转换为视图空间 -Z 距离。`clip_to_view_32`
// 为 clip_to_view[3][2] 矩阵元素(把 NDC 深度映射到视图空间 Z 的
// 逆投影常量列)。
float va_depth_to_view_z(float clip_to_view_32, float depth)
{
    return (-1.0f) / (depth * (-clip_to_view_32));
}

#endif // TINYGLADE_SSGI_VASHARED_HLSL
