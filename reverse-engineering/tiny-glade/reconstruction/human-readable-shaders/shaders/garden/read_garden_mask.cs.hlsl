// ==RECONSTRUCTED== shaders/garden/read_garden_mask.cs.hlsl
// 依据:_garden_read_garden_mask.cs.hlsl_cs_main_0.bin 的 spirv-cross
// (garden_mask.y > 0.3 -> 位,每线程 32 纹素、512 宽行)
// 状态:O (算法 + 阈值取自 SPIR-V);I (缓冲使用内嵌 __alias_ 名)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"

Texture2D<float4> garden_mask : register(t14, space0);
RWStructuredBuffer<uint> __alias_mask_out : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    // 每个线程负责一行 512 纹素中的 32 个,起始坐标由行号推出。
    uint row_start = id.x * 32u;
    uint2 start = uint2(row_start % 512u, row_start / 512u);

    // 把 32 个纹素的"覆盖度 > 0.3"逐位打包进 uint。
    uint bits = 0u;
    for (uint i = 0u; i < 32u; i++)
    {
        uint2 coord = start + uint2(i, 0u);
        bits |= (uint(garden_mask[coord].y > 0.3f) << (i & 31u));
    }
    __alias_mask_out[id.x] = bits;
}
