// ======================================================================
// inc/pack_unpack.hlsl —— 8 位颜色/法线打包解包公共头文件
// ======================================================================
// 用途:gbuffer / 烘焙颜色缓冲的颜色打包解包。
// 来源:_apply_screen_shadows cs 内联法线解码(p&0xff,(p>>8)&0xff,(p>>16)&0xff
//      然后 /255.0*2-1);_billboard_forest ps 内联 RGBA8 打包
//      (uint(clamp(c,0,1)*255.0+0.5) | <<8 | <<16,alpha 0xFF<<24)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/pack_unpack.hlsl | evidence: _apply_screen_shadows cs inlined normal decode (p&0xff,(p>>8)&0xff,(p>>16)&0xff then /255.0*2-1); _billboard_forest ps inlined RGBA8 pack (uint(clamp(c,0,1)*255.0+0.5) | <<8 | <<16, alpha 0xFF<<24); include-graph edge pack_unpack.hlsl -> bilinear.hlsl | status: O (pack/unpack algorithm from inlined use-sites); I (function names — inlined)
//
// 8-bit color/normal pack and unpack helpers for gbuffer / baked-color buffers.

#ifndef TINYGLADE_INC_PACK_UNPACK_HLSL
#define TINYGLADE_INC_PACK_UNPACK_HLSL

// 把 [0,1] RGB(A) 颜色打包为单个 uint(RGBA8,就近取整)。
uint pack_rgba8(float4 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint a = uint(clamp(c.w, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}

// 把 [0,1] RGB 颜色打包进 24 位(alpha 隐式省略时使用)。
uint pack_rgb8(float3 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (b << 16u) | (g << 8u) | r;
}

// 把 RGB8 uint 解包为 [0,1] float3。
float3 unpack_rgb8(uint p)
{
    float3 c;
    c.r = float(p & 0xFFu) * (1.0f / 255.0f);
    c.g = float((p >> 8u) & 0xFFu) * (1.0f / 255.0f);
    c.b = float((p >> 16u) & 0xFFu) * (1.0f / 255.0f);
    return c;
}

// 把 RGBA8 uint 解包为 [0,1] float4。
float4 unpack_rgba8(uint p)
{
    return float4(unpack_rgb8(p), float((p >> 24u) & 0xFFu) * (1.0f / 255.0f));
}

// 把 RGB8 uint 解包为 [-1,1] 法线(gbuffer 法线解码:/255*2-1)。
float3 unpack_normal_rgb8(uint p)
{
    return unpack_rgb8(p) * 2.0f - 1.0f;
}

#endif // TINYGLADE_INC_PACK_UNPACK_HLSL
