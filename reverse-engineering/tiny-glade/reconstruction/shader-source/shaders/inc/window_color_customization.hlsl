// ==RECONSTRUCTED== shaders/inc/window_color_customization.hlsl | evidence: consumers _nani_balcony_door/_nani_solid_vertex_color_window/_solid_vertex_color_wooden_gate sample window_colors + gate_color_ids; frame_constants + color_library available for palette tinting; include-graph edge hash.hlsl -> window_color_customization.hlsl | status: O (role); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL

// Sample the per-window color palette texture.
float3 sample_window_color(Texture2D window_colors, SamplerState s, uint color_id)
{
    float2 uv = float2((float(color_id) + 0.5) / 16.0, 0.5);
    return window_colors.SampleLevel(s, uv, 0.0).xyz;
}

// Pick a window color by palette id with a per-instance fallback to the theme.
float3 get_window_color(Texture2D window_colors, SamplerState s, uint color_id, float seed)
{
    if (color_id == 0xFFFFFFFFu) {
        return color_library.wall_1.xyz;
    }
    return sample_window_color(window_colors, s, color_id);
}

#endif // TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL
