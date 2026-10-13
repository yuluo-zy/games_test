// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/marching_squares.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// 【文件用途】楼梯生成:等高线网格单元的 marching-squares 情形分发。
//   4 位角点状态掩码(第 i 位在角点 i 达到/超过目标等高高度时置位,编码为 8>>i,
//   因此掩码 1 == 角点 3、掩码 8 == 角点 0)决定等离线穿过的单元边。
//   单角点掩码(1..4)及其补掩码(14..11)使用奇角点相邻的两条边;
//   两相邻角点掩码(3,6 及其补 12,9)使用高角点与低角点之间的两条边;
//   鞍形掩码(5,10)在两条不相连的线段上各放一个台阶。
//   边配对逐字取自 bin 中内联后的 switch/if-else 阶梯(每个 case = 一或两次 place_stairs 调用)。
// 【状态】O(case 表逐字取自内联分发);I(函数名——已内联)
// 本头文件重写只添加中文注释,代码逐字保留。

#pragma once
#ifndef TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL
#define TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL

#include "place_stairs.inc.hlsl"

// 按等高线网格单元的角点状态掩码分发 marching-squares 情形:选出该掩码
// 跨越的单元边,并在每条等离线线段上、高度 h 处放置一个楼梯台阶。
void place_stair_case(uint mask, float2 corners[4], float h)
{
    switch (mask)
    {
    case 1:  case 14:
        // 单角点 0 / 补情形:等离线连接角点0-3 与角点3-2 两侧边
        place_stairs(corners[0], corners[3], corners[3], corners[2], h);
        break;
    case 2:  case 13:
        // 单角点 1 / 补情形:等离线连接角点1-2 与角点3-2 两侧边
        place_stairs(corners[1], corners[2], corners[3], corners[2], h);
        break;
    case 3:  case 12:
        // 两相邻角点 0,1 / 补情形:等离线穿过边 0-3 与边 1-2
        place_stairs(corners[0], corners[3], corners[1], corners[2], h);
        break;
    case 4:  case 11:
        // 单角点 3 / 补情形:等离线连接角点0-1 与角点1-2 两侧边
        place_stairs(corners[0], corners[1], corners[1], corners[2], h);
        break;
    case 5:
        // 鞍形:两条不相连的等离线线段,各放一个台阶
        place_stairs(corners[0], corners[1], corners[0], corners[3], h);
        place_stairs(corners[1], corners[2], corners[3], corners[2], h);
        break;
    case 6:  case 9:
        // 两对角角点 0,2 / 补情形:等离线连接边 0-1 与边 3-2
        place_stairs(corners[0], corners[1], corners[3], corners[2], h);
        break;
    case 7:  case 8:
        // 三角点情形 / 补:等离线连接边 0-1 与边 0-3
        place_stairs(corners[0], corners[1], corners[0], corners[3], h);
        break;
    case 10:
        // 另一鞍形:两条不相连的等离线线段,各放一个台阶
        place_stairs(corners[0], corners[1], corners[1], corners[2], h);
        place_stairs(corners[0], corners[3], corners[3], corners[2], h);
        break;
    }
}

#endif // TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL
