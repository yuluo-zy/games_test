// ==RECONSTRUCTED== shaders/nani/draw_lists.inc.hlsl | nani-cull-a
// Observation: embedded as include in 30 nani compute bins. Shared record
// types of the cull/fill chain, recovered verbatim from consumer OpTypeStruct/
// debug-name tables:
//   GpuFilteredInstance     { uint raw; }  (packed instance ref, 32-bit)
//   GpuFilteredInstanceMeta { uint raw; }  (packed per-instance meta, 32-bit)
//   DrawIndexedIndirectCommand              (5x uint, stride 0x14; member names
//     recovered from the nani-family OpMemberName table: index_count,
//     instance_count, first_index, vertex_offset, first_instance)
// NaniDrawListInfo is referenced by the cull family (StructuredBuffer
// draw_list_info) and is elaborated there (see cull_and_bucket.inc.hlsl).
#ifndef NANI_DRAW_LISTS_INC_HLSL
#define NANI_DRAW_LISTS_INC_HLSL

struct GpuFilteredInstance
{
    uint raw;
};

struct GpuFilteredInstanceMeta
{
    uint raw;
};

struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif // NANI_DRAW_LISTS_INC_HLSL
