// ==RECONSTRUCTED== shaders/roof_material_id.inc.hlsl | F-V | 内嵌于 nani_roof_floor.raster (2) + nani_stairs_floor.raster (2)
// 屋顶-楼板共享记录类型 + 材质 id 分发。InstanceData 布局/名称逐字取自消费方
// bin 的 OpMemberName 表:
//   position f3 +0, wall_id u +12, basis0_xz f2 +16, scale f2 +24, roof_animation f +32,
//   shape u +36, double_sided u +40, is_roof_bottom u +44, material_id u +48,
//   override_color_id u +52, pattern_id u +56, pad_1 u +60  (64B 记录)。
// 材质分发函数体(material_id -> 木质/石质/瓦片材质)在 wave-2 的 nani_roof_floor
// 光栅化器中内联,在该重建完成前此处标记为 Unknown。

#ifndef TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL
#define TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL

// 屋顶-楼板件(屋顶/阁楼/楼梯楼板)的逐实例记录。
struct RoofFloorInstanceData
{
    float3 position;        // 实例世界位置
    uint wall_id;           // 所属墙体 id
    float2 basis0_xz;       // 水平面上的基向量(地面投影朝向)
    float2 scale;           // 楼板缩放
    float roof_animation;   // 屋顶动画进度
    uint shape;             // 形状类别
    uint double_sided;      // 是否双面渲染
    uint is_roof_bottom;    // 是否为屋顶底面
    uint material_id;       // 材质 id(驱动分发)
    uint override_color_id; // 覆盖颜色 id
    uint pattern_id;        // 图案 id
    uint pad_1;             // 填充对齐
};

// 按 material_id 为屋顶件选取楼板材质。
// (函数体 Unknown:已在 wave-2 的 nani_roof_floor 光栅化器中内联;分发按
// material_id/shape 在木质楼板、石质楼板与瓦片采样器之间切换。)
void get_roof_material_params(uint material_id, out float3 albedo, out float roughness)
{
    // 占位实现:白色高光材质,等待真实分发恢复后替换。
    albedo = float3(1.0f, 1.0f, 1.0f);
    roughness = 1.0f;
}

#endif // TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL
