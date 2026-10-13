// ==RECONSTRUCTED== shaders/inc/material/plant_material.inc.hlsl | evidence: include-graph edges material/plant_material.inc.hlsl -> material_eval.inc.hlsl and -> raymarch.hlsl (raw embedded style "eval/../plant_material.inc.hlsl"); consumers _clearing_meadow_*_flower/_garden_lowpoly_flower/_ivy_* | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL

#include "../material_eval.inc.hlsl"

// Per-leaf/flower plant surface parameters with soft translucency.
struct PlantMaterialParams {
    float3 base_color;
    float3 flower_color;
    float roughness;
    float translucency;
    uint is_flower;
};

PlantMaterialParams init_plant_material(float3 base_color, float3 flower_color, float roughness, uint is_flower)
{
    PlantMaterialParams p;
    p.base_color = base_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    p.is_flower = is_flower;
    return p;
}

#endif // TINYGLADE_INC_MATERIAL_PLANT_MATERIAL_HLSL
