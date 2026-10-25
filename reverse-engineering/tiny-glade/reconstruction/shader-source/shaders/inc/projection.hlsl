// ==RECONSTRUCTED== shaders/inc/projection.hlsl | evidence: include-graph edge frustum.hlsl -> projection.hlsl (21 consumer bins incl. nani culling, grass); no named functions recoverable (inlined); frustum.hlsl provides plane extraction independently, so this file supplies projection-matrix helpers | status: I/U (names/bodies Inference — inlined; standard projection-matrix helpers)
//
// Projection matrix helpers (perspective matrix construction and
// view-projection utilities).

#ifndef TINYGLADE_INC_PROJECTION_HLSL
#define TINYGLADE_INC_PROJECTION_HLSL

// Standard perspective projection matrix (right-handed, +Z far).
// fovy_radians in radians, aspect = width/height, near/far positive depths.
float4x4 perspective_projection(float fovy_radians, float aspect, float znear, float zfar)
{
    float y = 1.0f / tan(fovy_radians * 0.5f);
    float x = y / aspect;
    float a = zfar / (zfar - znear);
    float b = -znear * a;
    return float4x4(
        float4(x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, a, b),
        float4(0.0f, 0.0f, 1.0f, 0.0f));
}

// Orthographic projection matrix.
float4x4 orthographic_projection(float left, float right, float bottom, float top, float znear, float zfar)
{
    float2 inv = float2(1.0f / (right - left), 1.0f / (top - bottom));
    float a = 1.0f / (zfar - znear);
    return float4x4(
        float4(2.0f * inv.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, 2.0f * inv.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, a, -znear * a),
        float4(-(right + left) * inv.x, -(top + bottom) * inv.y, 0.0f, 1.0f));
}

// Unproject a clip-space position to view space using the inverse view-projection.
float3 unproject_to_view(float4x4 clip_to_view, float2 ndc, float depth)
{
    float4 clip = float4(ndc, depth, 1.0f);
    float4 view = mul(clip_to_view, clip);
    return view.xyz / view.w;
}

#endif // TINYGLADE_INC_PROJECTION_HLSL
