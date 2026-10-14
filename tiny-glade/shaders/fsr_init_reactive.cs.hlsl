// ==RECONSTRUCTED== shaders/fsr_init_reactive.cs.hlsl | B (dof-aa 复核, 2026-10-13:
//   平局;采用 B 方案主体)。RWTexture2D<half> u14 (R16f) + $Globals b13
//   {output_tex_dims, value};写入 output_tex[id.xy] = value。
//   用途: 用常量 value 填充 FSR 反应掩码(reactive mask)缓冲区。
//   工作组尺寸 [8,8,1],入口 cs_main。

#include "inc/common.hlsl"

// 输出尺寸与填充常量。
cbuffer $Globals : register(b13, space0)
{
    uint2 output_tex_dims;
    float value;
};

// 反应掩码输出(R16 半浮点)。
RWTexture2D<half> output_tex : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 全图写成常量值。
    output_tex[gl_GlobalInvocationID.xy] = value;
}
