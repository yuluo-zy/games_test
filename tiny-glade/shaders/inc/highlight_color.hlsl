// ============================================================
// inc/highlight_color.hlsl —— 高亮/编辑颜色选择
// 用途:用 color_library 的 temp_color_1/2/3 + glade_theme_id 选取
//   高亮/编辑几何使用的颜色。
//   消费方:_deferred_*、_gi_*、_nani_*_bricks。
// 入口:无(纯头文件;依赖 color_library.hlsl)
// 顺序说明:color_library.hlsl 必须先于本文件 include;把该 cbuffer 绑在
//   非 b14 寄存器的消费方需 guard-skip 本 include 并先自行声明拷贝
//   (裁定 inc-fix #12)。
// ============================================================
// ==RECONSTRUCTED== shaders/inc/highlight_color.hlsl | evidence: raw-per-bin (_deferred_*/_gi_*/_nani_*_bricks) uses color_library temp_color_1/2/3 + frame_constants.glade_theme_id for editing/highlight colors; exact picker function name inlined | status: O/I (theme->color mapping Inference)

#ifndef TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL
#define TINYGLADE_INC_HIGHLIGHT_COLOR_HLSL

// color_library.hlsl must be included before this file: get_highlight_color
// references the `color_library` cbuffer. A consumer that binds that cbuffer at
// a register other than b14 must guard-skip this include and declare its own
// copy first (adjudicated inc-fix #12 ordering note).
#include "color_library.hlsl"

// 高亮/选中几何使用的颜色,取自当前主题调色板。
// glade_theme_id 索引 temp_color_1/2/3。
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
