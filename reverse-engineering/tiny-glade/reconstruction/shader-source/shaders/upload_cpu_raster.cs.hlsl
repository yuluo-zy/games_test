// ==RECONSTRUCTED== shaders/upload_cpu_raster.cs.hlsl | evidence: spirv-cross of _upload_cpu_raster.cs.hlsl_{r16f,rgba16f}_0.bin (two entries; 64x64-tile RGBA8 CPU raster upload, uint word = 4 packed pixels; UAV formats R16f / Rgba16f inferred from OpTypeImage) | status: O (bindings/entry/LocalSize/tile decode); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/pack_unpack.hlsl"

// CPU rasterized terrain data. Each uint word packs 4 RGBA8 texels; the buffer
// is laid out as a 64x64 grid of 64x64-pixel tiles.
ByteAddressBuffer cpu_raster : register(t14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint output_tex_width;
};

// Upload the packed RGBA8 CPU raster into an R16F texture (red channel only).
RWTexture2D<half> output_tex_r16f : register(u0, space0);

// Upload the packed RGBA8 CPU raster into an RGBA16F texture.
[[vk::ext_capability(49)]]
RWTexture2D<half4> output_tex_rgba16f : register(u0, space0);

float4 decode_rgba8(uint word)
{
    return float4(
        float(word & 0xffu) * (1.0f / 255.0f),
        float((word >> 8u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 16u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 24u) & 0xffu) * (1.0f / 255.0f));
}

// Compute the linear slot index (in words) of the 4-pixel group containing
// `pixel`, then load that word. `pixel.x` is a multiple of 4; the byte offset
// passed to Load<uint> is the word index itself.
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
    uint2 start_pixel = dispatch_thread_id.xy * uint2(4u, 1u);
    float4 pixel = decode_rgba8(load_pixel_word(start_pixel));
    for (uint i = 0u; i < 4u; i++)
    {
        output_tex_rgba16f[start_pixel + uint2(i, 0u)] = float4(pixel[i], 0.0f, 0.0f, 0.0f);
    }
}
