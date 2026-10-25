// ==RECONSTRUCTED== shaders/calculate_reprojection_map.cs.hlsl | F-D
// Evidence: _calculate_reprojection_map.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: sampler_nnc s2, $Globals b13 (depth_tex_dims/prev_depth_tex_dims/
//   history_valid/velocity_tex_dims/output_tex_dims), depth_tex t14, prev_depth_tex t15,
//   velocity_tex t16, output_tex u17 (snorm float4), frame_constants set1/b0.
//   LocalSize [8,8,1], entry cs_main.
//   spirv-cross: reprojection map. Sky pixels (depth 0) project the clip origin to the
//   previous frame. Geometry pixels reconstruct view-space position, subtract the
//   world-space velocity (transformed to view), project to previous clip, quantize the
//   motion to fixed point. History-valid branch performs a 4-tap depth comparison with
//   GatherRed and outputs a validity mask; otherwise outputs (motion, 1, confidence).
// Status: O (structure from disassembly).
//
// === 中文说明 ===
// 重投影图计算:为时域超采样/历史重投影生成每像素的运动矢量与置信度。
//   入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
//   绑定: depth_tex t14, prev_depth_tex t15, velocity_tex t16, output_tex u17 (snorm float4),
//   $Globals b13, frame_constants set1/b0, sampler_nnc s2。
// 算法: 天空像素(深度 0)把远裁剪面原点投影到上一帧;几何像素重建视图空间位置,
//   减去世界空间速度,投影到上一帧裁剪空间并把运动量化到定点;
//   历史有效分支对 prev_depth 做 4 抽头深度比较,输出有效性掩码。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/bilinear.hlsl"
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 prev_depth_tex_dims;
    uint history_valid;
    uint2 velocity_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> prev_depth_tex : register(t15, space0);
