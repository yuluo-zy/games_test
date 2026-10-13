// 墙体调色板颜色 id 查询:按砖块/三角形索引读取 ByteAddressBuffer 中的调色板 id。

#ifndef TINYGLADE_INC_WALL_COLOR_IDS_HLSL
#define TINYGLADE_INC_WALL_COLOR_IDS_HLSL

#include "reservoir.hlsl"

// 读取砖块(或三角形)索引对应的墙体调色板颜色 id。
uint get_wall_color_id(ByteAddressBuffer wall_color_ids, uint index)
{
    return wall_color_ids.Load<uint>(index * 4u + 0u);
}

#endif // TINYGLADE_INC_WALL_COLOR_IDS_HLSL
