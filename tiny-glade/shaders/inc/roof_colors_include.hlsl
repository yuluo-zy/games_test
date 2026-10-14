// ======================================================================
// inc/roof_colors_include.hlsl —— 屋顶调色板颜色解析
// ======================================================================
// 用途:被 _nani_gothic_window_bricks/_nani_instanced_roof/
//       _nani_solid_vertex_color_bricks/_wall_wall_brick_lod* 读取
//       roof_color_ids + roof_colors(调色板)做屋顶瓦片着色;
//       color_library.roof_color_1/2 作为回退。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/roof_colors_include.hlsl | evidence: consumers _nani_gothic_window_bricks/_nani_instanced_roof/_nani_solid_vertex_color_bricks/_wall_wall_brick_lod* read roof_color_ids + roof_colors (palette) for roof tile shading; color_library.roof_color_1/2 as fallback | status: O (role); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL
#define TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL

// 把屋顶调色板颜色 id 解析为基准 albedo(roof_color_1/2 主题)。
float3 get_roof_color(uint color_id)
{
    if (color_id == 0u) {
        return color_library.roof_color_1.xyz;
    }
    return color_library.roof_color_2.xyz;
}

// 两个屋顶主题颜色的逐瓦片哈希混合(无调色板 id 时默认)。
float3 get_roof_default_color(uint tile_hash)
{
    float t = float(tile_hash & 0xFFFFu) / 65535.0;
    return lerp(color_library.roof_color_1.xyz, color_library.roof_color_2.xyz, t);
}

#endif // TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL
