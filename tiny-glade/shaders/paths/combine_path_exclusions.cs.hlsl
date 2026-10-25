// ==RECONSTRUCTED== shaders/paths/combine_path_exclusions.cs.hlsl
// 目标:_paths_combine_path_exclusions.cs.hlsl_cs_main_0.bin
// 依据:spirv-cross + SPIR-V 反汇编(4 个路径遮罩纹素加载之和 clamp 后写入 paths_exclusion;
// 无采样器,仅 texel Load;gid 为 int3 -> uint2 位转换,坐标同时用于取数与写回;
// 存储图像 R16f -> RWTexture2D<half>)
// 绑定:path_mask t14, cobblestone1_mask t15, cobblestone2_mask t16, cobblestone3_mask t17, paths_exclusion u18
// 状态:O (求和/clamp/存储);残留 codegen 差异(int3 构造 vs 直接 uint2 坐标)= dxc 版本产物

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> cobblestone1_mask : register(t15, space0);
Texture2D<float4> cobblestone2_mask : register(t16, space0);
Texture2D<float4> cobblestone3_mask : register(t17, space0);
RWTexture2D<half> paths_exclusion : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(int3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 像素坐标 = 分发线程坐标。
    uint2 p = uint2(gl_GlobalInvocationID.xy);
    // 路径遮罩与三张鹅卵石遮罩的红色通道之和,clamp 到 [0,1] 写入排除图。
    paths_exclusion[p] = clamp(path_mask.Load(int3(p, 0)).x
                              + cobblestone1_mask.Load(int3(p, 0)).x
                              + cobblestone2_mask.Load(int3(p, 0)).x
                              + cobblestone3_mask.Load(int3(p, 0)).x,
                              0.0f, 1.0f);
}
