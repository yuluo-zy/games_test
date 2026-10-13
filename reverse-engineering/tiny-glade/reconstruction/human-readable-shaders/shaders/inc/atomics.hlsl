// ============================================================
// inc/atomics.hlsl —— 原子计数器辅助函数
// 用途:为追加式/分配器缓冲区提供原子计数操作。
//   用于草实例(grass_blade_data)等场景的计数器缓冲区。
// 入口:无(纯头文件,被着色器 include 后调用)
// 关键绑定:参数传入 RWByteAddressBuffer 计数器
// ============================================================
// ==RECONSTRUCTED== shaders/inc/atomics.hlsl | evidence: embedded in 5 bins (grass_blade_data.hlsl family); no named functions recoverable | status: I/U (names/bodies Inference — atomic helper surface for grass / instance counters)
//
// Atomic counter helpers for append/allocator buffers.

#ifndef TINYGLADE_INC_ATOMICS_HLSL
#define TINYGLADE_INC_ATOMICS_HLSL

// 原子自增计数器,返回自增前的旧值(即"分配到的槽位")。
uint atomic_append(RWByteAddressBuffer counter_buffer, uint counter_byte_offset)
{
    uint previous;
    counter_buffer.InterlockedAdd(counter_byte_offset, 1u, previous);
    return previous;
}

// 原子地把 `value` 加到计数器,返回加之前的值。
uint atomic_add(RWByteAddressBuffer buffer, uint byte_offset, uint value)
{
    uint previous;
    buffer.InterlockedAdd(byte_offset, value, previous);
    return previous;
}

#endif // TINYGLADE_INC_ATOMICS_HLSL
