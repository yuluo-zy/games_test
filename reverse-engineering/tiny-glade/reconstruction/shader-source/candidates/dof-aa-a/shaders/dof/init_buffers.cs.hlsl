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
//     names        : push_constants, tile_offsets_initial_values, __alias_dispatch_args_buffer, DofTileOffsetsPacked, tile_offsets_buffer, cs_main
// ==RECONSTRUCTED== shaders/dof/init_buffers.cs.hlsl | dof-aa-a | status: O (verbatim from SPIR-V)
//   Seeds the per-tile dispatch-args and the initial DofTileOffsetsPacked values.

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

struct PushConstants
{
    uint args_type_count;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

ConstantBuffer<DofTileOffsetsPacked> tile_offsets_initial_values : register(b13, space0);

RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u14, space0);
RWStructuredBuffer<DofTileOffsetsPacked> tile_offsets_buffer : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (dispatch_id.x < push_constants.args_type_count)
    {
        __alias_dispatch_args_buffer[dispatch_id.x] = uint4(0u, 1u, 1u, 0u);
    }

    if (dispatch_id.x == 0u)
    {
        [unroll]
        for (uint i = 0u; i < 2u; ++i)
        {
            tile_offsets_buffer[0].offsets[i] = tile_offsets_initial_values.offsets[i];
        }
    }
}
