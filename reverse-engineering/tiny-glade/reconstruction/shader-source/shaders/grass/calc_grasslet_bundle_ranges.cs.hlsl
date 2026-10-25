// ==RECONSTRUCTED== shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl
// Evidence: _grass_calc_grasslet_bundle_ranges.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_grasslet_bundle_ranges u13 (RWStructuredBuffer<uint>),
//   grass_dispatch_args_buffer t14 (StructuredBuffer<uint4>), bundle_grasslet_index_buffer t15
//   (StructuredBuffer<uint>). LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disasm: dispatch guard reads the total at grass_dispatch_args_buffer[1].w
//   (byte 28); per grasslet index x, if bundle[x]-1 != (x==0 ? -1 : bundle[x-1]-1) then the
//   start of a new bundle range, atomic-add 128*x+1 (else 1) into grasslet_bundle_ranges[bundle[x]-1].
//   Status: O (structure + types verbatim).

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"

RWStructuredBuffer<uint> __alias_grasslet_bundle_ranges : register(u13, space0);
StructuredBuffer<uint4> grass_dispatch_args_buffer : register(t14, space0);
StructuredBuffer<uint> bundle_grasslet_index_buffer : register(t15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (dispatch_id.x >= grass_dispatch_args_buffer[1].w)
    {
        return;
    }
    uint prev;
    if (dispatch_id.x == 0u)
    {
        prev = 0xFFFFFFFFu;
    }
    else
    {
        prev = bundle_grasslet_index_buffer[dispatch_id.x - 1u] - 1u;
    }
    uint cur = bundle_grasslet_index_buffer[dispatch_id.x] - 1u;
    uint value;
    if (cur == prev)
    {
        value = 1u;
    }
    else
    {
        value = (128u * dispatch_id.x) + 1u;
    }
    InterlockedAdd(__alias_grasslet_bundle_ranges[cur], value);
}
