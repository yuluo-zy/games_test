// ==RECONSTRUCTED== shaders/nani/subtotal_prefix_sum.cs.hlsl | evidence: raw disasm of
// _nani_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin: unrolled Hillis-Steele scan (2 elems/lane,
// 1024/lane block) with range_start/range_end end masking; no length write | status: O

#line 1 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/prefix_scan.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

struct PushConstants
{
    uint range_start;
    uint range_end;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 local_id : SV_GroupThreadID)
{
    uint idx0 = local_id.x * 2u;
    uint base = idx0 + push_constants.range_start;
    bool in_range = (base + 1u) < push_constants.range_end;

    uint2 value;
    if (in_range)
    {
        value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), __alias_inout_buf.Load<uint>((base + 1u) * 4u + 0u));
    }
    else
    {
        if (base < push_constants.range_end)
        {
            value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), 0u);
        }
        else
        {
            value = uint2(0u, 0u);
        }
    }

    shared_data2x[idx0] = value.x;
    shared_data2x[idx0 + 1u] = value.y;
    GroupMemoryBarrierWithGroupSync();

    uint d = (local_id.x >> 0u) << 1u;
    shared_data2x[d + 1u] += shared_data2x[d];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 1u) << 2u;
    shared_data2x[d + 2u + (local_id.x & 1u)] += shared_data2x[d + 1u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 2u) << 3u;
    shared_data2x[d + 4u + (local_id.x & 3u)] += shared_data2x[d + 3u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 3u) << 4u;
    shared_data2x[d + 8u + (local_id.x & 7u)] += shared_data2x[d + 7u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 4u) << 5u;
    shared_data2x[d + 16u + (local_id.x & 15u)] += shared_data2x[d + 15u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 5u) << 6u;
    shared_data2x[d + 32u + (local_id.x & 31u)] += shared_data2x[d + 31u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 6u) << 7u;
    shared_data2x[d + 64u + (local_id.x & 63u)] += shared_data2x[d + 63u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 7u) << 8u;
    shared_data2x[d + 128u + (local_id.x & 127u)] += shared_data2x[d + 127u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 8u) << 9u;
    shared_data2x[d + 256u + (local_id.x & 255u)] += shared_data2x[d + 255u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 9u) << 10u;
    shared_data2x[d + 512u + (local_id.x & 511u)] += shared_data2x[d + 511u];
    GroupMemoryBarrierWithGroupSync();

    if (in_range)
    {
        __alias_inout_buf.Store<uint>(base * 4u + 0u, shared_data2x[idx0]);
        __alias_inout_buf.Store<uint>((base + 1u) * 4u + 0u, shared_data2x[idx0 + 1u]);
    }
    else
    {
        if (base < push_constants.range_end)
        {
            __alias_inout_buf.Store<uint>(base * 4u + 0u, shared_data2x[idx0]);
        }
    }
}
