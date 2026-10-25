// GI ReSTIR 通道的 reservoir 打包工具。
// GI ReSTIR 链在每个半分辨率纹素存一个 uint2 reservoir:
//   x = 打包的 payload 像素 id(px | py<<16),y = 两个半精度权重。
// 无偏权重 W 以 y 的高 16 位读回(eval 读 restir_reservoir_tex.y >> 16)。

#ifndef TINYGLADE_GI_RESTIR_INC_HLSL
#define TINYGLADE_GI_RESTIR_INC_HLSL

// 把像素 id 打包进 32 位 payload(低 16 位 x,高 16 位 y)。
uint pack_restir_payload(uint2 pixel)
{
    return pixel.x | (pixel.y << 16u);
}

// 解包 payload 的像素 id。
uint2 unpack_restir_payload(uint payload)
{
    return uint2(payload & 0xFFFFu, payload >> 16u);
}

// 把两个 float 权重打包进一个 uint(两个半精度槽位)。
uint pack_restir_weights(float2 w)
{
    return f32tof16(w.x) | (f32tof16(w.y) << 16u);
}

// 解包两个半精度权重。
float2 unpack_restir_weights(uint packed)
{
    return float2(f16tof32(packed & 0xFFFFu), f16tof32(packed >> 16u));
}

#endif // TINYGLADE_GI_RESTIR_INC_HLSL
