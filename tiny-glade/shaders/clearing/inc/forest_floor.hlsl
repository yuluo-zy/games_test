// ==RECONSTRUCTED== shaders/clearing/inc/forest_floor.hlsl
// 【文件用途】林地表层(forest-floor)着色辅助函数(地面覆盖物 / 落叶混合)。
//   内嵌于 terrain_editing floor 栅格化 bin(2 个变体)。重建后的 floor 着色器
//   没有观察到包含该头文件的内联使用点,因此这里仅提供一个最小、可编译的表层。
// 【状态】U(辅助函数名/函数体为推断——未观察到内联使用点)。
// 本头文件重写只添加中文注释,代码逐字保留。

#ifndef TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL
#define TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL

// 按覆盖量 cover 把两种地面覆盖色(ground 与 litter)混合。
float3 forest_floor_blend(float3 ground, float3 litter, float cover)
{
    return lerp(ground, litter, cover);
}

#endif // TINYGLADE_CLEARING_INC_FOREST_FLOOR_HLSL
