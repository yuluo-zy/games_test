// ==RECONSTRUCTED== shaders/fill_buffer.cs.hlsl | F-D
// 证据: _fill_buffer.cs.hlsl_cs_main_0.bin (SPIR-V, 偏移 23)。
//   绑定: push_constants (PushConstant 存储, cbuffer, 成员 value+range),
//   __alias_output u13/space0 (StorageBuffer uint[]), 工作组尺寸 [64,1,1],
//   入口 cs_main。
//   用途: 通用填充计算着色器 —— 把前 64 个线程(dispatchThreadID.x < range)对应
//   的缓冲区元素写成常量 value。
// 状态: O(存储类型;成员名 `value`/`range` 直接取自 OpMemberName)。

#include "inc/common.hlsl"

// CPU 侧通过 push constant 传入填充值和填充范围。
struct PushConstants
{
    uint value;
    uint range;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

// 被填充的目标缓冲区(原始写入别名)。
RWStructuredBuffer<uint> __alias_output : register(u13, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 只在有效范围内写入常量值。
    if (dispatchThreadID.x < push_constants.range)
    {
        __alias_output[dispatchThreadID.x] = push_constants.value;
    }
}
