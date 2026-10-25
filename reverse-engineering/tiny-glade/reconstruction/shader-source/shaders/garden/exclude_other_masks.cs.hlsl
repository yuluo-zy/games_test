// ==RECONSTRUCTED== shaders/garden/exclude_other_masks.cs.hlsl | evidence: spirv-cross + raw disasm of _garden_exclude_other_masks.cs.hlsl_cs_main_0.bin (int2 pixel = int2(id.xy); clear garden_mask.x where path.x>0.1 / rocky.x>0 / water.y>0.3, else copy g.z into .x) | status: O (algorithm from SPIR-V); I (threshold literals) | note: target emits !(cmp) via OpLogicalNot and keeps the int2 bitcast; installed dxc folds the negation into FUnordLessThanEqual and drops the redundant bitcast (dxc-version optimizer drift, behavior identical)

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

    if (path_mask[pixel].x > 0.1f || rocky_terrain[pixel].x > 0.0f || water_mask[pixel].y > 0.3f)
        garden_mask[pixel].x = 0.0f;
    else
        garden_mask[pixel].x = g.z;
}
