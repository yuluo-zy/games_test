// ==RECONSTRUCTED== shaders/garden/placement_utils.inc.hlsl | evidence: inlined use-sites in _garden_spawn_* + _water_spawn_* (world grid uv: ((float2(id.xy)+0.5)/grid_dims-0.5)*130; placement heightmap uv: world*0.007692307699+0.5; yaw rotation = transpose(float4x4(rows)) Rodrigues around normalize(0,1,0); compose = mul(mul(translate, mul(identity, scale)), rot); Flower03Xform{float4 position} for _garden_spawn_small_flowers_garden) | status: I (function/struct names Inference — inlined, not embedded); O (algorithms/constants from inlined SPIR-V)

#ifndef TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL
#define TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL

// Affine3Packed / rand01 / hash / terrain_heightmap helpers come from the
// consumer's prior includes (utils.hlsl, hash.hlsl, terrain_heightmap.inc.hlsl).
// This file adds the family-shared placement record + helpers.

struct Flower03Xform
{
    float4 position;
};

// Grid dispatch id -> centered world xz (terrain extent 130, world center 0).
float2 grid_world_pos(float2 id, int2 grid_dims)
{
    return ((float2(id) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;
}

// World xz -> placement-mask heightmap uv.
float2 placement_uv(float2 world_xz)
{
    return world_xz * 0.0076923076994717121124267578125f + 0.5f;
}

// Rodrigues rotation around an arbitrary axis, built as the transposed
// row-major 4x4 the placement passes embed (matches the inlined math).
float4x4 rotate_axis(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    float omc = 1.0f - c;
    float x = axis.x;
    float y = axis.y;
    float z = axis.z;
    return transpose(float4x4(
        float4(omc * x * x + c, omc * x * y - s * z, omc * x * z + s * y, 0.0f),
        float4(omc * x * y + s * z, omc * y * y + c, omc * y * z - s * x, 0.0f),
        float4(omc * x * z - s * y, omc * y * z + s * x, omc * z * z + c, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
}

// Yaw rotation around the vertical axis (the placement passes' common case).
float4x4 rotate_y(float angle)
{
    return rotate_axis(float3(0.0f, 1.0f, 0.0f), angle);
}

// translate * (identity * scale) * rotation — the exact compose order the
// placement passes emit (identity is left as an explicit factor, as compiled).
float4x4 placement_transform(float3 pos, float3 scale, float yaw)
{
    float4x4 identity = float4x4(
        float4(1.0f, 0.0f, 0.0f, 0.0f),
        float4(0.0f, 1.0f, 0.0f, 0.0f),
        float4(0.0f, 0.0f, 1.0f, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 s = float4x4(
        float4(scale.x, 0.0f, 0.0f, 0.0f),
        float4(0.0f, scale.y, 0.0f, 0.0f),
        float4(0.0f, 0.0f, scale.z, 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f));
    return mul(mul(affine_translation(pos), mul(identity, s)), rotate_y(yaw));
}

// Place a Flower03Xform record (float4 position) in a byte/structured buffer.
void store_flower03(RWByteAddressBuffer buffer, uint base, float4 position)
{
    buffer.Store<float4>(base * 16u + 0u, position);
}

#endif // TINYGLADE_GARDEN_PLACEMENT_UTILS_HLSL

// Garden-mask exclusion test shared by the spawn passes. `uv` is the placement
// heightmap uv; px = uv * path_mask_dims. Returns true when the cell is
// excluded (path / rocky / steep slope / water / foliage exclusion).
bool placement_excluded(
    Texture2D<float4> terrain_heightmap,
    uint2 terrain_heightmap_dims,
    Texture2D<float4> path_mask,
    uint2 path_mask_dims,
    Texture2D<float4> rocky_terrain,
    Texture2D<float4> water_mask,
    Texture2D<float4> foliage_exclusion_mask,
    SamplerState sampler_llb,
    SamplerState sampler_default,
    float2 uv)
{
    uint2 px = uint2(uv * float2(path_mask_dims));
    if (path_mask.Load(int3(px, 0u)).x > 0.2f)
        return true;

    float4 g = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5f) * 26.5f;
    grad /= (130.0f / max(float2(terrain_heightmap_dims), 1.0e-6f));
    if (rocky_terrain.Load(int3(px, 0u)).x > 0.0f || length(grad) > 0.9f)
        return true;

    if (water_mask.Load(int3(px, 0u)).y > 0.3f)
        return true;

    if (foliage_exclusion_mask.SampleLevel(sampler_llb, uv, 0.0f).x > 0.0f)
        return true;

    return false;
}

// Banded falloff (1 at d==0, 0 at d >= 1/inv_half_width), the shared placement
// scoring curve the garden/water spawners embed (1 - t^2*(3 - d*2*inv)).
float band_falloff_garden(float d, float inv_half_width)
{
    float t = d * inv_half_width;
    return 1.0f - (t * t) * (3.0f - d * (2.0f * inv_half_width));
}
