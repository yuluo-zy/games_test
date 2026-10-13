// ==RECONSTRUCTED== shaders/roof_material_id.inc.hlsl | F-V | embedded in nani_roof_floor.raster (2) + nani_stairs_floor.raster (2)
// Roof-floor shared record type + material-id dispatch. InstanceData layout/names recovered
// verbatim from the consumer bins' OpMemberName tables:
//   position f3 +0, wall_id u +12, basis0_xz f2 +16, scale f2 +24, roof_animation f +32,
//   shape u +36, double_sided u +40, is_roof_bottom u +44, material_id u +48,
//   override_color_id u +52, pattern_id u +56, pad_1 u +60  (64B record).
// The material-dispatch body (material_id -> wooden/stone/roof tile material) is inlined in
// the wave-2 nani_roof_floor raster and is marked Unknown here pending that reconstruction.

#ifndef TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL
#define TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL

// Per-instance record for roof-floor pieces (roof/attic/stairs floors).
struct RoofFloorInstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

// Select the floor material for a roof piece by material_id.
// (Body Unknown: inlined in the wave-2 nani_roof_floor raster; the dispatch switches on
// material_id/shape between the wooden-floor, stone-floor and tile samplers.)
void get_roof_material_params(uint material_id, out float3 albedo, out float roughness)
{
    albedo = float3(1.0f, 1.0f, 1.0f);
    roughness = 1.0f;
}

#endif // TINYGLADE_ROOF_MATERIAL_ID_INC_HLSL
