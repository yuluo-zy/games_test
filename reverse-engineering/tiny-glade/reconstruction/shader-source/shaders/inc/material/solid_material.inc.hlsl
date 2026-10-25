// ==RECONSTRUCTED== shaders/inc/material/solid_material.inc.hlsl | evidence: include-graph edge material/solid_material.inc.hlsl -> brdf_lut.hlsl (raw embedded style "eval/../solid_material.inc.hlsl"); consumers _nani_*_bricks/_plaster/_roof_floor/_wall_*_brick read solid albedo/roughness from instance data + brdf LUT | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL

// Per-instance solid surface parameters used by the deferred/forward lighting.
struct SolidMaterialParams {
    float3 albedo;
    float roughness;
    uint flags; // low bits: render flags (highlight, shadows, ao, ...)
};

// Build solid params from an albedo/roughness pair (instance or texture color).
SolidMaterialParams init_solid_material(float3 albedo, float roughness, uint flags)
{
    SolidMaterialParams p;
    p.albedo = albedo;
    p.roughness = saturate(roughness);
    p.flags = flags;
    return p;
}

#endif // TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
