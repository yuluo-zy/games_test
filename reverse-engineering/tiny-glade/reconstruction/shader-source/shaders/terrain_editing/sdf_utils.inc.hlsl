// ==RECONSTRUCTED== shaders/terrain_editing/sdf_utils.inc.hlsl | evidence: include-graph (leaf, no includes); consumed by rasterize_terrain_stroke via inc_terrain_stroke; SDF building blocks for stroke rasterization | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL
#define TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL

// Signed distance from a point to a segment (positive outside).
float sd_segment(float2 p, float2 a, float2 b)
{
    float2 pa = p - a;
    float2 ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0f, 1.0f);
    return length(pa - ba * h);
}

// Signed distance to an oriented box (positive outside).
float sd_box(float2 p, float2 center, float2 half_extents)
{
    float2 d = abs(p - center) - half_extents;
    return length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f);
}

// Signed distance to a circle (positive outside).
float sd_circle(float2 p, float2 center, float radius)
{
    return length(p - center) - radius;
}

// Smooth union of two distances.
float sd_smooth_union(float a, float b, float k)
{
    float h = clamp(0.5f + 0.5f * (b - a) / k, 0.0f, 1.0f);
    return lerp(b, a, h) - k * h * (1.0f - h);
}

// Smooth intersection of two distances.
float sd_smooth_intersect(float a, float b, float k)
{
    float h = clamp(0.5f - 0.5f * (b - a) / k, 0.0f, 1.0f);
    return lerp(b, a, h) + k * h * (1.0f - h);
}

#endif // TINYGLADE_TERRAIN_EDITING_SDF_UTILS_HLSL
