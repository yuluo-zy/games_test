// ==RECONSTRUCTED== shaders/dof/clear_halfres.cs.hlsl | A (dof-aa review, 2026-10-13: tie, include form) + Rgba16f fix: img_output RWTexture2D<half4> (target format operand Rgba16f, same buffer halfres writes).
//   Clears the half-res color output for the tile list (group offsets[6] from DofTileOffsets).
//
// === 中文说明 ===
// 清空半分辨率颜色输出:遍历 tile 列表中的第 6 组(clear_halfres),把对应
//   8x8 tile 的颜色清为 (0,0,0,0)。
// 入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
// 绑定: tile_offsets_buffer t15 (StructuredBuffer<DofTileOffsets>),
//   tile_coords_buffer t16, img_output u14 (RWTexture2D<half4> Rgba16f)。

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t15, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t16, space0);
RWTexture2D<half4> img_output : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID)
{
    // 该 pass 在 tile 偏移表中的组号固定为 6。
    uint tile_base = tile_offsets_buffer[0].offsets[6] + group_id.x;
    // 8x8 tile 的左上角像素坐标。
    uint2 tile_origin = uint2(8u, 8u) * tile_coords_buffer[tile_base];
    // 清空本线程在 tile 内的像素。
    img_output[(dispatch_id.xy & uint2(7u, 7u)) + tile_origin] = float4(0.0f, 0.0f, 0.0f, 0.0f);
}
