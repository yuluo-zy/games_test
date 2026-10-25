// ==RECONSTRUCTED== shaders/roof/dormer_hole.inc.hlsl | F-V | embedded in nani_instanced_roof.raster (2) + nani_roof_floor.raster (2) + nani_stairs_floor.raster (2)
// Dormer-hole record + occlusion helper. DormerHole layout/names recovered verbatim from the
// consumer OpMemberName tables (position f3, wall_id u, size f3, pad0 f2, normal f2, k, a;
// 48-byte record). The occlusion math is reconstructed from its inlined use in
// _nani_roof_floor ps_main: fragments inside the oriented dormer box (scaled by size,
// eased by k/a) are discarded.

#ifndef TINYGLADE_ROOF_DORMER_HOLE_HLSL
#define TINYGLADE_ROOF_DORMER_HOLE_HLSL

// One dormer hole record (48 bytes, stored in the `dormer_holes` byte-address buffer).
struct DormerHole
{
    float3 position;
    uint wall_id;
    float3 size;
    float2 pad0;
    float2 normal;   // basis0_xz
    float k;
    float a;
};

// Test whether a world-space fragment lies inside the dormer hole. Reconstructed from the
// inlined use-site: the fragment is projected into the dormer frame (basis from `normal`),
// scaled by size, and accepted if it falls within the rounded dormer profile.
bool is_inside_dormer_hole(float3 world_pos, DormerHole d)
{
    float3 basis = float3(d.normal.x, 0.0f, d.normal.y);
    float3x3 basis_mat = transpose(float3x3(cross(basis, float3(0.0f, 1.0f, 0.0f)),
                                            float3(0.0f, 1.0f, 0.0f), basis));
    float3 b0 = mul(basis_mat, float3(1.0f, 0.0f, 0.0f));
    float3 b1 = mul(basis_mat, float3(0.0f, 1.0f, 0.0f));
    float3 b2 = mul(basis_mat, float3(0.0f, 0.0f, 1.0f));
    float3 n = cross(b0, b1);
    float3 inv = (1.0f / dot(b2, n)).xxx;
    float3 p = mul(world_pos - d.position, float3x3(cross(b1, b2) * inv, cross(b2, b0) * inv, n * inv)) / d.size;
    float w = clamp((p.y * d.k) + d.a, 0.0f, 1.0f);
    bool inside = abs(p.x) < lerp(0.5f, 0.0f, (w * w) * w)
               && ((d.k == 1.0f) ? (p.y < 0.5f) && (p.y > -0.5f) : p.y < 0.7f)
               && abs(p.z) < 0.5f;
    return inside;
}

#endif // TINYGLADE_ROOF_DORMER_HOLE_HLSL
