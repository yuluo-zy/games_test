// ==RECONSTRUCTED== shaders/plaster.inc.hlsl | F-V | embedded in nani_cull_and_bucket_plaster.cs (2) + nani_plaster.raster (2)
// Plaster-wall shared record type. Member layout/names recovered verbatim from the consumer
// bins' OpMemberName tables:
//   PlasterInstanceData: bounding_sphere (float4), wall_id (uint), wall_length (float),
//     blend_uv_seam (float), quad_count (uint), verts_gpu_address (uint64),
//     row_stride (uint), peel_strength (float)  (64-byte packed layout).
// Function bodies are Unknown in this pass (physical-addressing consumers; nani family refines).

#ifndef TINYGLADE_PLASTER_INC_HLSL
#define TINYGLADE_PLASTER_INC_HLSL

// Per-wall plaster quad record (the raster converts each instance to quad_count quads).
struct PlasterInstanceData
{
    float4 bounding_sphere;
    uint wall_id;
    float wall_length;
    float blend_uv_seam;
    uint quad_count;
    uint64_t verts_gpu_address;
    uint row_stride;
    float peel_strength;
};

#endif // TINYGLADE_PLASTER_INC_HLSL
