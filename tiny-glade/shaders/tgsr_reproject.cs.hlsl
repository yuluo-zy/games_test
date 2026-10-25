// ==RECONSTRUCTED== shaders/tgsr_reproject.cs.hlsl | A (dof-aa review, 2026-10-13): A 方案胜出(w1-含-w2 的 bicubic 形式,匹配 taa/tgsr 系列;更精简的 include 闭包;rgb_to_ycbcr 来自 inc/color/ycbcr.hlsl)。
//   TGSR 重投影:取 2x2 运动四边形,在遮挡解除时重搜最大深度位置,用 5 抽头
//   bicubic 十字滤波器过滤历史,并写出 YCoCg 亮度/色度。

#include "inc/samplers.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/uv.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/history_valid.hlsl"
#include "tgsr_common.inc.hlsl"

// 全局常量($Globals):有效性标志与各纹理尺寸。
cbuffer $Globals : register(b13, space0)
{
    uint history_valid;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 depth_tex_dims;
    uint2 output_lum_tex_dims;
    uint2 output_chrom_tex_dims;
};

Texture2D<float4> history_tex : register(t14, space0);
Texture2D<float4> reprojection_tex : register(t15, space0);
Texture2D<float4> depth_tex : register(t16, space0);
RWTexture2D<half> output_lum_tex : register(u17, space0);
RWTexture2D<half2> output_chrom_tex : register(u18, space0);

// TGSR 重投影入口。
[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    // 深度纹理与输出纹理的尺寸。
    float2 depth_dims = float2(depth_tex_dims);
    float2 out_dims = float2(output_lum_tex_dims);
    // 从输出像素反推对应的 2x2 运动四边形角(四分之一分辨率)。
    uint2 quad_px = uint2((float2(dispatch_id.xy) + 0.5f) * (depth_dims / out_dims));

    // 读取 2x2 四角的运动向量。
    float2 r00 = reprojection_tex.Load(int3(quad_px + uint2(0xFFFFFFFFu, 0xFFFFFFFFu), 0)).xy;
    float2 r10 = reprojection_tex.Load(int3(quad_px + uint2(1u, 0xFFFFFFFFu), 0)).xy;
    float2 r01 = reprojection_tex.Load(int3(quad_px + uint2(0xFFFFFFFFu, 1u), 0)).xy;
    float2 r11 = reprojection_tex.Load(int3(quad_px + uint2(1u, 1u), 0)).xy;
    // 运动向量的包围盒与跨度。
    float2 r_min = min(min(r00, r10), min(r01, r11));
    float2 r_max = max(max(r00, r10), max(r01, r11));
    float2 r_delta = r_max - r_min;
    // 阈值:按深度纹理尺寸与运动向量量级取 10%。
    float2 r_threshold = max(1.0f / depth_dims, abs(r_max + r_min)) * 0.1f;
    // 四个角的运动向量是否彼此不一致(可能发生遮挡解除)。
    bool inconsistent = any(r_delta > r_threshold);
    // 与同一 2x2 块的相邻通道横向合并(异或 2 = 水平邻居)。
    bool quad_inconsistent = inconsistent
        | WaveReadLaneAt(inconsistent, WaveGetLaneIndex() ^ 2u);

    // 重搜位置:若该 2x2 块(或再往上一行的块)不一致,在 3x3 邻域内找最大深度
    // 的像素作为重投影锚点,以处理遮挡解除。
    uint2 search_px;
    if ((quad_inconsistent | WaveReadLaneAt(quad_inconsistent, WaveGetLaneIndex() ^ 16u)) != 0)
    {
        // 3x3 邻域最大深度搜索。
        float max_depth = depth_tex.Load(int3(quad_px, 0)).x;
        uint2 max_depth_px = quad_px;
        [loop]
        for (int dy = -1; dy <= 1; ++dy)
        {
            [loop]
            for (int dx = -1; dx <= 1; ++dx)
            {
                uint2 sample_px = quad_px + uint2(dx, dy);
                float d = depth_tex.Load(int3(sample_px, 0)).x;
                // 深度更大 = 更靠近相机(刚露出的前景),取为锚点。
                if (d > max_depth)
                {
                    max_depth = d;
                    max_depth_px = sample_px;
                }
            }
        }
        search_px = max_depth_px;
    }
    else
    {
        search_px = quad_px;
    }

    // 重投影采样坐标:当前像素中心 + 锚点处的运动向量,换算回输出像素。
    float2 reproj_px = (((float2(int2(dispatch_id.xy)) + 0.5f) / out_dims)
        + reprojection_tex.Load(int3(search_px, 0)).xy) * out_dims;

    // 5 抽头 bicubic 十字历史滤波器(分离式权重计算)。
    // 基本格点与格内偏移。
    float2 base = floor(reproj_px - 0.5f);
    float2 base_c = base + 0.5f;
    float2 frac = reproj_px - base_c;
    // Mitchel/三次样条权重分解。
    float2 h2 = frac * 0.5f;
    float2 w0 = frac * ((-0.5f) + (frac * (1.0f - h2)));
    float2 frac2 = frac * frac;
    float2 frac15 = frac * 1.5f;
    float2 w2 = frac * (0.5f + (frac * (2.0f - frac15)));
    float2 w3 = frac2 * ((-0.5f) + h2);
    float2 w1 = (1.0f + (frac2 * ((-2.5f) + frac15))) + w2;
    // 左右/中心的采样 UV(取 5 个采样点:左、上、中、右、下)。
    float2 left_uv = (base + (-0.5f)) / out_dims;
    float2 right_uv = (base + 2.5f) / out_dims;
    float2 center_uv = (base_c + (w2 / w1)) / out_dims;

    // 5 抽头十字加权平均(中 + 上下左右)。
    float3 hist = (
        (history_tex.SampleLevel(sampler_llc, float2(center_uv.x, left_uv.y), 0.0f).xyz * w1.x * w0.y)
        + (history_tex.SampleLevel(sampler_llc, float2(left_uv.x, center_uv.y), 0.0f).xyz * w0.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, center_uv, 0.0f).xyz * w1.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, float2(right_uv.x, center_uv.y), 0.0f).xyz * w3.x * w1.y)
        + (history_tex.SampleLevel(sampler_llc, float2(center_uv.x, right_uv.y), 0.0f).xyz * w1.x * w3.y)
    ) / ((((w1.x * w0.y) + (w0.x * w1.y)) + (w1.x * w1.y)) + ((w3.x * w1.y) + (w1.x * w3.y)));

    // 历史帧有效且非首帧时才使用历史,否则乘 0 归零。
    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    // 转成 YCoCg 颜色空间。
    float3 ycbcr = rgb_to_ycbcr(hist * float(history_available));

    // 亮度与色度分开写出。
    output_lum_tex[dispatch_id.xy] = ycbcr.x;
    output_chrom_tex[dispatch_id.xy] = ycbcr.yz;
}
