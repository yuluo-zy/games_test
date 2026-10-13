// ==RECONSTRUCTED== shaders/rocky_terrain/triangle_adjacency.inc.hlsl | F-R | include-only (embedded in _rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915.bin, _rocky_terrain_rocky_terrain.raster.hlsl_f0adff76827e73ff.bin)
// Per-triangle adjacency for crack fixing on the displaced rocky-terrain mesh.
// Each Triangle records, per edge, the index of the neighbouring triangle
// sharing that edge (Triangle.neighbours[edge]; -1 on a boundary) and the
// precomputed face normal (Triangle.normal). Adjacent displaced tiles do not
// line up exactly, so in the raster pass a fragment near a shared edge blends
// its shading normal toward the neighbour's face normal; the width of the
// blend band is narrowed for "top" (ridge) triangles so the ridge stays sharp.
// evidence: the crack-fix math is inlined in the rocky_terrain.raster ps_main
//   (workspace/f-r-scratch/modules/_rocky_terrain_rocky_terrain.raster.hlsl_b903e43ffb3da915_1.hlsl.cross):
//   min-barycentric edge selection, neighbour lookup, distance to the edge
//   segment, and the 0.5-factor lerp of the two face normals. Function names
//   are Inference (dxc inlined them; no OpFunctionCall survives).
// status: O (selection/distance algorithm verbatim from .cross); I (names).

#ifndef TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL
#define TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL

// Edge index (0..2) of the triangle edge nearest to a fragment, from its
// interpolated barycentric weights: the edge opposite the corner with the
// smallest weight. The returned index is also the slot in Triangle.neighbours
// holding the neighbour across that edge. Edge i joins vertices i and (i+1)%3.
uint nearest_edge_index(float3 barycentric)
{
    float mn = min(min(barycentric.x, barycentric.y), barycentric.z);
    if (mn == barycentric.z)
        return 0;
    if (mn == barycentric.x)
        return 1;
    return 2;
}

// Distance from a fragment to the clamped segment (edge_start -> edge_end), i.e.
// 0 on the edge interior and positive away from it.
float distance_to_edge(float3 fragment_pos, float3 edge_start, float3 edge_end)
{
    float3 e = edge_end - edge_start;
    float3 closest = lerp(edge_start, edge_end,
                          clamp(dot(fragment_pos - edge_start, e) / dot(e, e), 0.0f, 1.0f));
    return distance(fragment_pos, closest);
}

#endif // TINYGLADE_ROCKY_TERRAIN_TRIANGLE_ADJACENCY_HLSL
