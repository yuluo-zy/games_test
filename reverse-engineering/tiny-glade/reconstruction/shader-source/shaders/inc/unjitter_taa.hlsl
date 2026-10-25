// ==RECONSTRUCTED== shaders/inc/unjitter_taa.hlsl | evidence: consumers _taa/_tgsr read reprojected history using the current frame's sample_offset_pixels (view_constants) to remove TAA jitter; include-graph edge math.hlsl -> unjitter_taa.hlsl | status: O/I (function names/body Inference, inlined)

#ifndef TINYGLADE_INC_UNJITTER_TAA_HLSL
#define TINYGLADE_INC_UNJITTER_TAA_HLSL

// Remove the current frame's TAA sub-pixel jitter from a pixel coordinate.
float2 unjitter_taa_pixel(float2 jittered_pixel, float2 current_offset_pixels)
{
    return jittered_pixel - current_offset_pixels;
}

// Remove TAA jitter from a normalized UV coordinate.
float2 unjitter_taa_uv(float2 jittered_uv, float2 current_offset_pixels, float2 render_target_dims)
{
    return jittered_uv - (current_offset_pixels / render_target_dims);
}

// Previous-frame UV for history fetch: current unjittered UV plus the previous
// frame's motion (via the reprojection map, applied by the caller).
float2 taa_history_uv(float2 unjittered_uv, float2 reprojection_offset)
{
    return unjittered_uv + reprojection_offset;
}

#endif // TINYGLADE_INC_UNJITTER_TAA_HLSL
