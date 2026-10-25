// ==RECONSTRUCTED== shaders/garden/defs.inc.hlsl
// 依据:原始逐 bin 调试类型/OpMemberName(_garden_spawn_* + _garden_read_bush_positions_for_cpu + _nani_bush/_nani_cull_and_bucket_*):
// BushXform{Affine3Packed transform}, FlowerXform{Affine3Packed transform,float3 color,uint flower_type}
// (64 字节记录,color @48, flower_type @60);Affine3Packed 本身来自 inc/utils.hlsl
// (消费方先 include utils 再包含本文件 —— defs 不 include 任何内容)。
// 状态:O (结构布局逐字取自 OpMemberName);I (FlowerXform 字段名 color/flower_type 为推断——偏移吻合)

#ifndef TINYGLADE_GARDEN_DEFS_HLSL
#define TINYGLADE_GARDEN_DEFS_HLSL

// Affine3Packed 由消费方先前 include 的 inc/utils.hlsl 提供。

// 灌木实例变换记录。
struct BushXform
{
    Affine3Packed transform;
};

// 花朵实例变换记录:变换 + 颜色 + 花朵类型。
struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

#endif // TINYGLADE_GARDEN_DEFS_HLSL
