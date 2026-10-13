// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/isolines.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// 【文件用途】楼梯生成:在等高线网格单元上放置每个楼梯台阶对应的"等高层"台阶。
//   每个等高线网格单元的等高高度窗口:level_min = min(ceil((minCornerHeight-0.9)*1.847826...),16)
//   到 level_max = max(floor((maxCornerHeight-0.7)*1.847826...),0),每层的目标高度为
//   lerp(0.8,10,level/16)。对每一层,由 4 位角点状态掩码(8>>corner,当 cornerHeight >= h 时置位)
//   选择 marching-squares 情形;掩码 0/15(单元完全在该层之下/之上)跳过。名称"isolines"表示
//   每层都是穿过该单元的地形等高线。
// 【状态】O(层窗口 + 掩码循环逐字取自内联后的 main);I(函数名——已内联)
// 本头文件重写只添加中文注释,代码逐字保留。

#pragma once
#ifndef TINYGLADE_STAIRS_ISOLINES_INC_HLSL
#define TINYGLADE_STAIRS_ISOLINES_INC_HLSL

#include "marching_squares.inc.hlsl"

// 在穿越该等高线网格单元的地形等高层上放置楼梯台阶:
// 对该单元高度窗口内的每一楼梯等高层,按角点高度生成掩码并分发到 marching-squares 情形。
void place_stair_isolines(float2 corners[4], float4 corner_heights)
{
    // 单元四个角点高度的最小值(决定最低需要覆盖的等高层)
    float min_h = min(min(corner_heights.x, corner_heights.y), min(corner_heights.z, corner_heights.w));
    // 单元四个角点高度的最大值(决定最高需要覆盖的等高层)
    float max_h = max(max(corner_heights.x, corner_heights.y), max(corner_heights.z, corner_heights.w));

    // 等高层号窗口:由角点高度线性映射到 0..16 的层号,并钳制在 [0,16]。
    int level_min = min(int(ceil((min_h - 0.9f) * 1.84782612323760986328125f)), 16);
    int level_max = max(int(floor((max_h - 0.7f) * 1.84782612323760986328125f)), 0);

    // 对窗口内每一等高层逐层处理
    for (int level = level_min; level <= level_max; level++)
    {
        // 该层的目标等高高度:在 0.8..10 之间按层号线性插值(level*0.0625 == level/16)
        float h = lerp(0.8f, 10.0f, float(level) * 0.0625f);

        // 生成 4 位角点状态掩码:第 i 位在 corner_heights[i] >= h 时置位(编码为 8>>i)
        uint mask = 0u;
        for (int i = 0; i < 4; i++)
        {
            if (corner_heights[i] >= h)
                mask |= (8u >> uint(i));
        }

        // 掩码 0(单元整体低于该层)或 15(整体高于该层)时无等高线穿过,跳过
        if (mask == 0u || mask == 15u)
            continue;

        // 其余情况:把掩码交给 marching-squares 分发,在该层高度 h 上放置台阶
        place_stair_case(mask, corners, h);
    }
}

#endif // TINYGLADE_STAIRS_ISOLINES_INC_HLSL
