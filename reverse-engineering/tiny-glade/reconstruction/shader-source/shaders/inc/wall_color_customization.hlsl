// ==RECONSTRUCTED== shaders/inc/wall_color_customization.hlsl | evidence: consumers _nani_gothic_window_bricks/_nani_plaster/_nani_roof_floor/_nani_solid_vertex_color_bricks read a wall palette color id (from wall_color_ids / instance override_color_id) and resolve it against color_library.wall_1/wall_2/wall_moss/wall_bricks; exact mapping inlined | status: O/I (function names/mapping Inference)

#ifndef TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL

// Resolve a wall palette color id to a base albedo. Palette ids map to the
// theme's wall colors: 0/1 = wall_1/wall_2, 2 = wall_bricks, 3 = wall_moss.
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

// Blend between the two theme wall colors by a per-brick hash, used when no
// explicit customization color id is present.
float3 get_wall_default_color(uint brick_hash)
{
    float t = float(brick_hash & 0xFFFFu) / 65535.0;
    return lerp(color_library.wall_1.xyz, color_library.wall_2.xyz, t);
}

#endif // TINYGLADE_INC_WALL_COLOR_CUSTOMIZATION_HLSL
