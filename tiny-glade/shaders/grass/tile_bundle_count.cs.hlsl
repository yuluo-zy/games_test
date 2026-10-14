// ==RECONSTRUCTED== shaders/grass/tile_bundle_count.cs.hlsl
// Evidence: _grass_tile_bundle_count.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_tile_bundle_prefix_buffer u13, __alias_tile_bundle_subtotal_buffer u14
//   (RWStructuredBuffer<uint>), tile_buffer t15 (StructuredBuffer<GrassTilePacked>),
//   blade_prefix_buffer t16, blade_prefix_subtotal_buffer t17 (StructuredBuffer<uint>).
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disasm: per tile, compute the grasslet count (raw&0xFFFF)%512 and the
//   number of 64-slot bundle halves the tile spans given the group's blade-prefix offset,
//   then a 64-lane inclusive Hillis-Steele scan (6 unrolled steps, 7 barriers) into
//   blade-prefix buffer + group subtotal.
//   Status: O (structure verbatim; the original scan is unrolled, not the shared macro loop).
//
// 瓦片束计数:每瓦片先算其草刀片跨过的束数,再做 64 lane 的 Hillis-Steele
// 含式扫描,写出前缀与组子合计。

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"
#include "inc/prefix_scan.hlsl"

RWStructuredBuffer<uint> __alias_tile_bundle_prefix_buffer : register(u13, space0);
RWStructuredBuffer<uint> __alias_tile_bundle_subtotal_buffer : register(u14, space0);
StructuredBuffer<GrassTilePacked> tile_buffer : register(t15, space0);
StructuredBuffer<uint> blade_prefix_buffer : register(t16, space0);
StructuredBuffer<uint> blade_prefix_subtotal_buffer : register(t17, space0);

groupshared uint shared_data1x[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    // 计算本瓦片在组内刀片前缀偏移下占用的束数(按 64 槽切分)。
    uint tile_blades = (tile_buffer[dispatch_id.x].raw & 0xFFFFu) % 512u;
    uint slots = 64u - ((blade_prefix_buffer[dispatch_id.x] + blade_prefix_subtotal_buffer[group_id.x]) % 64u);
    uint bundle_count = 0u;
    for (uint remaining = tile_blades; remaining > 0u; )
    {
        uint take = min(remaining, slots);
        uint slots_left = slots - take;
        slots = (slots_left == 0u) ? 64u : slots_left;
        bundle_count++;
        remaining -= take;
    }
    shared_data1x[local_index] = bundle_count;
    GroupMemoryBarrierWithGroupSync();
    // Hillis-Steele 6 步扫描(步长 1..32,带边界保护)。
    uint i = (local_index >> 0u) << 1u;
    uint j = i + 1u;
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 1u) << 2u;
    j = i + 2u + (local_index & 1u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 1u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 2u) << 3u;
    j = i + 4u + (local_index & 3u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 3u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 3u) << 4u;
    j = i + 8u + (local_index & 7u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 7u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 4u) << 5u;
    j = i + 16u + (local_index & 15u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 15u];
    }
    GroupMemoryBarrierWithGroupSync();
    i = (local_index >> 5u) << 6u;
    j = i + 32u + (local_index & 31u);
    if (j < 64u)
    {
        shared_data1x[j] += shared_data1x[i + 31u];
    }
    GroupMemoryBarrierWithGroupSync();
    // 排他前缀 + 组子合计。
    __alias_tile_bundle_prefix_buffer[dispatch_id.x] = (local_index == 0u) ? 0u : shared_data1x[local_index - 1u];
    if (local_index == 0u)
    {
        __alias_tile_bundle_subtotal_buffer[group_id.x] = shared_data1x[63];
    }
}
