// ==RECONSTRUCTED== shaders/inc/sun.inc.hlsl | evidence: raw-per-bin + spirv-cross consumers read frame_constants.sun_color/sky_low_color/sky_high_color and color_library day/dusk pairs for atmospheric lighting; include-graph edge sun.inc.hlsl -> color_library.hlsl | status: O (sources); O/I (helper names/lerp Inference)

#ifndef TINYGLADE_INC_SUN_HLSL
#define TINYGLADE_INC_SUN_HLSL

#include "color_library.hlsl"

// Sun/atmosphere lighting sources used by the forward and deferred passes.
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

// Atmosphere color blended between the day and dusk palette by time-of-day.
float3 get_sky_color(float3 day_low, float3 day_high, float3 dusk_low, float3 dusk_high)
{
    float t = saturate(time_of_day);
    float3 low = lerp(day_low, dusk_low, t);
    float3 high = lerp(day_high, dusk_high, t);
    return low;
}

#endif // TINYGLADE_INC_SUN_HLSL
