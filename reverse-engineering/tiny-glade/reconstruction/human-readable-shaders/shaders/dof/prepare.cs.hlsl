// ==RECONSTRUCTED== shaders/dof/prepare.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (read-only StructuredBuffer<DofFocus>, member-wise reads, no OpMemberDecorate drift) + shared format fix: coc_out_tex RWTexture2D<half> (R16f), tile_output_tex RWTexture2D<half2> (Rg16f).
//   Computes the per-pixel signed circle-of-confusion from focus depth and writes the tile
//   min/max abs-CoC used by the analyze pass.
//
// === 中文说明 ===
// 景深准备 pass:由对焦距离计算每个像素的带符号弥散圆(CoC),并把每个 8x8 tile
//   的 |CoC| 最小/最大值写出给 analyze pass 使用。
// 入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
// 绑定: depth_tex t14, autofocus_buf t15 (StructuredBuffer<DofFocus>),
//   coc_out_tex u16 (RWTexture2D<half> R16f), tile_output_tex u17 (RWTexture2D<half2> Rg16f),
//   $Globals b13, groupshared min/max_abs_coc。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 coc_out_tex_dims;
    uint2 tile_output_tex_dims;
};

StructuredBuffer<DofFocus> autofocus_buf : register(t15, space0);
Texture2D<float4> depth_tex : register(t14, space0);
RWTexture2D<half> coc_out_tex : register(u16, space0);
RWTexture2D<half2> tile_output_tex : register(u17, space0);

groupshared uint min_abs_coc_asuint;
groupshared uint max_abs_coc_asuint;

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    // 组内初始化 tile 的 |CoC| 极值(最小从很大的初值开始)。
    if (local_index == 0u)
    {
        min_abs_coc_asuint = 1343554297u;
        max_abs_coc_asuint = 0u;
    }
    GroupMemoryBarrierWithGroupSync();

    // 重建视图空间位置(通过深度)。
    float4 view_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
        float4((((float2(int2(dispatch_id.xy)) + 0.5f) / float2(coc_out_tex_dims)) - 0.5f) * float2(2.0f, -2.0f),
               depth_tex.Load(int3(dispatch_id.xy, 0)).x, 1.0f)));
    float3 view_pos = view_world.xyz / view_world.w;

    // 对焦参数:由 autofocus 缓冲提供远近对焦距离(取倒数做线性插值)。
    float focus_scale = autofocus_buf[camera_idx].focus_scale;
    float inv_focus0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_focus1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float focus_max = max(0.001f, max(inv_focus0, inv_focus1)) + 9.9999997473787516e-06f;
    float focus_min = max(0.001f, min(inv_focus0, inv_focus1)) - 9.9999997473787516e-06f;
    // 移轴模式:把 y 分量清零,使对焦平面垂直。
    bool use_tilt_shift = autofocus_buf[camera_idx].use_tilt_shift != 0u;
    view_pos.y *= (use_tilt_shift ? 0.0f : 1.0f);

    // 相机位置与视线方向,求沿视线的距离倒数 inv_dist。
    float4 cam_h = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w;
    float3 cam_dir = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float inv_dist = 1.0f / max(1.000000013351432e-10f, dot(view_pos - cam_pos, cam_dir));
    float focus_range = focus_max - focus_min;
    float epsilon = min(0.01f, 0.05f * focus_range);

    // 带符号距离:超出远焦点为正(景深向后偏),近于近焦点为负(向前偏),之间线性渐变。
    float signed_dist;
    if (inv_dist > focus_max)
    {
        signed_dist = (inv_dist - focus_max) + epsilon;
    }
    else if (inv_dist < focus_min)
    {
        signed_dist = (inv_dist - focus_min) - epsilon;
    }
    else
    {
        signed_dist = -epsilon + ((2.0f * epsilon) * (inv_dist - focus_min)) / focus_range;
    }

    // 移轴模式下按视线水平长度归一化距离(倾斜对焦平面)。
    float coc_dir_dist;
    if (use_tilt_shift)
    {
        coc_dir_dist = signed_dist / max(0.01f, length(cam_dir.xz));
    }
    else
    {
        coc_dir_dist = signed_dist;
    }

    // CoC 像素值 = 高度 * 1/720 * (-距离) * focus_scale * 投影 y 缩放。
    float coc_signed = (float(depth_tex_dims.y) * 0.0013888889225199819f)
        * ((-coc_dir_dist) * focus_scale * view_constants[0].view_to_clip[1].y);
    // |CoC| 钳制到 高度*1/24,并把 bit 模式作为整数值参与组内原子极值。
    uint coc_abs = asuint(min(float(depth_tex_dims.y) * 0.041666667908430099f, abs(coc_signed)));

    // 组内归约 tile 的 |CoC| 最小/最大值。
    InterlockedMin(min_abs_coc_asuint, coc_abs);
    InterlockedMax(max_abs_coc_asuint, coc_abs);

    GroupMemoryBarrierWithGroupSync();
    // 第一个线程把极值写进 tile 级输出(按 8x8 tile 索引)。
    if (local_index == 0u)
    {
        tile_output_tex[dispatch_id.xy / uint2(8u, 8u)] =
            float2(asfloat(min_abs_coc_asuint), asfloat(max_abs_coc_asuint));
    }

    // 写每个像素的带符号 CoC。
    coc_out_tex[dispatch_id.xy] = coc_signed;
}
