// ==RECONSTRUCTED== shaders/nani/inc/cull_and_bucket_owned_custom.hlsl | nani-cull-a
// Observation: embedded as include in 8 bins (curved_halftimber / plaster /
// smoke / stairs_floor culls). Per-material cull records owned by the cull
// family. CurvedHalftimberInstanceData recovered verbatim from OpMemberName;
// the plaster/stairs_floor/smoke records live in their own incs
// (plaster.inc.hlsl, trimesh.hlsl, smoke incs).
#ifndef NANI_INC_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL
#define NANI_INC_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    uint is_straight;
    uint is_xz;
    float plank_depth;
    uint caps;
    uint mesh_atlas_part_idx;
    uint pad0;
    uint pad1;
    uint pad2;
};

#endif // NANI_INC_CULL_AND_BUCKET_OWNED_CUSTOM_HLSL
