// ============================================================
// terrain_editing/read_terrain_rock_mask.cs.hlsl
// 功能:读取岩石地形贴图,把每行 32 个像素的"岩石存在"打包成 32 位掩码。
// 入口:cs_main,64 线程/工作组;每线程负责一行中连续的 32 个像素。
// 绑定:rocky_terrain (t14, space0)、mask_out (u15, space0)。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl | evidence: spirv-cross of _terrain_editing_read_terrain_rock_mask.cs.hlsl_cs_main_0.bin (pack 32 rock-presence bits per row) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"

// 岩石掩码输出缓冲(每线程一个 uint)。
RWByteAddressBuffer __alias_mask_out : register(u15, space0);
// 岩石地形贴图。
Texture2D<float4> rocky_terrain : register(t14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 每线程负责 32 个连续像素:起始线性索引 = 32 * thread。
    uint index = dispatch_thread_id.x * 32u;
    // 把线性索引拆成 512 宽的二维坐标。
    uint2 pixel = uint2(index % 512u, index / 512u);

    uint mask = 0u;
    for (uint i = 0u; i < 32u; i++)
    {
        // 采样岩石贴图,综合三个通道判定"岩石强度"。
        float4 rocky = rocky_terrain.Load(int3(pixel + uint2(i, 0u), 0u));
        float rock_strength = (rocky.x + max(0.0f, rocky.y - 0.1f)) + max(0.0f, rocky.z - 0.2f);
        // 强度超过阈值则把对应位写 1。
        mask |= uint(rock_strength > 0.1f) << (i & 31u);
    }

    // 输出 32 位岩石掩码。
    __alias_mask_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, mask);
}
