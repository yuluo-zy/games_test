// ==RECONSTRUCTED== shaders/dof/fused.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (OpExtInst/ImageFetch/Phi parity, no 8/11 guard, R11fG11fB10f + [[vk::ext_capability(49)]] gate). blue_noise_texture declared locally (main inc/blue_noise.hlsl no longer declares the global).
//   Fused full-res bokeh DoF: same ring gather as halfres but without tile indirection, using
//   pixel-space Loads (21-tap ring, 6 bins, 6-tap disk scatter).
//
// === 中文说明 ===
// 融合的全分辨率散景景深(DoF):与 halfres 相同的环形采集,但没有 tile 间接寻址,
//   使用像素空间 Load(21 抽头环、6 个深度分箱、6 抽头散景圆盘散射)。
// 入口 cs_main,工作组 8x8x1,无 TGV_ 变体。
// 绑定: blue_noise_texture t14, color_tex t15, coc_tex t17,
//   img_output u18 (RWTexture2D<float3> R11fG11fB10f, [[vk::ext_capability(49)]]),
//   $Globals b13, groupshared dof_ring_samples[64][11]。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/hash.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/morton.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tex : register(t17, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> img_output : register(u18, space0);

groupshared uint dof_ring_samples[64][11];

// 把两个 half 打包进一个 uint(f16x2),环采样表按此存储 CoC 对。
uint PackRingPair(float2 v)
{
    return f32tof16(v.x) | (f32tof16(v.y) << 16u);
}

