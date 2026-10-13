// ==RECONSTRUCTED== shaders/upload_cpu_raster.cs.hlsl | evidence: spirv-cross of _upload_cpu_raster.cs.hlsl_{r16f,rgba16f}v0.bin (two entries; 64x64-tile RGBA8 CPU raster upload, uint word = 4 packed pixels; UAV formats R16f / Rgba16f inferred from OpTypeImage) | status: O (bindings/entry/LocalSize/tile decode); I (names)
//
// 中文说明:把 CPU 侧光栅化生成的 RGBA8 数据上传到 GPU 纹理。
//   数据布局:每个 uint 打包 4 个 RGBA8 像素;缓冲按 64x64 像素的方块平铺组织。
//   两个入口分别把数据写入 R16F(只取红通道)或 RGBA16F 纹理。

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/pack_unpack.hlsl"

// CPU 光栅化地形数据。每个 uint 打包 4 个 RGBA8 纹素;缓冲按 64x64 像素方块平铺。
ByteAddressBuffer cpu_raster : register(t14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint output_tex_width;
};

// 上传目标:R16F 纹理(仅红通道)。
RWTexture2D<half> output_tex_r16f : register(u0, space0);

// 上传目标:RGBA16F 纹理。
[[vk::ext_capability(49)]]
RWTexture2D<half4> output_tex_rgba16f : register(u0, space0);

// 解包一个 uint 里的 4 个 RGBA8 像素,转成 [0,1] 的 float4。
float4 decode_rgba8(uint word)
{
    return float4(
        float(word & 0xffu) * (1.0f / 255.0f),
        float((word >> 8u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 16u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 24u) & 0xffu) * (1.0f / 255.0f));
}

// 计算包含 `pixel` 的 4 像素组的线性字槽索引并加载该 uint。
// `pixel.x` 是 4 的倍数;传给 Load<uint> 的字节偏移即字槽本身。
uint load_pixel_word(uint2 pixel)
{
    uint tile_width = output_tex_width / 64u;
    uint2 tile = pixel / uint2(tile_width, tile_width);
    uint2 in_tile = pixel % uint2(tile_width, tile_width);
    uint slot = (((64u * tile.y) + tile.x) * (tile_width * tile_width))
                + (in_tile.y * tile_width) + in_tile.x;
    return cpu_raster.Load<uint>(slot);
}

[numthreads(8, 8, 1)]
void r16f(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 每个线程处理一个 4x1 像素组,把解包后的对应分量写入 R16F。
    uint2 start_pixel = dispatch_thread_id.xy * uint2(4u, 1u);
    float4 pixel = decode_rgba8(load_pixel_word(start_pixel));
    for (uint i = 0u; i < 4u; i++)
    {
        output_tex_r16f[start_pixel + uint2(i, 0u)] = pixel[i];
    }
}

[numthreads(8, 8, 1)]
void rgba16f(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 与 r16f 相同,但把分量扩展到 RGBA16F(其余通道为 0)。
    uint2 start_pixel = dispatch_thread_id.xy * uint2(4u, 1u);
    float4 pixel = decode_rgba8(load_pixel_word(start_pixel));
    for (uint i = 0u; i < 4u; i++)
    {
        output_tex_rgba16f[start_pixel + uint2(i, 0u)] = float4(pixel[i], 0.0f, 0.0f, 0.0f);
    }
}
