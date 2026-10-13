// ==RECONSTRUCTED== shaders/grass/calc_dispatch_args.cs.hlsl
// Evidence: _grass_calc_dispatch_args.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_grass_dispatch_args_buffer u13 (RWStructuredBuffer<uint4>),
//   bundle_grasslet_index_buffer t14, tile_bundle_subtotal_buffer t15 (StructuredBuffer<uint>),
//   all StorageBuffer(SSBO,BufferBlock). LocalSize [1,1,1], entry cs_main, single invocation.
//   spirv-cross + OpName types: reads the block total at tile_bundle_subtotal_buffer[256]
//   (word 256, byte 1024), the last bundle's grasslet start index, then stores the grass
//   draw-arg record {last,1,1,0} at element 0 and {ceil(total/64),1,1,total} at element 1.
// Status: O (structure + types verbatim).

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"

RWStructuredBuffer<uint4> __alias_grass_dispatch_args_buffer : register(u13, space0);
StructuredBuffer<uint> bundle_grasslet_index_buffer : register(t14, space0);
StructuredBuffer<uint> tile_bundle_subtotal_buffer : register(t15, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    uint total = tile_bundle_subtotal_buffer[256];
    uint last_grasslet;
    if (total == 0u)
    {
        last_grasslet = 0u;
    }
    else
    {
        last_grasslet = bundle_grasslet_index_buffer[total - 1u];
    }
    __alias_grass_dispatch_args_buffer[0] = uint4(last_grasslet, 1u, 1u, 0u);
    __alias_grass_dispatch_args_buffer[1] = uint4((total + 63u) / 64u, 1u, 1u, total);
}
