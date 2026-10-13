// ==RECONSTRUCTED== shaders/inc/uv.hlsl | evidence: _apply_screen_shadows cs inlined (pixel->uv: (int2+0.5)/dims; uv->sample: -0.5; sample->view->world: mul(view_to_world, mul(sample_to_view, float4(sample*float2(2,-2), depth, 1)))); ViewConstants sample/view/clip matrix set; include-graph edge frame_constants.hlsl -> uv.hlsl | status: O (coordinate-space transform math from inlined use-sites); I (function names/signatures — inlined; matrices passed as params because uv.hlsl is included before frame_constants.hlsl declares the cbuffer)
//
// Coordinate-space helpers shared by frame_constants consumers. Matrix-based
// (no cbuffer dependency) so it compiles before frame_constants.hlsl declares
// the view matrix set.

#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL

// Integer pixel coordinate -> normalized UV ([0,1], texel centers at .5).
float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}

// Normalized UV -> centered "sample" coordinate (origin at pixel centers,
// y up). Sample space is what the view_to_sample / sample_to_view matrices
// operate on.
float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}

// Sample-space x,y (already centered) -> clip-space x,y (-1..1, y up).
float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}

// Reconstruct a view-space position from a sample coordinate and linear depth.
// sample_uv is a centered coordinate; depth is the view-space -z distance.
float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}

// View-space position -> centered sample coordinate.
float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}

// View-space position -> clip space.
float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}

// World-space position -> view space.
float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}

// World-space position -> centered sample coordinate.
float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif // TINYGLADE_INC_UV_HLSL
