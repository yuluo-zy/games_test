// ==RECONSTRUCTED== shaders/buildings/curved_halftimber.inc.hlsl | evidence: include-graph direct edge -> inc/material/solid_material.inc.hlsl; consumed by nani_curved_halftimber raster + nani cull/bucket (wave-2 family); curved half-timber wall material | status: O (include edge, material role); I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL
#define TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL

#include "inc/material/solid_material.inc.hlsl"

// Curved half-timber parameters: timber frame color and fill (plaster) color.
struct CurvedHalftimberParams
{
    float3 timber_color;
    float3 fill_color;
    float timber_thickness;
    float2 uv_scale;
    uint flags;
};

// Evaluate the curved half-timber solid material at a local uv. The timber
// frame is treated as a solid albedo blend between the frame and fill colors.
SolidMaterialParams evaluate_curved_halftimber(CurvedHalftimberParams p, float2 uv, float3 normal)
{
    float timber = 0.0f;
    // Frame lines along the local uv (curvature applied by the instance mesh).
    float2 line_uv = uv * p.uv_scale;
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.x) - 0.5f)));
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.y) - 0.5f)));
    float3 albedo = lerp(p.fill_color, p.timber_color, saturate(timber));
    return init_solid_material(albedo, 0.5f, p.flags);
}

#endif // TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL
