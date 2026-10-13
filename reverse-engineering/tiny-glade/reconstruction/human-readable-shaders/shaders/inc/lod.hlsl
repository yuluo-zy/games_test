// ======================================================================
// inc/lod.hlsl —— 网格 LOD 选择公共头文件
// ======================================================================
// 用途:被 _nani_cull_and_bucket / _grass_calc_tiles 用于按距离在
//       grass/static LOD 之间选择。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/lod.hlsl | evidence: raw-per-bin (_nani_cull_and_bucket/_grass_calc_tiles) struct{first_index,index_count,bounding_sphere_radius,pad0}; consumers select between grass/static LODs by distance; lod1_commands_offset in grass globals | status: O/I (struct + function names Inference)

#ifndef TINYGLADE_INC_LOD_HLSL
#define TINYGLADE_INC_LOD_HLSL

// 组合顶点/索引缓冲中单个网格 LOD 的范围。
struct MeshLod {
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

// 选择不超过其距离阈值的最高 LOD 级别。
// thresholds 是 LOD 0 与 LOD 1 生效时的距离平方。
uint select_lod(float distance_sq, float lod0_distance_sq, float lod1_distance_sq)
{
    if (distance_sq < lod0_distance_sq) {
        return 0u;
    }
    if (distance_sq < lod1_distance_sq) {
        return 1u;
    }
    return 2u;
}

// 草地 LOD 选择:全分辨率草叶 vs 单个 lod1 绘制。
uint select_grass_lod(float distance_sq, float lod1_distance_sq)
{
    return (distance_sq < lod1_distance_sq) ? 0u : 1u;
}

#endif // TINYGLADE_INC_LOD_HLSL
