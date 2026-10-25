// ==RECONSTRUCTED== shaders/inc/color_library.hlsl | evidence: raw-per-bin OpMemberName ColorLibrary 37 float4 (temp_color_1..night_tint) exactly as listed; cbuffer variable `color_library` bound at per-consumer set0 bindings (13/14/18/21/23/24 observed) so the cbuffer is declared here at a placeholder register | status: O (struct + members); I (cbuffer register placeholder)

#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL

// The theme color library. Member order matches the embedded OpMemberName
// table verbatim.
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

// Consumers bind this cbuffer at their own set0 slot; b14 is the most common
// observed binding (terrain_lighting_texture family). The cbuffer NAME matches
// the original OpName (`color_library`, adjudicated inc-fix #10); consumers
// reference the member variable `color_library` (e.g. color_library.temp_color_1).
// 恢复适配：各消费者可指定原发行已证实的槽位，默认保持 b14。
#ifndef TINYGLADE_COLOR_LIBRARY_REGISTER
#define TINYGLADE_COLOR_LIBRARY_REGISTER b14
#endif
cbuffer color_library : register(TINYGLADE_COLOR_LIBRARY_REGISTER, space0) {
    ColorLibrary color_library;
};

#endif // TINYGLADE_INC_COLOR_LIBRARY_HLSL
