// ==RECONSTRUCTED== shaders/garden/defs.inc.hlsl | evidence: raw-per-bin debug type/OpMemberName (_garden_spawn_* + _garden_read_bush_positions_for_cpu + _nani_bush/_nani_cull_and_bucket_*): BushXform{Affine3Packed transform}, FlowerXform{Affine3Packed transform,float3 color,uint flower_type} (64-byte record, color @48, flower_type @60); Affine3Packed itself comes from inc/utils.hlsl (consumer includes utils before this file — defs includes nothing) | status: O (struct layouts verbatim from OpMemberName); I (FlowerXform field names color/flower_type Inference — offsets match)

#ifndef TINYGLADE_GARDEN_DEFS_HLSL
#define TINYGLADE_GARDEN_DEFS_HLSL

// Affine3Packed is provided by the consumer's prior include of inc/utils.hlsl.

struct BushXform
{
    Affine3Packed transform;
};

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
};

#endif // TINYGLADE_GARDEN_DEFS_HLSL
