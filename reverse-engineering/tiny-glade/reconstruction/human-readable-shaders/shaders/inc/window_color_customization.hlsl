// 窗户颜色定制:从每窗调色板纹理采样颜色,并按实例回退到主题墙色。

#ifndef TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL
#define TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL

// 采样每窗调色板纹理。
float3 sample_window_color(Texture2D window_colors, SamplerState s, uint color_id)
{
    float2 uv = float2((float(color_id) + 0.5) / 16.0, 0.5);
    return window_colors.SampleLevel(s, uv, 0.0).xyz;
}

// 按调色板 id 取窗色;0xFFFFFFFF 表示未指定,回退到主题墙色。
float3 get_window_color(Texture2D window_colors, SamplerState s, uint color_id, float seed)
{
    if (color_id == 0xFFFFFFFFu) {
        return color_library.wall_1.xyz;
    }
    return sample_window_color(window_colors, s, color_id);
}

#endif // TINYGLADE_INC_WINDOW_COLOR_CUSTOMIZATION_HLSL
