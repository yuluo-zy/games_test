// ==RECONSTRUCTED== shaders/extract_half_res_depth_normals.cs.hlsl | F-D
// 证据: _extract_half_res_depth_normals.cs.hlsl_cs_main_0.bin (SPIR-V, 偏移 23)。
//   绑定: stencil_tex t14 (uint4), depth_input_tex t15, gbuffer_input_tex t16
//   (uint4), depth_output_tex u17, normals_output_tex u18 (snorm float4),
//   frame_constants set1/b0, 工作组尺寸 [8,8,1], 入口 cs_main。
//   用途: 把全分辨率深度/法线抽取到半分辨率缓冲。每个半分辨率像素按
//   frame_index & 3 在 2x2 半分辨率块中选取抖动子像素,读取该处深度,
//   依据模板缓冲的水体位 (1|2) 置位/清除深度最低位后写出;再把 RGB8 打包的
//   G 缓冲法线解包并变换到视图空间(归一化),连同象限符号标记一并写出。
// 状态: O(结构来自反汇编)。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/halfres.hlsl"

// 模板缓冲,低 3 位记录材质标记(1/2 为水体)。
Texture2D<uint4> stencil_tex : register(t14, space0);
// 全分辨率深度输入。
Texture2D<float4> depth_input_tex : register(t15, space0);
// 全分辨率 G 缓冲,法线以 RGB8 打包。
Texture2D<uint4> gbuffer_input_tex : register(t16, space0);
// 半分辨率深度输出。
RWTexture2D<float> depth_output_tex : register(u17, space0);
// 半分辨率法线输出(snorm 浮点,alpha 携带象限符号)。
RWTexture2D<snorm float4> normals_output_tex : register(u18, space0);

// 2x2 半分辨率块内的 4 个抖动子像素偏移(旋转序)。
static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 按帧索引低 2 位选取当前帧的抖动子像素。
    uint2 sub = hi_px_subpixels[frame_index & 3u];
    // 由半分辨率像素坐标反推对应的全分辨率像素(块起点 + 子像素偏移)。
    uint2 px = uint2(int2(uint2(int3(dispatchThreadID).xy * int2(2, 2)) + sub));

    // 读取全分辨率深度。
    float depth = depth_input_tex.Load(int3(px, 0)).x;
    // 读取模板值,保留低 3 位(1 或 2 表示水体)。
    uint stencil = stencil_tex.Load(int3(px, 0)).x & 7u;
    bool is_water;
    if (!(1u == stencil))
    {
        is_water = 2u == stencil;
    }
    else
    {
        is_water = true;
    }
    // 水体把深度最低位置 1(向远处标记),非水体清 0;后续剔除用该位区分水体。
    float out_depth = is_water ? asfloat(asuint(depth) | 1u) : asfloat(asuint(depth) & 4294967294u);
    depth_output_tex[dispatchThreadID.xy] = out_depth;

    // 解包 RGB8 法线:各通道 0..255 -> 0..1 -> *2-1 到 [-1,1],再变换到视图空间并归一化。
    uint packed = gbuffer_input_tex.Load(int3(px, 0)).x;
    float pad;
    float3 view_normal = normalize(mul(view_constants[0].world_to_view, float4((float4(float(packed & 255u) * 0.0039215688593685626983642578125f, float((packed >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((packed >> 16u) & 255u) * 0.0039215688593685626983642578125f, pad).xyz * 2.0f) - 1.0f, 0.0f)).xyz);
    // 象限符号标记:y 分量给出法线位于半分辨率块的上/下半(+1/-1),x 分量区分
    // 水平象限(右为 1,左为 0.5),供全分辨率重建时恢复法线朝向。
    float sign_flag = (hi_px_subpixels[frame_index & 3u].x == 1u ? 1.0f : 0.5f) * (hi_px_subpixels[frame_index & 3u].y == 1u ? 1.0f : (-1.0f));
    normals_output_tex[dispatchThreadID.xy] = float4(view_normal, sign_flag);
}
