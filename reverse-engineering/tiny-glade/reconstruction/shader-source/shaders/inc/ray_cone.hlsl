// ==RECONSTRUCTED== shaders/inc/ray_cone.hlsl | status: Inference (minimal
// behavior-consistent body, filled in P1c integration pass, adjudicated inc-fix #13).
// Used by water / GI reflection-cone and reprojection consumers (307 bins embed it);
// the exact helper names are not recoverable (inlined away in SPIR-V), so this is a
// self-consistent minimal ray-cone LOD utility. Wave-1 consumers only embed the file
// for OpSource parity; they do not call these helpers.

#ifndef TINYGLADE_INC_RAY_CONE_HLSL
#define TINYGLADE_INC_RAY_CONE_HLSL

// Ray with a growing cone aperture, used to select reflection / irradiance mip
// levels by the cone's footprint at distance t.
struct RayCone
{
    float3 origin;
    float3 direction;
    float spread; // tan(half aperture)
};

// Half-width of the cone footprint at distance t.
float ray_cone_half_width(RayCone cone, float t)
{
    return t * cone.spread;
}

// Mip level for a cone footprint covering `footprint_texels` texels: log2 of the
// largest axis. `footprint_world` and `world_per_texel` are float2 extents.
float ray_cone_mip(float2 footprint_world, float2 world_per_texel)
{
    float2 footprint_texels = max(footprint_world / max(world_per_texel, 1e-6f.xx), 1.0f.xx);
    return log2(max(footprint_texels.x, footprint_texels.y));
}

#endif // TINYGLADE_INC_RAY_CONE_HLSL
