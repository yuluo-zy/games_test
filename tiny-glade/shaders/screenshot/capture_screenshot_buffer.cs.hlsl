// ==RECONSTRUCTED== shaders/screenshot/capture_screenshot_buffer.cs.hlsl
// 中文说明:
//   把截图捕获到字节寻址缓冲,按 RGBA8 行存储。bin:
//   _screenshot_capture_screenshot_buffer.cs.hlsl_{sample1,sample4}v0.bin
//   入口 cs:sample1 cs:sample4,local_size [8,8,1]。sample1 读非 MSAA 源;
//   sample4 平均/重采样 Texture2DMS 源。尺寸相同时用直接拷贝(+ sample4
//   的 MSAA 平均);否则高斯降采样(capture_screenshot.inc.hlsl)。

#include "inc/common.hlsl"            // 采样器(此处未用)
#include "inc/pack_unpack.hlsl"
#include "capture_screenshot.inc.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 dst_dims;
    uint2 src_tex_dims;
    uint2 src_ms_tex_dims;
};

RWByteAddressBuffer __alias_dst_buf : register(u14, space0);

Texture2D<float4> src_tex : register(t0, space0);
Texture2DMS<float4> src_ms_tex : register(t0, space0);

// 线性 -> sRGB 逐通道(严格阈值匹配发行 codegen)。
float encode_srgb(float c)
{
    if (0.0031308f >= c) return c * 12.92f;
    return 1.055f * pow(c, 0.416666657f) - 0.055f;
}

// 打包为不透明 RGBA8(alpha 恒为 1)。
uint pack_rgba8_opaque(float3 c)
{
    float r = encode_srgb(c.x);
    float g = encode_srgb(c.y);
    float b = encode_srgb(c.z);
    return (uint(clamp(r, 0.0f, 1.0f) * 255.0f + 0.5f)
          | (uint(clamp(g, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
          | (uint(clamp(b, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
         | 0xFF000000u;
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

    uint index = dispatch_id.x + dispatch_id.y * dst_dims.x;
    __alias_dst_buf.Store<uint>(index * 4u, pack_rgba8_opaque(c));
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

    uint index = dispatch_id.x + dispatch_id.y * dst_dims.x;
    __alias_dst_buf.Store<uint>(index * 4u, pack_rgba8_opaque(c));
}
