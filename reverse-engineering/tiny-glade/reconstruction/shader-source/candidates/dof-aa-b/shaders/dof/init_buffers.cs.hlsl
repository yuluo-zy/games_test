// ==SKELETON==
// SOURCE PATH : shaders/dof/init_buffers.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_init_buffers.cs.hlsl_cs_main_0.bin
//     file_sha256  : 5ba58e0cac151637d80e30bbbc3e28ff0fb4b10d292463211847be199982907c
//     pass/stage   : _dof_init_buffers (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [64, 1, 1]
//     index_pass   : W49 (Inference)
//     names        : push_constants, tile_offsets_initial_values, __alias_dispatch_args_buffer, DofTileOffsetsPacked, tile_offsets_buffer, cs_main
//
// ==RECONSTRUCTED== shaders/dof/init_buffers.cs.hlsl
// Evidence: _dof_init_buffers.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 58).
//   bindings: push_constants (PushConstant {args_type_count}), tile_offsets_initial_values
//   b13 (ConstantBuffer<DofTileOffsetsPacked>), __alias_dispatch_args_buffer u14
//   (RWStructuredBuffer<uint4>), tile_offsets_buffer u15 (RWStructuredBuffer<DofTileOffsetsPacked>);
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross: if (gid.x < args_type_count) dispatch_args_buffer[gid.x] =
//   uint4(0,1,1,0); if (gid.x == 0) tile_offsets_buffer[i] = initial.offsets[i].
// Status: O (structure from disassembly; names from debug_names).

#include "inc/common.hlsl"
#include "tiles.inc.hlsl"

struct PushConstants
{
    uint args_type_count;
};
[[vk::push_constant]]
PushConstants push_constants;

ConstantBuffer<DofTileOffsetsPacked> tile_offsets_initial_values : register(b13, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u14, space0);
RWStructuredBuffer<DofTileOffsetsPacked> tile_offsets_buffer : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x < push_constants.args_type_count)
    {
        __alias_dispatch_args_buffer[gl_GlobalInvocationID.x] = uint4(0u, 1u, 1u, 0u);
    }
    if (gl_GlobalInvocationID.x == 0u)
    {
        tile_offsets_buffer[0] = tile_offsets_initial_values;
    }
}
