// ============================================================
// terrain_editing/sdf_utils.inc.hlsl
// 功能:笔画光栅化所需的 2D 有符号距离场(SDF)基础函数:
//       点-线段、矩形、圆形的距离,以及平滑并/交运算。
// 消费方:rasterize_terrain_stroke 经由 inc_terrain_stroke.inc.hlsl。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/sdf_utils.inc.hlsl | evidence: include-graph (leaf, no includes); consumed by rasterize_terrain_stroke via inc_terrain_stroke; SDF building blocks for stroke rasterization | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL
#define TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL

// 点到线段的带符号距离(外部为正)。
float sd_segment(float2 p, float2 a, float2 b)
{
    float2 pa = p - a;
    float2 ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0f, 1.0f);
    return length(pa - ba * h);
}

// 到有向矩形的带符号距离(外部为正)。
float sd_box(float2 p, float2 center, float2 half_extents)
{
    float2 d = abs(p - center) - half_extents;
    return length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f);
}

// 到圆形的带符号距离(外部为正)。
float sd_circle(float2 p, float2 center, float radius)
{
    return length(p - center) - radius;
}

// 两个距离的平滑并集(k 为平滑半径)。
float sd_smooth_union(float a, float b, float k)
{
    float h = clamp(0.5f + 0.5f * (b - a) / k, 0.0f, 1.0f);
    return lerp(b, a, h) - k * h * (1.0f - h);
}

// 两个距离的平滑交集(k 为平滑半径)。
float sd_smooth_intersect(float a, float b, float k)
{
    float h = clamp(0.5f - 0.5f * (b - a) / k, 0.0f, 1.0f);
    return lerp(b, a, h) + k * h * (1.0f - h);
}

#endif // TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL
