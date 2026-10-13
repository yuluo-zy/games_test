// ==RECONSTRUCTED== shaders/inc/draw_indirect_types.hlsl | evidence: raw-per-bin (_generate_grass/_wall_generate_draw_lists/_garden_*/_nani_draw_indexed_indirect_to_dispatch) OpMemberName DrawIndexedIndirectCommand{count,instanceCount,firstIndex,vertexOffset,firstInstance} stride 0x14, DrawIndirectCommand{vertexCount,instanceCount,firstVertex,firstInstance} stride 0x10 | status: O

#ifndef TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL

// Vulkan-compatible indexed indirect draw command (stride 0x14). Fields and
// order match the embedded OpMemberName table exactly.
struct DrawIndexedIndirectCommand {
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

// Vulkan-compatible non-indexed indirect draw command (stride 0x10).
struct DrawIndirectCommand {
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

#endif // TINYGLADE_INC_DRAW_INDIRECT_TYPES_HLSL
