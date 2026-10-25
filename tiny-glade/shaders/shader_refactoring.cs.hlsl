// ==RECONSTRUCTED== shaders/shader_refactoring.cs.hlsl | F-D
// 证据: _shader_refactoring.cs.hlsl_capture_0.bin (入口 `capture`, [8,8,1])
//   + _shader_refactoring.cs.hlsl_compare_0.bin (入口 `compare`, [8,8,1])。
//   绑定: rendered_tex t14, reference_tex u15 (capture); + feedback_tex u16 (compare)。
//   能力: Shader, StorageImageExtendedFormats, StorageImageWriteWithoutFormat。
//   spirv-cross capture: reference_tex[gid.xy] = rendered_tex[gid.xy].xyz;
//   spirv-cross compare: 逐分量 |rendered-reference|>1e-5 时在压暗 8% 的
//     (0,1,1) 参考色调上叠加红色标记。
// Status: O (函数体来自反汇编;入口名 `capture`/`compare` 逐字保留)。

#include "inc/common.hlsl"

// 待对比的已渲染画面(只读)
Texture2D<float4> rendered_tex : register(t14, space0);
// 保存捕获的参考画面的目标纹理
RWTexture2D<float3> reference_tex : register(u15, space0);
// 输出差异标记的反馈纹理
RWTexture2D<float3> feedback_tex : register(u16, space0);

// 捕获入口:把当前渲染结果逐像素写入参考纹理。
[numthreads(8, 8, 1)]
void capture(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 拷贝 RGB,忽略 alpha。
    reference_tex[dispatchThreadID.xy] = rendered_tex[dispatchThreadID.xy].xyz;
}

// 对比入口:逐像素比较渲染结果与参考,超过阈值的地方在反馈纹理上标红。
[numthreads(8, 8, 1)]
void compare(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 读取参考像素(RGB)。
    float3 reference = reference_tex[dispatchThreadID.xy].xyz;
    // 计算渲染结果与参考的逐分量绝对差。
    float3 diff = abs(rendered_tex[dispatchThreadID.xy].xyz - reference);
    // 任一通道差超过 1e-5 即认为该像素有差异。
    bool any_diff = any(diff > 1.0e-5f);
    // 输出 = 压暗 8% 的青绿色参考色调,有差异时叠加红色(1,0,0)标记。
    feedback_tex[dispatchThreadID.xy] = reference * 0.08f * float3(0.0f, 1.0f, 1.0f)
        + select(bool3(any_diff, any_diff, any_diff), float3(1.0f, 0.0f, 0.0f), float3(0.0f, 0.0f, 0.0f));
}
