// ======================================================================
// inc/projection.hlsl —— 投影矩阵辅助公共头文件
// ======================================================================
// 用途:透视/正交投影矩阵构造与逆投影工具。
// 关系:frustum.hlsl -> projection.hlsl(21 个消费者 bin,含 nani 剔除与 grass);
//       frustum.hlsl 单独提供平面提取,本文件供给投影矩阵辅助。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/projection.hlsl | evidence: include-graph edge frustum.hlsl -> projection.hlsl (21 consumer bins incl. nani culling, grass); no named functions recoverable (inlined); frustum.hlsl provides plane extraction independently, so this file supplies projection-matrix helpers | status: I/U (names/bodies Inference — inlined; standard projection-matrix helpers)
//
// Projection matrix helpers (perspective matrix construction and
// view-projection utilities).

#ifndef TINYGLADE_INC_PROJECTION_HLSL
#define TINYGLADE_INC_PROJECTION_HLSL

// 标准透视投影矩阵(右手系,+Z 远)。
// fovy_radians 为弧度,aspect = 宽/高,near/far 为正深度。
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

// 正交投影矩阵。
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

// 用逆视图投影矩阵把裁剪空间位置反投影到视图空间。
float3 unproject_to_view(float4x4 clip_to_view, float2 ndc, float depth)
{
    float4 clip = float4(ndc, depth, 1.0f);
    float4 view = mul(clip_to_view, clip);
    return view.xyz / view.w;
}

#endif // TINYGLADE_INC_PROJECTION_HLSL
