// ==RECONSTRUCTED== shaders/smoke.inc.hlsl | F-V | 内嵌于 nani_cull_and_bucket_smoke.cs (2) + nani_smoke.raster (2)
// 烟雾实例共享记录类型。成员布局/名称逐字取自消费方 bin 的 OpMemberName 表:
//   SmokeInstanceData: xform (Affine3Packed, 12 个 float), smoke_type (uint), seed (uint),
//     thickness (float), pad0 (uint)  (64 字节紧凑布局)。
// Affine3Packed 定义在 inc/utils.hlsl (12-float 列主序 3x4 变换)。
// 函数体在此轮为 Unknown(物理寻址的消费方;nani 系列会进一步细化)。

#ifndef TINYGLADE_SMOKE_INC_HLSL
#define TINYGLADE_SMOKE_INC_HLSL

#include "inc/utils.hlsl"

// 单根烟雾柱的实例记录。
struct SmokeInstanceData
{
    Affine3Packed xform;    // 烟雾柱的仿射变换(位置/朝向)
    uint smoke_type;        // 烟雾类型
    uint seed;              // 随机种子(驱动形状抖动)
    float thickness;        // 烟雾厚度/浓度
    uint pad0;              // 填充对齐
};

#endif // TINYGLADE_SMOKE_INC_HLSL
