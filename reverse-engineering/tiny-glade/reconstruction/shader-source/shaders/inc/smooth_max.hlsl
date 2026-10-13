#pragma once
// ==RECONSTRUCTED== shaders/inc/smooth_max.hlsl | evidence: _terrain_editing_cache_stroke.cs.hlsl_cs_main_0 (only 1 consumer bin; inlined use is plain max(stroke,cache) -- the smooth_max function itself is not exercised there); standard Inigo Quilez smooth maximum | status: U
//
// The single consumer only shows max(); the smooth_max body below is the standard IQ smax
// (max(a,b) + h^2/(4k), h = max(k-|a-b|,0)).  Function name not embedded -> "smooth_max" Inference.

float smooth_max(float a, float b, float k)
{
    float h = max(k - abs(a - b), 0.0f);
    return max(a, b) + (h * h * 0.25f) / k;
}
