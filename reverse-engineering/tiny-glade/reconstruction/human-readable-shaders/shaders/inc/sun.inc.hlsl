// ======================================================================
// inc/sun.inc.hlsl —— 太阳/大气光源访问器公共头文件
// ======================================================================
// 用途:前向与延迟 pass 读取 frame_constants 的太阳/天空颜色与
//       color_library 的昼夜调色板做大气光照。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/sun.inc.hlsl | evidence: raw-per-bin + spirv-cross consumers read frame_constants.sun_color/sky_low_color/sky_high_color and color_library day/dusk pairs for atmospheric lighting; include-graph edge sun.inc.hlsl -> color_library.hlsl | status: O (sources); O/I (helper names/lerp Inference)

#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#include "color_library.hlsl"

// 前向与延迟 pass 使用的太阳/大气光源。
float3 get_sun_color()
{
    return sun_color.xyz;
}

float3 get_sky_low_color()
{
    return sky_low_color.xyz;
}

float3 get_sky_high_color()
{
    return sky_high_color.xyz;
}

float3 get_sun_direction()
{
    return dir_to_sun;
}

// 按时间在昼/昏调色板之间混合的大气颜色。
float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif // TINYGLADE_INC_SUN_HLSL
