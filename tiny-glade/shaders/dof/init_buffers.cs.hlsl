// ==RECONSTRUCTED== shaders/dof/init_buffers.cs.hlsl | B (dof-aa review, 2026-10-13: tie, slight B; whole-struct copy matches target OpStore count).
//   Seeds the per-tile dispatch-args and the initial DofTileOffsetsPacked values.
//
// === 中文说明 ===
// 初始化 DoF tile 系统缓冲:填充每 tile 的 dispatch 参数,并写入初始
//   DofTileOffsetsPacked 值(整结构拷贝)。
// 入口 cs_main,工作组 64x1x1,无 TGV_ 变体。
// 绑定: tile_offsets_initial_values b13 (ConstantBuffer<DofTileOffsetsPacked>),
//   __alias_dispatch_args_buffer u14 (RWStructuredBuffer<uint4>),
//   tile_offsets_buffer u15 (RWStructuredBuffer<DofTileOffsetsPacked>), push_constants。

#include "inc/common.hlsl"
#include "dof/tiles.inc.hlsl"

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
    // 为前 args_type_count 个 tile 写默认 dispatch 参数 (组数 1, 尺寸 1x1)。
    if (gl_GlobalInvocationID.x < push_constants.args_type_count)
    {
        __alias_dispatch_args_buffer[gl_GlobalInvocationID.x] = uint4(0u, 1u, 1u, 0u);
    }
    // 第一个线程写入初始 tile 偏移表(整结构拷贝,匹配目标 OpStore 数量)。
    if (gl_GlobalInvocationID.x == 0u)
    {
        tile_offsets_buffer[0] = tile_offsets_initial_values;
    }
}
