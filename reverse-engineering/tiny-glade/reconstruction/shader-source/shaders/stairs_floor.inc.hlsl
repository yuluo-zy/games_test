// ==RECONSTRUCTED== shaders/stairs_floor.inc.hlsl | F-V | embedded in nani_cull_and_bucket_stairs_floor.cs (2) + nani_stairs_floor.raster (2)
// Stairs-floor shared record types. Member layouts/names recovered verbatim from the consumer
// bins' OpMemberName tables (raw-per-bin.jsonl):
//   NaniTrimeshChunk: gpu_address (uint64), vertex_index_count, vertex_byte_offset, bounding_sphere.
//   StairsFloorInstanceData: { NaniTrimeshChunk mesh; uint color_id; pad0; pad1; pad2 }.
// Function bodies are Unknown in this pass (consumers use PhysicalStorageBufferAddresses and
// could not be spirv-cross decompiled; the nani family will refine when reconstructed).

#ifndef TINYGLADE_STAIRS_FLOOR_INC_HLSL
#define TINYGLADE_STAIRS_FLOOR_INC_HLSL

// GPU-mesh chunk referenced by physical address from a stairs instance.
struct NaniTrimeshChunk
{
    uint64_t gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};

// Per-instance record for stairs-floor rendering.
struct StairsFloorInstanceData
{
    NaniTrimeshChunk mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
};

#endif // TINYGLADE_STAIRS_FLOOR_INC_HLSL