// 从打包的 uint 解出两个 half。
float2 UnpackRingPair(uint v)
{
    return float2(f16tof32(v & 0xFFFFu), f16tof32(v >> 16u));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    float2 pixel_center = float2(dispatch_id.xy) + 0.5f;
    float2 color_dims = float2(color_tex_dims);
    float2 uv = pixel_center / color_dims;
    // 最大 CoC 限幅 = 高度 * 1/24。
    float coc_limit = float(color_tex_dims.y) * 0.041666667908430099f;
    uint2 pixel = uint2(pixel_center);
    // 蓝噪声抖动:按帧号旋转纹素偏移,得到随机相位/旋转。
    float4 noise = ((blue_noise_texture.Load(int3((pixel + uint2(13u, 13u)
        + uint2(int(frame_index * 59u), int(frame_index * 37u))) & uint2(255u, 255u), 0)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
    float ring_offset = noise.y;
    // CoC 环间距 = 高度 * 1/240。
    float coc_step = float(color_tex_dims.y) * 0.0041666668839752674f;
    float phase = noise.z;
    // 相位决定辐条方向翻转(镜像散景)。
    bool flip = phase < 0.5f;
    float mask_phase = flip ? (1.0f - phase) : phase;
    float spoke_sign = flip ? -1.0f : 1.0f;

    // 颜色/CoC/权重累加器。
    float4 color_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float coc_acc = 0.0f;
    float coc_weight = 0.0f;
    float weight_acc = 0.0f;

    // 两个辐条方向(正交)分别做环形采集。
    [loop]
    for (uint spoke = 0u; spoke < 2u; ++spoke)
    {
        float angle = (float(spoke) + noise.x) * 1.5707963705062866f;
        float2 dir = float2(cos(angle), sin(angle));
        // 计算辐条方向在像素空间的方向,并按散景各向异性缩放(旋转矩阵 a / 1-a)。
        float4 v = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
        float2 p = v.xy / v.z;
        p.y = -p.y;
        float2 n = normalize(p);
        float2x2 rot = float2x2(n, float2(-n.y, n.x));
        float a = lerp(0.699999988079071f, 1.0f,
            exp((-debug_tweakers.z * bokeh_anisotropy) * dot(p, p)));
        float2 dir_px = mul(transpose(rot), mul(rot, dir) * float2(a, 1.0f / a)) * spoke_sign;

        {
            // 沿辐条 21 个 CoC 采样,交错打包进 groupshared 环形表。
            float pending = 0.0f;
            float lerp_factor = noise.x * 0.2f;
            [unroll]
            for (uint k = 0u; k < 21u; ++k)
            {
                float coc = clamp(coc_tex.SampleLevel(sampler_nnc,
                    (float2(pixel) + dir_px * ((float(int(k - 10u)) + ring_offset) * coc_step)) / color_dims,
                    0.0f).x, -coc_limit, coc_limit);
                float coc_abs = abs(coc);
                // 追踪环上最大绝对 CoC(带 lerp 平滑)。
                coc_acc = lerp(coc_acc, max(coc_acc, coc_abs), lerp_factor);
                if ((k & 1u) == 0u)
                {
                    pending = coc;
                }
                else
                {
                    // 成对打包 (pending, coc) 到环形表。
                    dof_ring_samples[local_index][k >> 1u] =
                        PackRingPair(float2(pending, 0.0f)) | (PackRingPair(float2(coc, 0.0f)) << 16u);
                }
            }
            // 最后一对只写低 half。
            dof_ring_samples[local_index][10u] = PackRingPair(float2(pending, 0.0f));
        }

        // 按 CoC 分箱:对每个辐条采样,判断它落入 6 个深度分箱的哪些。
        uint mask[6];
        [unroll]
        for (uint i = 0u; i < 6u; ++i)
        {
            mask[i] = 0u;
        }
        [loop]
        for (uint j = 0u; j < 21u; ++j)
        {
            uint lane_bit = j & 1u;
            uint pair = j >> 1u;
            float2 pair_coc = UnpackRingPair(dof_ring_samples[local_index][pair]);
            float ring_dist = (float(int(j - 10u)) + ring_offset) * coc_step;
            [unroll]
            for (uint b = 0u; b < 6u; ++b)
            {
                // 分箱边界:frac(b+phase)*1/3 - 1。CoC 距离超过边界则置位。
                if ((ring_dist / (((float(b) + mask_phase) * 0.3333333432674408f) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[b] |= (1u << (j & 31u));
                }
            }
        }

        // 对每个非空分箱,用 firstbit 找到穿越边界的位置,插值出精确的 CoC 位置并采样颜色。
        float4 spoke_color = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint bin = 0u; bin < 6u; ++bin)
        {
            float mask_dist = ((float(bin) + mask_phase) * 0.3333333432674408f) - 1.0f;
            if (mask[bin] == 0u)
            {
                continue;
            }
            // 正/负分箱方向,找第一个穿越采样(低 bit 或高 bit)。
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                crossing = firstbitlow(mask[bin]);
                uint prev = uint(max(0, (int(crossing) - 1)));
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][prev >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[prev & 1u];
            }
            else
            {
                crossing = firstbithigh(mask[bin]);
                uint next = min(20u, crossing + 1u);
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][next >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[next & 1u];
            }
            float ring_dist = (float(int(crossing - 10u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : -1.0f);
            float candidate = ring_dist - step;
            // 交叉点插值权重,得到亚像素级 CoC 边界位置 coc_pos。
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs(mask_dist) * float(mask[bin] != 0u);
            float denom = cross_weight - (coc_a - dist_ratio);
            float coc_pos = lerp(candidate, ring_dist, clamp(cross_weight / denom, 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.Load(int3(uint2(int2(float2(pixel) + dir_px * coc_pos)), 0)).x,
                -coc_limit, coc_limit);
            // near 判定:CoC 位置与实测值接近。
            float epsilon = cross_weight * 0.1f;
            bool near = false;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            float add_weight;
            float add_color_weight;
            // No 8/11 near-branch guard for fused (target).
            if (near)
            {
                add_weight = weight_acc;
                add_color_weight = sample_weight;
            }
            else
            {
                // 远处/近处的平滑过渡:大 CoC(远)偏向权重,小 CoC(近)偏向颜色。
                float s = smoothstep(0.0f, 30.0f, coc_b)
                    * smoothstep(float(color_tex_dims.y) * 0.008333333767950535f, 0.0f, coc_a);
                add_weight = weight_acc + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            weight_acc = add_weight;
            // 在 CoC 边界位置采样颜色,贡献给本辐条。
            spoke_color += color_tex.SampleLevel(sampler_llc,
                (floor(float2(pixel) + dir_px * (near ? coc_pos : candidate)) + 0.5f) / color_dims,
                log2(max(0.0f, (abs(coc_a) - denom) * 0.25f))) * add_color_weight;
        }
        color_acc += spoke_color;
    }

    // 散景圆盘散射:按颜色 alpha 权重散射,用 6 个噪声圆盘点把前景亮色铺开。
    float scatter = weight_acc * smoothstep(0.01f * color_acc.w,
        max(9.9999997473787516e-06f, 0.2f * color_acc.w), weight_acc);
    if (scatter > 0.0f)
    {
        float min_coc = -coc_limit;
        float4 disk_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint d = 0u; d < 6u; ++d)
        {
            float2 disc = frac(frac((float2(0.7548776865005493f, 0.5698403120040894f) * float(d)) + 0.5f) + noise.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.2831854820251465f;
            uint2 sample_px = uint2(int2(float2(pixel) + (float2(cos(theta) * r, sin(theta) * r) * coc_acc)));
            float4 c = coc_tex.Load(int3(sample_px, 0));
            if (c.x > (min_coc - 10.0f))
            {
                disk_acc += color_tex.Load(int3(sample_px, 0));
                min_coc = max(min_coc, c.x);
            }
        }
        color_acc += (disk_acc / max(1.0f, disk_acc.w)) * scatter;
    }

    // 按颜色权重归一化输出。
    img_output[dispatch_id.xy] = max(float3(0.0f, 0.0f, 0.0f), color_acc.xyz / max(9.9999997473787516e-06f, color_acc.w));
}
