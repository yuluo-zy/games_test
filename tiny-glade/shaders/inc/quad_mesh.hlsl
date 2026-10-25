// ======================================================================
// inc/quad_mesh.hlsl —— 全屏/单位四边形几何辅助
// ======================================================================
// 用途:由 SV_VertexID 驱动的全屏/单位四边形几何。
// 来源:_billboard_forest raster vs_main 内联的四角映射
//       (float2(vid & 1u, (vid>>1u)&1u),围绕 0.5 缩放)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/quad_mesh.hlsl | evidence: _billboard_forest raster vs_main inlined quad-corner mapping (float2(vid & 1u, (vid>>1u)&1u) then scaled around 0.5); include-graph edges quad_mesh.hlsl -> image.hlsl, math.hlsl -> quad_mesh.hlsl | status: O (corner-from-vertex-id mapping pattern from inlined use-site); I (function names — inlined)
//
// Fullscreen / unit quad geometry helpers driven by SV_VertexID.

#ifndef TINYGLADE_INC_QUAD_MESH_HLSL
#define TINYGLADE_INC_QUAD_MESH_HLSL

// 给定顶点 id(0..3)返回单位四边形的角:(0,0),(1,0),(0,1),(1,1)。
float2 quad_corner(uint vertex_id)
{
    return float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u));
}

// 全屏三角形顶点 id(0..2)对应的裁剪空间位置:把 (0,0),(2,0),(0,2)
// 角映射到覆盖三角形。
float4 fullscreen_triangle_clip(uint vertex_id)
{
    float2 uv = float2(float(vertex_id & 1u), float(vertex_id >> 1u));
    return float4(uv * 2.0f - 1.0f, 0.0f, 1.0f);
}

// 单位四边形裁剪位置(两个三角形,6 个顶点)。
float4 unit_quad_clip(uint vertex_id)
{
    float2 corner = quad_corner(vertex_id);
    return float4(corner * 2.0f - 1.0f, 0.0f, 1.0f);
}

#endif // TINYGLADE_INC_QUAD_MESH_HLSL
