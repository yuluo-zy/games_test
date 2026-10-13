// ==RECONSTRUCTED== shaders/garden/read_bush_positions_for_cpu.cs.hlsl
// 依据:spirv-cross + _garden_read_bush_positions_for_cpu.cs.hlsl_cs_main_0.bin 的原始反汇编
// (线程0写 float2(count,count);其他线程从 bush_buffer[id-1] 读 BushXform 仿射,
// 构建 float4x3,mul(transpose(m),(0,0,0,1)).xz -> bush_cpu)
// 状态:O (算法 + 结构类型取自 SPIR-V);I (float2 count 存储 = CPU 回读约定)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect_readonly.hlsl"
#include "garden/defs.inc.hlsl"

StructuredBuffer<DrawIndexedIndirectCommand> draw_commands : register(t13, space0);
StructuredBuffer<BushXform> bush_buffer : register(t15, space0);
RWStructuredBuffer<float2> __alias_bush_cpu : register(u16, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
};

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint count = draw_commands[uint(draw_indirect_buffer_offset)].instanceCount;

    // 线程 0 写出灌木数量(count,count),供 CPU 回读。
    if (id.x == 0u)
    {
        __alias_bush_cpu[id.x] = asfloat(int(count)).xx;
    }
    // 其余线程把第 id.x-1 棵灌木的仿射变换原点变换到世界 xz。
    else if (id.x < uint(int(count) + 1))
    {
        float data[12] = bush_buffer[id.x - 1u].transform.data;

        // Affine3Packed 的行主仿射 -> float4x3。
        float4x3 m = float4x3(
            float3(data[0], data[1], data[2]),
            float3(data[3], data[4], data[5]),
            float3(data[6], data[7], data[8]),
            float3(data[9], data[10], data[11]));

        // 转置矩阵乘以原点 (0,0,0,1),取 xz 平面位置。
        __alias_bush_cpu[id.x] = mul(transpose(m), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    }
}
