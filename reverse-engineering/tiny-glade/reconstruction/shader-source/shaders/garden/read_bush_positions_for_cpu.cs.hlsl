// ==RECONSTRUCTED== shaders/garden/read_bush_positions_for_cpu.cs.hlsl | evidence: spirv-cross + raw disasm of _garden_read_bush_positions_for_cpu.cs.hlsl_cs_main_0.bin (thread0 writes float2(count,count); other threads read BushXform affine from bush_buffer[id-1], build float4x3, mul(transpose(m),(0,0,0,1)).xz -> bush_cpu) | status: O (algorithm + struct types from SPIR-V); I (float2 count store = CPU readback convention)

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

    if (id.x == 0u)
    {
        __alias_bush_cpu[id.x] = asfloat(int(count)).xx;
    }
    else if (id.x < uint(int(count) + 1))
    {
        float data[12] = bush_buffer[id.x - 1u].transform.data;

        float4x3 m = float4x3(
            float3(data[0], data[1], data[2]),
            float3(data[3], data[4], data[5]),
            float3(data[6], data[7], data[8]),
            float3(data[9], data[10], data[11]));

        __alias_bush_cpu[id.x] = mul(transpose(m), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    }
}
