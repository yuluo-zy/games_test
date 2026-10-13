// ==RECONSTRUCTED== shaders/inc/roof_colors_include.hlsl | evidence: consumers _nani_gothic_window_bricks/_nani_instanced_roof/_nani_solid_vertex_color_bricks/_wall_wall_brick_lod* read roof_color_ids + roof_colors (palette) for roof tile shading; color_library.roof_color_1/2 as fallback | status: O (role); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL
#define TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL

// Resolve a roof palette color id to a base albedo (roof_color_1/2 themes).
float3 get_roof_color(uint color_id)
{
    if (color_id == 0u) {
        return color_library.roof_color_1.xyz;
    }
    return color_library.roof_color_2.xyz;
}

// Per-tile hash blend of the two roof theme colors (default when no palette id).
float3 get_roof_default_color(uint tile_hash)
{
    float t = float(tile_hash & 0xFFFFu) / 65535.0;
    return lerp(color_library.roof_color_1.xyz, color_library.roof_color_2.xyz, t);
}

#endif // TINYGLADE_INC_ROOF_COLORS_INCLUDE_HLSL
