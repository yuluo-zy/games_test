// ==RECONSTRUCTED== shaders/nani/subtotal_prefix_sum.cs.hlsl
// 依据:_nani_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin 的原始反汇编:
// 展开的 Hillis-Steele 扫描(每通道 2 元素,1024/通道块)带 range_start/range_end
// 端点掩码;不写长度。
// 状态:O

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
    // 本通道覆盖 2 个元素(base, base+1);越界部分掩码为 0。
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

    // 载入 groupshared 后,展开 10 级 Hillis-Steele 包含扫描(步距 1,2,4,...,512)。
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

    // 只回写范围内的元素(与载入掩码对称)。
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
