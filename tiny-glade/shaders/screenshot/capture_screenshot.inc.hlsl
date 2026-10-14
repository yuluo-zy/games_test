// ==RECONSTRUCTED== shaders/screenshot/capture_screenshot.inc.hlsl
// 中文说明:
//   截图捕获共享辅助函数。证据来自 _screenshot_capture_screenshot_{buffer,texture}
//   .cs.hlsl_{sample1,sample4} bin:
//   - 高斯加权箱式降采样(按纵横比校正源中心,半径 clamp(1..3),
//     权重 exp(-3*d^2/sigma^2));
//   - 同尺寸直接拷贝路径。
//   | 状态:O(降采样算法/常量来自内联 SPIR-V);I(辅助函数名/签名——被内联)

#ifndef TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL
#define TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL

// 直接拷贝路径:读一个源纹素。
float3 capture_direct(Texture2D<float4> tex, uint2 coord)
{
    return clamp(tex.Load(int3(coord, 0)).xyz, 0.0f, 1.0f);
}

// 把 tex 高斯加权箱式降采样到 dst_dims,写回 dst_pixel 处。
float3 capture_gaussian(Texture2D<float4> tex, uint2 src_dims, uint2 dst_dims, uint2 dst_pixel)
{
    float2 srcf = float2(src_dims);
    float2 dstf = float2(dst_dims);

    // 纵横比校正:统一按短边归一,取两轴缩放较小者。
    float2 src_aspect = srcf / float(min(src_dims.x, src_dims.y));
    float2 dst_aspect = dstf / float(min(dst_dims.x, dst_dims.y));
    float2 ratio = src_aspect / dst_aspect;
    float2 scale = min(ratio.x, ratio.y).xx;

    // 采样半径随缩放比换算到源空间,clamp 到 [1,3]。
    float2 sigma_scale = (((3.0f / dstf) * dst_aspect) * scale / src_aspect) * srcf;
    float radius = clamp(max(sigma_scale.x, sigma_scale.y), 1.0f, 3.0f);

    // 目标像素中心映射回源空间中心。
    float2 center = (((float2(int2(dst_pixel)) + 0.5f) / dstf - 0.5f) * dst_aspect) * scale / src_aspect + 0.5f;
    center *= srcf;

    // 以中心为中心的半径内箱式收集。
    int2 lo = int2((center - 0.5f) - radius);
    int2 hi = int2((center + 0.5f) + radius);

    float4 acc = 0.0f;
    for (int y = lo.y; y < hi.y; ++y)
    {
        for (int x = lo.x; x < hi.x; ++x)
        {
            int2 p = int2(x, y);
            float2 d = (float2(p) + 0.5f) - center;
            float w = exp((-3.0f * dot(d, d)) / (radius * radius));
            acc += float4(tex.Load(int3(p, 0)).xyz, 1.0f) * w * float(all(uint2(p) < src_dims));
        }
    }

    // 归一化:加权平均后夹紧到 [0,1]。
    return clamp(acc.xyz / max(9.9999997e-6f, acc.w), 0.0f, 1.0f);
}

// MSAA 变体:同样的降采样,但读取 Texture2DMS 的 sample 0。
float3 capture_gaussian_msaa(Texture2DMS<float4> tex, uint2 src_dims, uint2 dst_dims, uint2 dst_pixel)
{
    float2 srcf = float2(src_dims);
    float2 dstf = float2(dst_dims);

    float2 src_aspect = srcf / float(min(src_dims.x, src_dims.y));
    float2 dst_aspect = dstf / float(min(dst_dims.x, dst_dims.y));
    float2 ratio = src_aspect / dst_aspect;
    float2 scale = min(ratio.x, ratio.y).xx;

    float2 sigma_scale = (((3.0f / dstf) * dst_aspect) * scale / src_aspect) * srcf;
    float radius = clamp(max(sigma_scale.x, sigma_scale.y), 1.0f, 3.0f);

    float2 center = (((float2(int2(dst_pixel)) + 0.5f) / dstf - 0.5f) * dst_aspect) * scale / src_aspect + 0.5f;
    center *= srcf;

    int2 lo = int2((center - 0.5f) - radius);
    int2 hi = int2((center + 0.5f) + radius);

    float4 acc = 0.0f;
    for (int y = lo.y; y < hi.y; ++y)
    {
        for (int x = lo.x; x < hi.x; ++x)
        {
            int2 p = int2(x, y);
            float2 d = (float2(p) + 0.5f) - center;
            float w = exp((-3.0f * dot(d, d)) / (radius * radius));
            acc += float4(tex.Load(uint2(p), 0u).xyz, 1.0f) * w * float(all(uint2(p) < src_dims));
        }
    }

    return clamp(acc.xyz / max(9.9999997e-6f, acc.w), 0.0f, 1.0f);
}

#endif // TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL
