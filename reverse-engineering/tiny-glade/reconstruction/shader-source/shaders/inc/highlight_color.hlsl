// ==RECONSTRUCTED== shaders/inc/highlight_color.hlsl | evidence: raw-per-bin (_deferred_*/_gi_*/_nani_*_bricks) uses color_library temp_color_1/2/3 + frame_constants.glade_theme_id for editing/highlight colors; exact picker function name inlined | status: O/I (theme->color mapping Inference)

#ifndef TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL
#define TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL

// color_library.hlsl must be included before this file: get_highlight_color
// references the `color_library` cbuffer. A consumer that binds that cbuffer at
// a register other than b14 must guard-skip this include and declare its own
// copy first (adjudicated inc-fix #12 ordering note).
#include "color_library.hlsl"

// Color used to tint highlighted/selected geometry, picked from the active
// theme's palette colors. glade_theme_id indexes temp_color_1/2/3.
float3 get_highlight_color(uint glade_theme_id)
{
    float3 c;
    if (glade_theme_id == 0u) {
        c = color_library.temp_color_1.xyz;
    } else if (glade_theme_id == 1u) {
        c = color_library.temp_color_2.xyz;
    } else {
        c = color_library.temp_color_3.xyz;
    }
    return c;
}

#endif // TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL
