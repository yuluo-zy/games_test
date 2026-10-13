// ==RECONSTRUCTED== shaders/screenshot/capture_screenshot_texture.cs.hlsl
// 中文说明:
//   把截图捕获到 unorm float4 RWTexture。bin:
//   _screenshot_capture_screenshot_texture.cs.hlsl_{sample1,sample4}v0.bin
//   入口 cs:sample1 cs:sample4,local_size [8,8,1]。sample1/sample4 分别处理
//   非 MSAA 与 MSAA 源;同尺寸直接拷贝 vs 高斯降采样。

#include "inc/common.hlsl"            // 采样器(此处未用)
#include "inc/color/srgb.hlsl"
#include "capture_screenshot.inc.hlsl"
#include "inc/uv.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 dst_tex_dims;
    uint2 dst_dims;
    uint2 src_tex_dims;
    uint2 src_ms_tex_dims;
};

RWTexture2D<unorm float4> dst_tex : register(u15, space0);

Texture2D<float4> src_tex : register(t0, space0);
Texture2DMS<float4> src_ms_tex : register(t0, space0);

// 线性 -> sRGB 逐通道。
float encode_srgb(float c)
{
    if (0.0031308f >= c) return c * 12.92f;
    return 1.055f * pow(c, 0.416666657f) - 0.055f;
}

float4 encode4(float3 c)
{
    return float4(encode_srgb(c.x), encode_srgb(c.y), encode_srgb(c.z), 1.0f);
}

[numthreads(8, 8, 1)]
void sample1(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (any(dispatch_id.xy >= dst_dims))
        return;

    // 同尺寸直接拷贝,否则高斯降采样。
    float3 c;
    if (all(src_tex_dims == dst_dims))
        c = capture_direct(src_tex, dispatch_id.xy);
    else
        c = capture_gaussian(src_tex, src_tex_dims, dst_dims, dispatch_id.xy);

    dst_tex[dispatch_id.xy] = encode4(c);
}

[numthreads(8, 8, 1)]
void sample4(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (any(dispatch_id.xy >= dst_dims))
        return;

    // MSAA 源:同尺寸取 4 样本平均;否则 MSAA 高斯降采样。
    float3 c;
    if (all(src_ms_tex_dims == dst_dims))
    {
        float3 acc = 0.0f;
        for (int i = 0; i < 4; ++i)
            acc += src_ms_tex.Load(dispatch_id.xy, i).xyz;
        c = clamp(acc * 0.25f, 0.0f, 1.0f);
    }
    else
        c = capture_gaussian_msaa(src_ms_tex, src_ms_tex_dims, dst_dims, dispatch_id.xy);

    dst_tex[dispatch_id.xy] = encode4(c);
}
