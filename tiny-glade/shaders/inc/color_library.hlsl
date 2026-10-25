// ============================================================
// inc/color_library.hlsl —— 主题调色板(cbuffer color_library)
// 用途:定义 37 个 float4 的调色板结构与默认绑定槽位。
//   消费方:几乎所有需要主题色的着色器。
// 关键绑定:set0,默认 b14;消费方可用 TINYGLADE_COLOR_LIBRARY_REGISTER 覆盖
//   (实测 binding 有 13/14/18/21/23/24)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/color_library.hlsl | evidence: raw-per-bin OpMemberName ColorLibrary 37 float4 (temp_color_1..night_tint) exactly as listed; cbuffer variable `color_library` bound at per-consumer set0 bindings (13/14/18/21/23/24 observed) so the cbuffer is declared here at a placeholder register | status: O (struct + members); I (cbuffer register placeholder)

#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL

// 主题调色板。成员顺序与嵌入的 OpMemberName 表逐项一致。
struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

// 消费方在各自的 set0 槽位绑定此 cbuffer;b14 是最常见的实测绑定
// (terrain_lighting_texture 族)。cbuffer 名与原版 OpName 一致
// (`color_library`,裁定 inc-fix #10);消费方以 `color_library` 成员变量引用
// (如 color_library.temp_color_1)。
// 恢复适配:各消费者可指定原发行已证实的槽位,默认保持 b14。
#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif // TINYGLADE_INC_COLOR_LIBRARY_HLSL
