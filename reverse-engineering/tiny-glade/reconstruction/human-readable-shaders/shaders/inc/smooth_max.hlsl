#pragma once
// ======================================================================
// inc/smooth_max.hlsl —— 平滑最大值(IQ smax)
// ======================================================================
// 来源:_terrain_editing_cache_stroke.cs.hlsl_cs_main_0(仅 1 个消费者 bin;
//       内联使用为普通 max(stroke,cache),此处 smooth_max 函数并未被真正调用);
//       标准 Inigo Quilez 平滑最大值。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/smooth_max.hlsl | evidence: _terrain_editing_cache_stroke.cs.hlsl_cs_main_0 (only 1 consumer bin; inlined use is plain max(stroke,cache) -- the smooth_max function itself is not exercised there); standard Inigo Quilez smooth maximum | status: U
//
// The single consumer only shows max(); the smooth_max body below is the standard IQ smax
// (max(a,b) + h^2/(4k), h = max(k-|a-b|,0)).  Function name not embedded -> "smooth_max" Inference.

// IQ 平滑最大值:smax(a,b,k) = max(a,b) + h^2/(4k),h = max(k-|a-b|,0)。
float smooth_max(float a, float b, float k)
{
    float h = max(k - abs(a - b), 0.0f);
    return max(a, b) + (h * h * 0.25f) / k;
}
