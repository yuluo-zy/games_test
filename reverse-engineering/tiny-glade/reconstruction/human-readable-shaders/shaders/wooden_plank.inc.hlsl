// ==RECONSTRUCTED== shaders/wooden_plank.inc.hlsl | F-V | embedded in nani_cull_and_bucket_wooden_plank.cs (2) + nani_wooden_plank.raster (2)
// 中文说明:
//   木板的共享记录类型定义。成员布局/名字按消费者 bin 的 OpMemberName 表原样恢复:
//     WoodenPlank: transform(Affine3Packed,12 个 float)、gate_seed(uint)、
//       piece_seed(uint)、wall_id(uint)、flags(uint)  —— 64 字节紧凑布局。
//   Affine3Packed 定义于 inc/utils.hlsl。函数体本轮未恢复(物理寻址的消费者
//   nani 家族会继续细化)。

#ifndef TINYGLADE_WOODEN_PLANK_INC_HLSL
#define TINYGLADE_WOODEN_PLANK_INC_HLSL

#include "inc/utils.hlsl"

// 单块木板实例记录(木板挂在大门/拱门的木墙上)。
struct WoodenPlank
{
    Affine3Packed transform;
    uint gate_seed;
    uint piece_seed;
    uint wall_id;
    uint flags;
};

#endif // TINYGLADE_WOODEN_PLANK_INC_HLSL
