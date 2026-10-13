// ==RECONSTRUCTED== shaders/rocky_terrain/triangle_adjacency.inc.hlsl | F-R | include-only (embedded in _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin, _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin)
// 【文件用途】位移岩石地形网格的三角形邻接/裂缝修复辅助函数。
//   每个 Triangle 记录每条边共享该边的邻接三角形索引(Triangle.neighbours[edge];边界为 -1)
//   以及预计算的面法线(Triangle.normal)。相邻的位移瓦片不会精确对齐,因此在栅格化
//   通道中,靠近共享边的片元会把其着色法线向邻接面法线混合;混合带的宽度对"脊顶"
//   (ridge)三角形会收窄,使脊线保持锐利。
// 【证据】裂缝修复数学内联于 rocky_terrain.raster 的 ps_main 中:
//   min-barycentric 边选择、邻接查找、到边线段的距离、两个面法线的 0.5 系数 lerp。
//   函数名是推断的(dxc 已内联,无 OpFunctionCall 留存)。
// 【状态】O(选择/距离算法逐字取自 .cross);I(名称)。
// 本头文件重写只添加中文注释,代码逐字保留。

#ifndef TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL
#define TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL

// 由插值后的重心坐标取离片元最近的三角形边的边号(0..2):
// 即重心权重最小的角点所对的边。返回值同时也是 Triangle.neighbours 中
// 存放该边对侧邻接三角形的槽位。边 i 连接顶点 i 与 (i+1)%3。
uint nearest_edge_index(float3 barycentric)
{
    // 最小权重对应离片元最远的角点,其对边即最近边
    float mn = min(min(barycentric.x, barycentric.y), barycentric.z);
    if (mn == barycentric.z)
        return 0;
    if (mn == barycentric.x)
        return 1;
    return 2;
}

// 片元到钳制线段 (edge_start -> edge_end) 的距离:
// 在线段内部时为 0,远离线段时为正。
float distance_to_edge(float3 fragment_pos, float3 edge_start, float3 edge_end)
{
    float3 e = edge_end - edge_start;
    // 把投影系数钳制到 [0,1],得到线段上最近点
    float3 closest = lerp(edge_start, edge_end,
                          clamp(dot(fragment_pos - edge_start, e) / dot(e, e), 0.0f, 1.0f));
    return distance(fragment_pos, closest);
}

#endif // TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL
