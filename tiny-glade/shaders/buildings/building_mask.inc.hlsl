// ==RECONSTRUCTED== shaders/buildings/building_mask.inc.hlsl
// 依据:include 图直接边 -> inc/foliage_exclusion.hlsl;消费方为 rasterize_buildings /
// gauss_blur_building_mask / garden 生成器;rasterize_buildings 内联形状测试
// (圆 shape_id 0,朝向包围盒 shape_id 1)针对 buildings_buffer。
// 状态:O (include 边,建筑形状角色);O/I (函数名/函数体为推断——内联)

#ifndef TINYGLADE_BUILDING_MASK_HLSL
#define TINYGLADE_BUILDING_MASK_HLSL

// 建筑足迹记录,布局与 rasterize_buildings 读取方式完全一致(32 字节步长)。
// 字段由 CPU 侧建筑列表打包。
struct BuildingShape
{
    float2 center;       // 中心
    float2 dims;         // 尺寸
    float2 orientation;  // 朝向(单位方向)
    int shape_id;        // 形状类型(0=圆,1=朝向盒)
    int pad0;            // 填充
};

// 世界位置是否落在圆形建筑足迹内(shape_id 0):半径 = dims.x + 0.75。
bool is_building_circle(float2 world_xz, float2 center, float radius)
{
    return distance(world_xz, center) < radius;
}

// 世界位置是否落在朝向包围盒建筑足迹内(shape_id 1)。
// orientation 为单位方向;包围盒半尺寸为 dims 按 0.5 缩放。
bool is_building_box(float2 world_xz, float2 center, float2 dims, float2 orientation)
{
    float2 local = mul(world_xz - center,
                       float2x2(float2(orientation.x, -orientation.y),
                                float2(orientation.y, orientation.x)));
    float half_w = (dims.x + 1.0f) * 0.5f;
    float half_h = (dims.y + 1.0f) * 0.5f;
    return abs(local.x) <= half_w && abs(local.y) <= half_h;
}

// 建筑足迹是否覆盖该世界位置。
bool is_building(BuildingShape building, float2 world_xz)
{
    if (building.shape_id == 0)
    {
        return is_building_circle(world_xz, building.center, building.dims.x + 0.75f);
    }
    else if (building.shape_id == 1)
    {
        return is_building_box(world_xz, building.center, building.dims, building.orientation);
    }
    return false;
}

// 单个足迹的建筑遮罩标量:足迹内 1.0,足迹外 0.0。
// 推断(裁定 inc-fix #5 "get_*_mask()" 命名;消费方内联该测试)。
float get_building_mask(BuildingShape building, float2 world_xz)
{
    return is_building(building, world_xz) ? 1.0f : 0.0f;
}

#endif // TINYGLADE_BUILDING_MASK_HLSL
