// ==RECONSTRUCTED== shaders/grass/tile_bundle_gen.cs.hlsl
// Evidence: _grass_tile_bundle_gen.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_bundle_buffer u13 (RWStructuredBuffer<GrassBundlePacked>),
//   __alias_bundle_grasslet_index_buffer u14 (RWStructuredBuffer<uint>),
//   tile_bundle_prefix_buffer t15, tile_bundle_subtotal_buffer t16, blade_prefix_buffer t18,
//   blade_prefix_subtotal_buffer t19 (StructuredBuffer<uint>), tile_buffer t17
//   (StructuredBuffer<GrassTilePacked>). LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disasm: per tile, walk the 64-slot bundle halves and write a
//   GrassBundlePacked{ tile_x<<24 | tile_y<<16 | ((tile_blades-remaining)*128 + take) }
//   record plus a grasslet-start marker (uint(slots==64)) into bundle_grasslet_index_buffer.
//   Status: O (structure verbatim).
//
// 草束生成:每个瓦片按 64 槽的束切分其刀片,写入束记录与草刀片束索引标记。

#include "inc/common.hlsl"
#include "grass/grass_common.inc.hlsl"

RWStructuredBuffer<GrassBundlePacked> __alias_bundle_buffer : register(u13, space0);
RWStructuredBuffer<uint> __alias_bundle_grasslet_index_buffer : register(u14, space0);
StructuredBuffer<uint> tile_bundle_prefix_buffer : register(t15, space0);
StructuredBuffer<uint> tile_bundle_subtotal_buffer : register(t16, space0);
StructuredBuffer<GrassTilePacked> tile_buffer : register(t17, space0);
StructuredBuffer<uint> blade_prefix_buffer : register(t18, space0);
StructuredBuffer<uint> blade_prefix_subtotal_buffer : register(t19, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    // 本瓦片束记录的起始索引(前缀 + 组内子合计)。
    uint bundle_base = tile_bundle_prefix_buffer[dispatch_id.x] + tile_bundle_subtotal_buffer[group_id.x];
    GrassTilePacked tile = tile_buffer[dispatch_id.x];
    uint tile_x = (tile.raw >> 24u) & 255u;
    uint tile_y = (tile.raw >> 16u) & 255u;
    uint tile_blades = (tile.raw & 0xFFFFu) % 512u;
    // 当前束剩余槽位数(束为 64,前一瓦片占用的取反)。
    uint slots = 64u - ((blade_prefix_buffer[dispatch_id.x] + blade_prefix_subtotal_buffer[group_id.x]) % 64u);
    uint index = 0u;
    // 把瓦片刀片按 64 槽切分,逐个写入束记录。
    for (uint remaining = tile_blades; remaining > 0u; )
    {
        uint take = min(remaining, slots);
        uint bundle_index = bundle_base + index;
        GrassBundlePacked bundle = { ((tile_x << 24u) | (tile_y << 16u)) | (((tile_blades - remaining) * 128u) + take) };
        __alias_bundle_buffer[bundle_index] = bundle;
        // 束起点标记:只有当束恰好从 64 槽满束开始时才为 1。
        __alias_bundle_grasslet_index_buffer[bundle_index] = uint(slots == 64u);
        uint slots_left = slots - take;
        slots = (slots_left == 0u) ? 64u : slots_left;
        index++;
        remaining -= take;
    }
}
