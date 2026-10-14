// 墙体颜色定制:把墙体调色板颜色 id 解析为基色反照率,或按砖块哈希在
// 主题墙色间混合。id 映射:0/1 = wall_1/wall_2,2 = wall_bricks,3 = wall_moss。

#ifndef TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL

// 把墙体调色板颜色 id 解析为基础反照率。
float3 get_wall_custom_color(uint color_id)
{
    if (color_id == 0u) {
        return color_library.wall_1.xyz;
    } else if (color_id == 1u) {
        return color_library.wall_2.xyz;
    } else if (color_id == 2u) {
        return color_library.wall_bricks.xyz;
    }
    return color_library.wall_moss.xyz;
}

// 按砖块哈希在两种主题墙色间混合,用于没有显式定制颜色 id 的情况。
float3 get_wall_default_color(uint brick_hash)
{
    float t = float(brick_hash & 0xFFFFu) / 65535.0;
    return lerp(color_library.wall_1.xyz, color_library.wall_2.xyz, t);
}

#endif // TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL
