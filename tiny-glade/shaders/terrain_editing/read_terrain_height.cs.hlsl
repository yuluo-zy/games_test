// ============================================================
// terrain_editing/read_terrain_height.cs.hlsl
// 功能:把地形高度图(普通/视觉两套)读取并打包为成对的 16 位高度值,
//       每个线程输出一个 uint(两个相邻像素高度),写入两个别名缓冲。
// 入口:cs_main,64 线程/工作组,每线程负责一对相邻像素。
// 绑定:terrain_heightmap (t14)、visual_terrain_heightmap (t15)、
//       height_out (u16)、visual_height_out (u17)。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/read_terrain_height.cs.hlsl | evidence: spirv-cross of _terrain_editing_read_terrain_height.cs.hlsl_cs_main_0.bin (pairs of 16-bit packed heights into two alias buffers) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/pack_unpack.hlsl"
Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"

// 打包高度输出缓冲。
RWByteAddressBuffer __alias_height_out : register(u16, space0);
// 视觉高度输出缓冲。
RWByteAddressBuffer __alias_visual_height_out : register(u17, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);

// Pack a [0,1] height sample into a 16-bit unsigned integer.
// 把 [0,1] 的高度采样打包成 16 位无符号整数。
uint pack_height(float h)
{
    return uint((clamp(h, 0.0f, 1.0f) * 65535.0f) + 0.5f);
}

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 每个线程处理两个连续像素:index = 2*thread, index+1 = 下一个像素。
    uint index = dispatch_thread_id.x * 2u;
    // 把线性索引拆成 512 宽的二维坐标。
    uint2 pixel = uint2(index % 512u, index / 512u);
    uint2 pixel_next = pixel + uint2(1u, 0u);

    // 打包两个相邻像素的高度到同一个 uint(低 16 位 + 高 16 位)。
    uint packed = pack_height(terrain_heightmap.Load(int3(pixel, 0)).x)
                + (pack_height(terrain_heightmap.Load(int3(pixel_next, 0)).x) << 16u);
    // 视觉高度同法打包。
    uint packed_visual = pack_height(visual_terrain_heightmap.Load(int3(pixel, 0)).x)
                       + (pack_height(visual_terrain_heightmap.Load(int3(pixel_next, 0)).x) << 16u);

    // 写入对应的打包缓冲。
    __alias_height_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, packed);
    __alias_visual_height_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, packed_visual);
}
