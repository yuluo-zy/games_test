// ============================================================
// terrain_editing/cache_stroke.cs.hlsl
// 功能:把当前笔划(stroke)累加进缓存(cache)后清空笔划。
//       每个线程处理一个像素:取笔划与缓存的红色通道最大值写入缓存,
//       并把笔划像素清零(标记已消费)。
// 入口:cs_main,8x8 线程/工作组。
// 绑定:stroke (u14, space0)、cache (u15, space0)。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/cache_stroke.cs.hlsl | evidence: spirv-cross of _terrain_editing_cache_stroke.cs.hlsl_cs_main_0.bin (accumulate max stroke into cache, clear stroke) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/smooth_max.hlsl"

// 当前笔划(待累加并清空)。
RWTexture2D<float4> stroke : register(u14, space0);
// 历史笔划累加缓存。
RWTexture2D<float4> cache : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 线程对应的像素坐标。
    uint2 pixel = uint2(int3(dispatch_thread_id).xy);
    // 取笔划与缓存中较强的值,更新缓存(等值写入 RGB 三通道)。
    float m = max(stroke[pixel].x, cache[pixel].x);
    cache[pixel] = float4(m, m, m, 1.0f);
    // 清空笔划,表示该像素的笔划已并入缓存。
    stroke[pixel] = float4(0.0f, 0.0f, 0.0f, 1.0f);
}
