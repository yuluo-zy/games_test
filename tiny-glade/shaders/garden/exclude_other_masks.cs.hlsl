// ==RECONSTRUCTED== shaders/garden/exclude_other_masks.cs.hlsl
// 依据:spirv-cross + _garden_exclude_other_masks.cs.hlsl_cs_main_0.bin 的原始反汇编
// (int2 pixel = int2(id.xy);当 path.x>0.1 / rocky.x>0 / water.y>0.3 时清空 garden_mask.x,
// 否则把 g.z 复制进 .x)
// 状态:O (算法取自 SPIR-V);I (阈值字面量)
// 备注:目标经 OpLogicalNot 发射 !(cmp) 并保留 int2 位转换;本机 dxc 把取反折叠进
// FUnordLessThanEqual 并丢弃冗余位转换(dxc 版本优化器漂移,行为一致)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"

RWTexture2D<float4> garden_mask : register(u14, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> rocky_terrain : register(t16, space0);
Texture2D<float4> water_mask : register(t17, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    int2 pixel = int2(id.xy);
    float4 g = garden_mask[pixel];

    // 路径/岩石/水面任一命中即排除该格,否则把原始 g.z 覆盖为覆盖度。
    if (path_mask[pixel].x > 0.1f || rocky_terrain[pixel].x > 0.0f || water_mask[pixel].y > 0.3f)
        garden_mask[pixel].x = 0.0f;
    else
        garden_mask[pixel].x = g.z;
}
