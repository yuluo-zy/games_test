// ==RECONSTRUCTED== shaders/inc/utils.hlsl | evidence: OpLine attribution (spawner consumers _rocky_terrain_stairs_stairs, _garden_marching_squares, _garden_spawn_flowers_*, _water_spawn_*): affine 3x4 compose (transpose basis matrix from forward+up, translation matrix, scale matrix) then column-major 12-float pack into Affine3Packed.data — order verbatim from _rocky_terrain_stairs_stairs decompile (_4453[0].x,_4453[1].x,_4453[2].x, ...); OpSampledImage/OpImageSampleExplicitLod taps for placement height sampling | status: O (Affine3Packed layout + pack order + basis/translation/scale construction from inlined use-sites); I (function names — inlined)
//
// Procedural-instance utilities: affine 3x4 transform helpers and the packed
// transform record used by spawner shaders.

#ifndef TINYGLADE_INC_UTILS_HLSL
#define TINYGLADE_INC_UTILS_HLSL

#include "math_const.hlsl"

// A 3x4 affine transform packed as 12 floats, column-major
// (data[0..2]=col0.xyz, data[3..5]=col1.xyz, data[6..8]=col2.xyz,
//  data[9..11]=translation).
// Guarded so wall/inc/draw_lists.hlsl's identical definition (same layout, verified
// against the same bins) is idempotent regardless of include order (wave-2c adjudication).
#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif

// Rotation-only basis matrix from a forward direction (z-up-agnostic):
// rows [forward; up; cross(forward, up); 0 0 0 1].
float4x4 affine_basis(float3 forward)
{
    float3 f = normalize(forward);
    float3 up = float3(0.0f, 1.0f, 0.0f);
    float3 right = cross(f, up);
    float4x4 m = float4x4(
        float4(f, 0.0f),
        float4(up, 0.0f),
        float4(right, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return transpose(m);
}

// Pure translation affine matrix.
float4x4 affine_translation(float3 pos)
{
    return float4x4(
        float4(1.0f, 0.0f, 0.0f, pos.x),
        float4(0.0f, 1.0f, 0.0f, pos.y),
        float4(0.0f, 0.0f, 1.0f, pos.z),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}

// Pure (possibly non-uniform) scale affine matrix.
float4x4 affine_scale(float3 s)
{
    return float4x4(
        float4(s.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, s.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, s.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
}

// Pack a column-major 4x4 (rotation cols 0..2 + translation col 3) into the
// 12-float Affine3Packed record, dropping the affine row.
Affine3Packed pack_affine(float4x4 m)
{
    Affine3Packed o;
    o.data[0] = m[0].x; o.data[1] = m[1].x; o.data[2] = m[2].x;
    o.data[3] = m[0].y; o.data[4] = m[1].y; o.data[5] = m[2].y;
    o.data[6] = m[0].z; o.data[7] = m[1].z; o.data[8] = m[2].z;
    o.data[9] = m[0].w; o.data[10] = m[1].w; o.data[11] = m[2].w;
    return o;
}

// Rebuild the column-major 4x4 (affine row restored).
float4x4 unpack_affine(Affine3Packed a)
{
    float4x4 m;
    m[0] = float4(a.data[0], a.data[3], a.data[6], a.data[9]);
    m[1] = float4(a.data[1], a.data[4], a.data[7], a.data[10]);
    m[2] = float4(a.data[2], a.data[5], a.data[8], a.data[11]);
    m[3] = float4(0.0f, 0.0f, 0.0f, 1.0f);
    return m;
}

// Compose translation * rotation * scale and pack.
Affine3Packed compose_affine_packed(float3 pos, float3 forward, float3 scale)
{
    return pack_affine(mul(affine_translation(pos), mul(affine_basis(forward), affine_scale(scale))));
}

#endif // TINYGLADE_INC_UTILS_HLSL
