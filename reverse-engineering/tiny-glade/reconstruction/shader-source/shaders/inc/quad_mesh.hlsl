// ==RECONSTRUCTED== shaders/inc/quad_mesh.hlsl | evidence: _billboard_forest raster vs_main inlined quad-corner mapping (float2(vid & 1u, (vid>>1u)&1u) then scaled around 0.5); include-graph edges quad_mesh.hlsl -> image.hlsl, math.hlsl -> quad_mesh.hlsl | status: O (corner-from-vertex-id mapping pattern from inlined use-site); I (function names — inlined)
//
// Fullscreen / unit quad geometry helpers driven by SV_VertexID.

#ifndef TINYGLADE_INC_QUAD_MESH_HLSL
#define TINYGLADE_INC_QUAD_MESH_HLSL

// Corner of the unit quad for the given vertex id (0..3): (0,0),(1,0),(0,1),(1,1).
float2 quad_corner(uint vertex_id)
{
    return float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u));
}

// Fullscreen clip-space position for a fullscreen-triangle vertex id (0..2):
// maps (0,0),(2,0),(0,2) corners to a covering triangle.
float4 fullscreen_triangle_clip(uint vertex_id)
{
    float2 uv = float2(float(vertex_id & 1u), float(vertex_id >> 1u));
    return float4(uv * 2.0f - 1.0f, 0.0f, 1.0f);
}

// Unit-quad clip position (two triangles, 6 vertices).
float4 unit_quad_clip(uint vertex_id)
{
    float2 corner = quad_corner(vertex_id);
    return float4(corner * 2.0f - 1.0f, 0.0f, 1.0f);
}

#endif // TINYGLADE_INC_QUAD_MESH_HLSL