Texture2D<float4> velocity_tex : register(t16, space0);
RWTexture2D<snorm float4> output_tex : register(u17, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 uv = (float2(int2(dispatchThreadID.xy)) + 0.5f) / float2(depth_tex_dims);

    // 天空像素(深度 == 0):把远裁剪面原点投影到上一帧。
    if (depth_tex.Load(int3(dispatchThreadID.xy, 0)).x == 0.0f)
    {
        float4 clip_pos = float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
        float4 prev_clip = mul(view_constants[0].clip_to_prev_clip, mul(view_constants[0].view_to_clip, mul(view_constants[0].clip_to_view, clip_pos)));
        float2 prev_uv = ((prev_clip.xy / prev_clip.w) * float2(0.5f, -0.5f)) + 0.5f;
        // 钳制到 [0,1];z 通道记录是否仍在屏内(用于有效性)。
        float2 clamped = clamp(prev_uv, 0.0f, 1.0f);
        output_tex[dispatchThreadID.xy] = float4(prev_uv - ((clip_pos.xy * float2(0.5f, -0.5f)) + 0.5f), float(all(clamped == prev_uv)), 1.0f);
    }
    else
    {
        // 几何像素:重建视图空间位置,计算运动矢量。
        float depth = depth_tex.Load(int3(dispatchThreadID.xy, 0)).x;
        float4 view_pos = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), depth, 1.0f));
        float w = view_pos.w;
        float view_z = view_pos.z / w;
        float4 view_pos_n = view_pos / w;

        // 世界空间速度变换到视图空间,并从当前帧位置反推上一帧位置。
        float4 vel = mul(view_constants[0].world_to_view, float4(velocity_tex.Load(int3(uint2(dispatchThreadID.xy), 0)).xyz, 0.0f));
        float3 prev_pos = view_pos_n.xyz - vel.xyz;

        // 投影到上一帧裁剪空间,并把像素运动量化为 15-bit 定点(乘 32767,四舍五入后乘 3.05e-5)。
        float4 prev_clip = mul(view_constants[0].clip_to_prev_clip, mul(view_constants[0].view_to_clip, float4(prev_pos.x, prev_pos.y, prev_pos.z, view_pos_n.w)));
        float2 motion = floor(((((prev_clip.xy / prev_clip.w) * float2(0.5f, -0.5f)) + 0.5f) - uv) * 32767.0f + 0.5f) * 3.0518509447574615478515625e-05f;

        // 历史是否可用:非零 history_valid 且非首帧。
        bool use_history;
        if (history_valid != 0u)
        {
            use_history = !(0u == frame_index);
        }
        else
        {
            use_history = false;
        }

        float validity;
        float confidence;
        if (use_history)
        {
            // 用上一帧深度做 4 抽头比较,判断运动是否被遮挡(产生有效性/置信度)。
            float4 prev_view_pos = mul(view_constants[0].prev_clip_to_prev_view, prev_clip);
            float2 out_dims = float2(output_tex_dims);
            float2 tap_pos = trunc(((uv + motion) + ((view_constants[0].sample_offset_clip - view_constants[0].prev_sample_offset_clip) * 0.5f) * float2(1.0f, -1.0f)) * out_dims - 0.5f);

            // GatherRed 取上一帧深度 4 个纹素,转成视图深度差。
            float4 prev_depth_gathered = (1.0f / (prev_depth_tex.GatherRed(sampler_nnc, (tap_pos + 1.0f) / out_dims).wzxy * (-view_constants[0].prev_clip_to_prev_view[3].z))) - (prev_view_pos / prev_view_pos.w).z;
            float4 neg = -prev_depth_gathered;

            // 深度差阈值检查:过大的正/负差都判定为遮挡,得到 reject 掩码。
            float view_dir_z = -normalize(view_pos.xyz / view_pos.w).z;
            float4 in_range = step(abs(prev_depth_gathered * min(-0.20000000298023223876953125f, 1.0f)), ((view_z * (-0.004999999888241291046142578125f)) / view_dir_z));
            float4 threshold = ((view_z * (-0.0003000000142492353916168212890625f)) / view_dir_z);
            float4 reject_neg = step(neg, threshold);
            float4 reject_pos = step(-neg, threshold);

            // 4 个采样点是否在输出边界内。
            int2 tap = int2(tap_pos);
            bool in_bounds_00 = all(tap >= int2(0, 0)) && all(uint2(tap) < output_tex_dims);
            int2 tap_x = tap + int2(1, 0);
            bool in_bounds_10 = all(tap_x >= int2(0, 0)) && all(uint2(tap_x) < output_tex_dims);
            int2 tap_y = tap + int2(0, 1);
            bool in_bounds_01 = all(tap_y >= int2(0, 0)) && all(uint2(tap_y) < output_tex_dims);
            int2 tap_xy = tap + int2(1, 1);
            bool in_bounds_11 = all(tap_xy >= int2(0, 0)) && all(uint2(tap_xy) < output_tex_dims);

            // 有效性 = 是否被遮挡(正负任一 reject);置信度 = 深度在范围内的采样数加权。
            bool rejected = (int(any(reject_pos != 0.0f)) > 0) ? (int(any(reject_neg != 0.0f)) > 0) : false;
            validity = float(rejected);
            confidence = dot(float4(in_range.x * float(in_bounds_00), in_range.y * float(in_bounds_10), in_range.z * float(in_bounds_01), in_range.w * float(in_bounds_11)), float4(1.0f, 2.0f, 4.0f, 8.0f)) * 0.066666670143604278564453125f;
        }
        else
        {
            validity = 1.0f;
            confidence = 1.0f;
        }

        // 输出: (运动矢量, 置信度, 有效性残差 + 速度长度贡献)。
        output_tex[dispatchThreadID.xy] = float4(motion, confidence, (1.0f - validity) + length((vel.xz * (((-0.5f) / view_z) * view_constants[0].view_to_clip[1].y)) * float2(view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x, 1.0f)));
    }
}
