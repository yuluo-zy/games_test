// ==RECONSTRUCTED== shaders/clearing/canopy_vert.inc.hlsl
// 【文件用途】树冠顶点辅助函数。内嵌于 tree_canopy 的 bin 中,供
//   tree_canopy.raster.hlsl 使用(按生长缩放放置树冠 + 树冠摆动,
//   摆动把放置记录的 bend_vector 与时间驱动的弯曲结合)。
//   放置记录/变换定义在 clearing/inc/instanced_placement.hlsl 与 canopy_common.inc.hlsl。
// 【状态】O(放置/生长逻辑取自使用点);I(辅助函数名——已内联)。
// 本头文件重写只添加中文注释,代码逐字保留。

#ifndef TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL
#define TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL

// 树冠顶点位置:局部树冠顶点按生长缩放放大,并按风摆动向量
// (放置记录的 bend_vector * 摆动量)在 XZ 平面偏移。
float3 canopy_vertex(float3 local_pos, float growth, float2 bend, float sway)
{
    return local_pos * growth + float3(bend.x * sway, 0.0f, bend.y * sway);
}

#endif // TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL
