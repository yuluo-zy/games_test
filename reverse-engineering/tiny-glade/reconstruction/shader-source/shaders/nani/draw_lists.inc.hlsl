// ==RECONSTRUCTED== shaders/nani/draw_lists.inc.hlsl | evidence: raw-per-bin debug types of
// nani cull/fill bins: GpuFilteredInstance{raw}, GpuFilteredInstanceMeta{raw},
// NaniDrawListInfo{global_instance_offset}; nani bins' DrawIndexedIndirectCommand carries member
// names index_count/instance_count/first_index/vertex_offset/first_instance (distinct from the
// draw_indirect_types.hlsl names used by the wall/grass chain) | status: O (layouts verbatim)

#ifndef TINYGLADE_NANI_DRAW_LISTS_HLSL
#define TINYGLADE_NANI_DRAW_LISTS_HLSL

// One raw-packed filtered-instance index (mesh*64 + vertex/instance within mesh).
struct GpuFilteredInstance
{
    uint raw;
};

// Per-instance meta (raw instance index within the draw list).
struct GpuFilteredInstanceMeta
{
    uint raw;
};

// Per-draw-list info read by the cull pass (base offset of the draw list).
struct NaniDrawListInfo
{
    uint global_instance_offset;
};

// Vulkan DrawIndexedIndirectCommand as used by the nani draw-list chain.
struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif // TINYGLADE_NANI_DRAW_LISTS_HLSL
