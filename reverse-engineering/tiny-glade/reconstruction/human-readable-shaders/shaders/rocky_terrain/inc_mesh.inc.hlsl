// ==RECONSTRUCTED== shaders/rocky_terrain/inc_mesh.inc.hlsl | F-R | include-only (embedded in _rocky_terrain_displace_rocky_terrain.cs.hlsl_cs_main_0.bin, _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin, _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin)
// 【文件用途】岩石地形共享网格记录(顶点/三角形)布局。
//   位移后的岩石地形网格由 displace_rocky_terrain.cs(每个三角形一个线程)生成,
//   再被 rocky_terrain.raster 通道(及楼梯通道)通过两个 SSBO 消费:
//     vertices  - 每顶点 48 字节记录(偏移:rest_position 0, cell_bby 12,
//                 position 16, cell_id 28, dir_to_centroid 32, is_corner 44)
//     triangles - 每三角形 32 字节记录(偏移:neighbours 0, is_top 12,
//                 normal 16, pad1 28)
// 【证据】spirv-cross + 原始 SPIR-V 成员偏移(OpMemberDecorate Offset),取自
//   _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin 的 VS/PS 与
//   displace_rocky_terrain CS:VS 在字节 16 处读取 position(因此 cell_bby 是
//   位于 12 的单个 float,而非 float3);CS 在 28 写 cell_id / 44 写 is_corner;
//   triangles 步长 32:neighbours int3 @0, is_top int @12, normal float3 @16,
//   pad1 float @28。
// 【状态】O(成员顺序/类型/偏移逐字取自 .cross + SPV 装饰)。
// 本头文件为纯声明,重写只添加中文注释,不改变任何代码。

#ifndef TINYGLADE_ROCKY_TERRAIN_INC_MESH_HLSL
#define TINYGLADE_ROCKY_TERRAIN_INC_MESH_HLSL

// 位移后岩石地形网格的一个顶点,共 48 字节。
struct Vertex
{
    float3 rest_position;      // 未位移的静止位置(网格生成前的原始位置)
    float cell_bby;            // 单一浮点,顶点在栅格中的"底部/包围盒 Y"辅助值(见文件头证据)
    float3 position;           // 位移后的最终世界/网格位置
    int cell_id;               // 所属栅格单元 id(用于相邻关系/分裂修复)
    float3 dir_to_centroid;    // 指向单元质心的方向(用于位移方向计算)
    int is_corner;             // 是否为网格单元角点
};

// 位移网格的一个三角形。neighbours[i] 是与第 i 条边共享的邻接三角形索引
// (即角点 i 对边),边界上为 -1。共 32 字节。
struct Triangle
{
    int3 neighbours;    // 三条边各自的邻接三角形索引,边界为 -1
    int is_top;         // 是否"脊顶"三角形(用于裂缝修复时收窄混合带,保持脊线锐利)
    float3 normal;      // 预计算的面法线
    float pad1;         // 填充
};

#endif // TINYGLADE_ROCKY_TERRAIN_INC_MESH_HLSL
