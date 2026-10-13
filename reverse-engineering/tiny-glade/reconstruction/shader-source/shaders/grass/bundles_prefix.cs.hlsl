// ==RECONSTRUCTED== shaders/grass/bundles_prefix.cs.hlsl
// Evidence: _grass_bundles_prefix.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_tile_bundle_subtotal_buffer u13 (RWByteAddressBuffer).
//   LocalSize [128,1,1], entry cs_main.
//   spirv-cross + disasm: 128 threads, two words per lane over the 256-word block;
//   loads 2 words, 8-step inclusive Hillis-Steele scan (256 elements), writes back the
//   exclusive prefix plus the block total at word 256 (byte 1024).
//   Status: O (structure verbatim; original scan is unrolled).

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"
#include "inc/prefix_scan.hlsl"

RWByteAddressBuffer __alias_tile_bundle_subtotal_buffer : register(u13, space0);

groupshared uint shared_data2x[256];

[numthreads(128, 1, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint word = dispatch_id.x * 2u;
    shared_data2x[word] = __alias_tile_bundle_subtotal_buffer.Load<uint>(word * 4 + 0);
    shared_data2x[word + 1u] = __alias_tile_bundle_subtotal_buffer.Load<uint>(word * 4 + 4);
    GroupMemoryBarrierWithGroupSync();
    uint i = (dispatch_id.x >> 0u) << 1u;
    shared_data2x[i + 1u] += shared_data2x[i];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 1u) << 2u;
    shared_data2x[i + 2u + (dispatch_id.x & 1u)] += shared_data2x[i + 1u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 2u) << 3u;
    shared_data2x[i + 4u + (dispatch_id.x & 3u)] += shared_data2x[i + 3u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 3u) << 4u;
    shared_data2x[i + 8u + (dispatch_id.x & 7u)] += shared_data2x[i + 7u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 4u) << 5u;
    shared_data2x[i + 16u + (dispatch_id.x & 15u)] += shared_data2x[i + 15u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 5u) << 6u;
    shared_data2x[i + 32u + (dispatch_id.x & 31u)] += shared_data2x[i + 31u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 6u) << 7u;
    shared_data2x[i + 64u + (dispatch_id.x & 63u)] += shared_data2x[i + 63u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 7u) << 8u;
    shared_data2x[i + 128u + (dispatch_id.x & 127u)] += shared_data2x[i + 127u];
    GroupMemoryBarrierWithGroupSync();
    uint excl;
    if (dispatch_id.x > 0u)
    {
        excl = shared_data2x[word - 1u];
    }
    else
    {
        excl = 0u;
    }
    __alias_tile_bundle_subtotal_buffer.Store<uint>(word * 4 + 0, excl);
    __alias_tile_bundle_subtotal_buffer.Store<uint>(word * 4 + 4, shared_data2x[word]);
    if (local_index == 0u)
    {
        __alias_tile_bundle_subtotal_buffer.Store<uint>(1024, shared_data2x[255]);
    }
}
