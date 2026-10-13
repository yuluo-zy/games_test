// ==RECONSTRUCTED== shaders/fsr_copy_history.cs.hlsl | B (dof-aa 复核, 2026-10-13):
//   B 方案胜出 (input_tex[id.xy] 纹素直接索引与 OpImageFetch + OpVectorShuffle 匹配;
//   A 方案的 Load() 会多出操作码)。
//   用途: 简单的输入->输出拷贝,output_tex[id.xy] = input_tex[id.xy]。
//   绑定: input_tex t14, output_tex u15, 工作组尺寸 [8,8,1], 入口 cs_main。

#include "inc/common.hlsl"

// 历史帧输入。
Texture2D<float4> input_tex : register(t14, space0);
// 拷贝目标。
RWTexture2D<float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 逐像素原样拷贝。
    output_tex[gl_GlobalInvocationID.xy] = input_tex[gl_GlobalInvocationID.xy];
}
