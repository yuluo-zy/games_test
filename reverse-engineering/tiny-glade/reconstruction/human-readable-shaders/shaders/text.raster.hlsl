// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/text.raster.hlsl | F-E | bins: _text.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 文本/矢量形状渲染器:vs 把打包的 PrimPacked 记录展开成四边形;
// ps 有两条路径:一是形状流解释器(按图元各自的 op 列表从 shape_stream
// 采样,bindless 纹理),二是字形图集路径(带程序化抖动噪声偏移)。

#include "inc/common.hlsl"            // 采样器(sampler_lnc s0、sampler_llc s3)
#include "inc/quad_mesh.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/uv.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
// inc/blue_noise.hlsl 不再声明该纹理(water-ice 评审 inc-fix #3 / tracker #17
// 移除了寄存器声明);此处由本文件在 t14 自行绑定。
Texture2D<float4> blue_noise_texture : register(t14, space0);
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    float2 window_size_pixels;
    uint2 font_atlas_tex_dims;
};

struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

ByteAddressBuffer primitives : register(t15, space0);
ByteAddressBuffer shape_stream : register(t16, space0);
Texture2D<float4> font_atlas_tex : register(t17, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint vid = vertex_id % 6u;
    uint prim = vertex_id / 6u;

    float4 corner;
    if (vid == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vid == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vid == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vid == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float pixel_scale = window_size_pixels.y * 0.001f;

    uint packed_color = primitives.Load<uint>(prim * 48u + 32u);
    float r = float(packed_color & 255u) * (1.0f / 255.0f);
    float g_raw = float((packed_color >> 8u) & 255u);
    float b_raw = float((packed_color >> 16u) & 255u);
    float a_raw = float((packed_color >> 24u) & 255u);
    float g = g_raw * (1.0f / 255.0f);
    float b = b_raw * (1.0f / 255.0f);
    float a = a_raw * (1.0f / 255.0f);

    float da;
    float dg;
    float db;
    if (0.04045f < a) da = pow((a + 0.055f) * 0.947867333f, 2.4f);
    else da = a_raw * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = b_raw * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = g_raw * 0.000303526991f;

    uint stream = primitives.Load<uint>(prim * 48u + 36u);
    bool snap = (stream == 0u) || (stream == 0xFFFFFFFFu);

    float2 pos_min = snap ? round(primitives.Load<float2>(prim * 48u + 0u) * pixel_scale) / pixel_scale
                          : primitives.Load<float2>(prim * 48u + 0u);
    float2 pos = pos_min + primitives.Load<float2>(prim * 48u + 8u) * (corner.xy + 0.5f);
    float2 uv = (primitives.Load<float2>(prim * 48u + 16u) + primitives.Load<float2>(prim * 48u + 24u) * corner.zw)
              / float2(font_atlas_tex_dims);

    o.position = float4((pos / (window_size_pixels / pixel_scale) - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = (stream != 0u) ? corner.zw : uv;
    o.local = float4(pos - (pos_min + primitives.Load<float2>(prim * 48u + 8u) * 0.5f), primitives.Load<float2>(prim * 48u + 8u));
    o.color = float4(float3(da, db, dg) * r, r);
    o.stream = stream;
    return o;
}

// 把一个 RGBA8 uint(sRGB)解码成线性 float4。
float4 decode_rgba8(uint p)
{
    float r = float(p & 255u) * (1.0f / 255.0f);
    float g = float((p >> 8u) & 255u) * (1.0f / 255.0f);
    float b = float((p >> 16u) & 255u) * (1.0f / 255.0f);
    float a = float((p >> 24u) & 255u) * (1.0f / 255.0f);
    float dr;
    float dg;
    float db;
    if (0.04045f < r) dr = pow((r + 0.055f) * 0.947867333f, 2.4f);
    else dr = float(p & 255u) * 0.000303526991f;
    if (0.04045f < g) dg = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else dg = float((p >> 8u) & 255u) * 0.000303526991f;
    if (0.04045f < b) db = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else db = float((p >> 16u) & 255u) * 0.000303526991f;
    return float4(dr, dg, db, a);
}

// 供字形抖动用的哈希值噪声(四角哈希的双线性插值),输出 [0,1)。
float hash_noise_octave(float2 p)
{
    int2 base = int2(floor(p));
    float2 f = frac(p);
    float2 s = f * f * (3.0f - f * 2.0f);

    uint2 c00 = uint2(base);
    uint2 c10 = uint2(base + int2(1, 0));
    uint2 c01 = uint2(base + int2(0, 1));
    uint2 c11 = uint2(base + int2(1, 1));

    uint h00 = c00.y + (c00.y << 10u); h00 ^= h00 >> 6u; h00 += h00 << 3u; h00 ^= h00 >> 11u;
    h00 = ((c00.x * 1664525u) + (h00 + (h00 << 15u)) + 1013904223u) * 1664525u;
    h00 ^= h00 >> 11u; h00 ^= (h00 << 7u) & 2636928640u; h00 ^= (h00 << 15u) & 4022730752u;
    float n00 = asfloat((h00 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h10 = c10.y + (c10.y << 10u); h10 ^= h10 >> 6u; h10 += h10 << 3u; h10 ^= h10 >> 11u;
    h10 = ((c10.x * 1664525u) + (h10 + (h10 << 15u)) + 1013904223u) * 1664525u;
    h10 ^= h10 >> 11u; h10 ^= (h10 << 7u) & 2636928640u; h10 ^= (h10 << 15u) & 4022730752u;
    float n10 = asfloat((h10 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h01 = c01.y + (c01.y << 10u); h01 ^= h01 >> 6u; h01 += h01 << 3u; h01 ^= h01 >> 11u;
    h01 = ((c01.x * 1664525u) + (h01 + (h01 << 15u)) + 1013904223u) * 1664525u;
    h01 ^= h01 >> 11u; h01 ^= (h01 << 7u) & 2636928640u; h01 ^= (h01 << 15u) & 4022730752u;
    float n01 = asfloat((h01 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    uint h11 = c11.y + (c11.y << 10u); h11 ^= h11 >> 6u; h11 += h11 << 3u; h11 ^= h11 >> 11u;
    h11 = ((c11.x * 1664525u) + (h11 + (h11 << 15u)) + 1013904223u) * 1664525u;
    h11 ^= h11 >> 11u; h11 ^= (h11 << 7u) & 2636928640u; h11 ^= (h11 << 15u) & 4022730752u;
    float n11 = asfloat((h11 & 0x7FFFFFu) | 0x3F800000u) - 1.0f;

    float nx = lerp(n00, n10, s.x);
    float ny = lerp(n01, n11, s.x);
    return lerp(nx, ny, s.y);
}

struct PsIn
{
    float2 uv : TEXCOORD0;
    float4 local : TEXCOORD1;
    float4 color : TEXCOORD2;
    nointerpolation uint stream : TEXCOORD3;
};

// 字形图集采样路径:程序化抖动噪声会偏移采样坐标。
float4 glyph_sample(PsIn vin, float4 frag_coord)
{
    float2 n = (frag_coord.xy / window_size_pixels.y) * 1440.0f * 0.3f;
    float2 n2 = n * 2.5f;

    float rnd1 = hash_noise_octave(n2 * 0.7192f);
    float rnd2 = hash_noise_octave(n2 * 0.89812f);
    float rnd3 = hash_noise_octave(n * 0.13128f);

    float4 atlas = font_atlas_tex.SampleLevel(sampler_lnc, vin.uv, 0.0f);

    float4 c = atlas;
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + float2(0.008f, -0.008f) * (rnd1 - 0.5f), 0.0f)), (rnd2 - 0.48f).xxxx);
    c = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, vin.uv + 0.004f * (rnd2 - 0.5f), 0.0f)), (rnd3 - 0.48f).xxxx);
    c.w *= 0.97f;
    c = clamp(c, 0.0f, 1.0f);

    float4 col = float4(c.rgb * c.a, c.a);
    if (all(atlas.rgb == float3(1.0f, 1.0f, 1.0f)))
        col = vin.color * col;
    return col;
}

// 采样一个 bindless 精灵并乘入 acc(返回 acc * sprite)。
float4 apply_sprite(PsIn vin, float4 acc, uint tex_idx, uint mode, float4 frag_coord)
{
    float2 uv0 = vin.uv;
    float2 px = 0.0f;
    float2 nx = 0.0f;
    if (mode == 1u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        uv0.x = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y) + 0.5f;
        nx = -ddy(uv0) * 0.5f;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 2u)
    {
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float u = (vin.uv.x - 0.5f) * (vin.local.z / vin.local.w) / (dims.x / dims.y);
        uv0 = float2(u + 0.5f, vin.uv.y);
        float aa = smoothstep(0.0f, 1.0f, max(0.0f, (abs(u) - 0.4f) * 2.0f)) * 0.25f;
        float2 n = -ddy(uv0) * 0.5f;
        n.y += float(int(sign(n.y))) * aa * 0.5f;
        nx = n;
        float2 blue = blue_noise_texture.Load(int3((uint2(frag_coord.xy) + uint2(frac(float2(9.5585f, 7.3381f)) * 256.0f)) % uint2(256u, 256u), 0u)).rg;
        uv0 += (((blue * 255.0f) * (1.0f / 256.0f) + (1.0f / 512.0f)) - 0.5f) * aa;
        px = ddx(uv0) * 0.5f;
    }
    else if (mode == 3u)
    {
        // 平铺重复的精灵(2x2 近邻块)。
        uint w, h;
        bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].GetDimensions(w, h);
        float2 dims = float2(w, h);
        float2 size = dims * 0.33333f;
        float2 uv_rep = vin.local.zw;
        float scale = max(1.0f, (0.66667f * max(size.x, size.y)) / vin.local.z);
        uv_rep *= scale;
        float2 uv_scaled = vin.uv * scale;
        float2 block = size * 0.33333f;
        float texel_a[2][2];
        float weight[2][2];
        for (int c = 0; c < 2; ++c)
        {
            if (uv_scaled[c] < block[c])
            {
                texel_a[c][0] = texel_a[c][1] = (uv_scaled[c] / block[c]) * 0.33333f;
                weight[c][0] = weight[c][1] = 0.5f;
            }
            else if (uv_scaled[c] < (uv_rep[c] - block[c]))
            {
                float d = uv_scaled[c] - block[c];
                float denom = uv_rep[c] - 2.0f * block[c];
                float wfrac = denom / block[c];
                texel_a[c][0] = (abs(1.0f - 2.0f * frac(0.5f + ((wfrac * d / denom) * 0.5f))) * 0.33333f) + 0.33333f;
                texel_a[c][1] = (abs(1.0f - 2.0f * frac(((wfrac * ((uv_rep[c] - block[c]) - uv_scaled[c])) / denom) * 0.5f)) * 0.33333f) + 0.33333f;
                weight[c][1] = d / denom;
                weight[c][0] = 1.0f - weight[c][1];
            }
            else
            {
                float v = (((uv_scaled[c] - (uv_rep[c] - block[c])) / block[c]) * 0.33333f) + 0.66667f;
                texel_a[c][0] = texel_a[c][1] = v;
                weight[c][0] = weight[c][1] = 0.5f;
            }
        }
        float wsum = 0.0f;
        float4 sum = 0.0f;
        for (int j = 0; j < 2; ++j)
        {
            for (int i = 0; i < 2; ++i)
            {
                float w = weight[0][i] * weight[1][j];
                w = w * w * w;
                wsum += w;
                sum += bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleLevel(sampler_llc, float2(texel_a[0][i], texel_a[1][j]), 0.0f) * w;
            }
        }
        float4 s = sum / wsum;
        return acc * float4(s.rgb * s.a, s.a);
    }
    float4 s = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleGrad(sampler_llc, uv0, px, nx);
    return acc * float4(s.rgb * s.a, s.a);
}

float4 ps_main(PsIn pin, float4 position : SV_Position) : SV_Target0
{
    if (pin.stream == 0u)
        return pin.color;
    if (pin.stream == 0xFFFFFFFFu)
        return glyph_sample(pin, position);

    // 形状流解释器:遍历 [stream>>8, (stream&255)+(stream>>8)) 范围内的操作。
    uint start = pin.stream >> 8u;
    uint end = (pin.stream & 255u) + start;

    float4 acc = pin.color;
    uint op = start;
    uint count = 0u;
    while (op < end)
    {
        if (count >= 256u)
            return float4(1.0f, 0.0f, 1.0f, 1.0f); // 错误:op 流失控(超过 256 步)

        uint word = shape_stream.Load<uint>(((op * 4u) >> 2u) * 4u + 0u);
        uint type = word & 255u;
        if (type == 0u)
        {
            // 颜色乘法操作。
            float4 tint = decode_rgba8(shape_stream.Load<uint>((((op + 1u) * 4u) >> 2u) * 4u + 0u));
            acc *= tint;
            op += 2u;
        }
        else if (type == 1u)
        {
            // 精灵(bindless)采样操作。
            uint tex_idx = (word >> 8u) & 0xFFFFu;
            uint mode = (word >> 24u) & 3u;
            acc = apply_sprite(pin, acc, tex_idx, mode, position);
            op += 1u;
        }
        else if (type == 2u)
        {
            // 圆角矩形遮罩操作。
            float radius = float((word >> 8u) & 255u);
            float2 d = (abs(pin.local.xy) - (pin.local.zw * 0.5f)) + radius;
            float mask = clamp(-(length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f) - radius - 0.5f), 0.0f, 1.0f);
            acc *= mask;
            op += 1u;
        }
        else
        {
            break;
        }
        count++;
    }

    return acc;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提取自 _text.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims : packoffset(c0);
    float2 window_size_pixels : packoffset(c0.z);
    uint2 font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer primitives : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;
static float4 out_var_C2;
static uint out_var_C3;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint vid = uint(gl_VertexIndex) % 6u;
    float4 corner;
    do
    {
        if (vid == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (vid == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (vid == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (vid == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (vid == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float pixel_scale = window_size_pixels.y * 0.001000000047497451305389404296875f;
    uint prim = uint(gl_VertexIndex) / 6u;
    float r = float(primitives.Load<uint>(prim * 48 + 32) & 255u) * 0.0039215688593685626983642578125f;
    float g_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 8u) & 255u);
    float g = g_raw * 0.0039215688593685626983642578125f;
    float b_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 16u) & 255u);
    float b = b_raw * 0.0039215688593685626983642578125f;
    float a_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 24u) & 255u);
    float a = a_raw * 0.0039215688593685626983642578125f;
    float da;
    if (0.040449999272823333740234375f < a)
    {
        da = pow((a + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        da = a_raw * 0.000303526991046965122222900390625f;
    }
    float db;
    if (0.040449999272823333740234375f < b)
    {
        db = pow((b + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        db = b_raw * 0.000303526991046965122222900390625f;
    }
    float dg;
    if (0.040449999272823333740234375f < g)
    {
        dg = pow((g + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        dg = g_raw * 0.000303526991046965122222900390625f;
    }
    bool snap;
    if (!(primitives.Load<uint>(prim * 48 + 36) == 0u))
    {
        snap = primitives.Load<uint>(prim * 48 + 36) == 4294967295u;
    }
    else
    {
        snap = true;
    }
    float2 pos_min;
    if (snap)
    {
        pos_min = round(primitives.Load<float2>(prim * 48 + 0) * pixel_scale) / pixel_scale.xx;
    }
    else
    {
        pos_min = primitives.Load<float2>(prim * 48 + 0);
    }
    float2 pos = pos_min + (primitives.Load<float2>(prim * 48 + 8) * (corner.xy + 0.5f.xx));
    float2 uv = (primitives.Load<float2>(prim * 48 + 16) + (primitives.Load<float2>(prim * 48 + 24) * corner.zw)) / float2(font_atlas_tex_dims);
    bool2 use_corner_zw = (primitives.Load<uint>(prim * 48 + 36) != 0u).xx;
    gl_Position = float4(((pos / (window_size_pixels / pixel_scale.xx)) - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = float2(use_corner_zw.x ? corner.zw.x : uv.x, use_corner_zw.y ? corner.zw.y : uv.y);
    out_var_C1 = float4(pos - (pos_min + (primitives.Load<float2>(prim * 48 + 8) * 0.5f)), primitives.Load<float2>(prim * 48 + 8));
    out_var_C2 = float4(float3(da, db, dg).xyz * r, r);
    out_var_C3 = primitives.Load<uint>(prim * 48 + 36);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提取自 _text.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims : packoffset(c0);
    float2 window_size_pixels : packoffset(c0.z);
    uint2 font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer shape_stream : register(t16, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> font_atlas_tex : register(t17, space0);

static float4 gl_FragCoord;
static float2 in_var_C0;
static float4 in_var_C1;
static float4 in_var_C2;
static uint in_var_C3;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint in_var_C3 : TEXCOORD3;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

uint2 spvTextureSize(Texture2D<float4> Tex, uint Level, out uint Param)
{
    uint2 ret;
    Tex.GetDimensions(Level, ret.x, ret.y, Param);
    return ret;
}

void frag_main()
{
    float2 uv = in_var_C0;
    uint2 frag_px = uint2(gl_FragCoord.xy);
    bool is_shape_stream;
    if (in_var_C3 != 0u)
    {
        is_shape_stream = in_var_C3 != 4294967295u;
    }
    else
    {
        is_shape_stream = false;
    }
    float4 out_result;
    if (is_shape_stream)
    {
        uint start = in_var_C3 >> 8u;
        uint end = (in_var_C3 & 255u) + start;
        float4 acc;
        uint count;
        acc = in_var_C2;
        count = 0u;
        float texel_a[2][2];
        float weight[2][2];
        float4 acc_new;
        uint op_new;
        uint op = start;
        for (;;)
        {
            bool continue_loop;
            if (count < 256u)
            {
                continue_loop = op < end;
            }
            else
            {
                continue_loop = false;
            }
            if (continue_loop)
            {
                uint op_word_off = (op * 4u) >> 2u;
                float4 case_result;
                uint case_next_op;
                switch (int(shape_stream.Load<uint>(op_word_off * 4 + 0) & 255u))
                {
                    case 0:
                    {
                        uint tint_word_off = ((op + 1u) * 4u) >> 2u;
                        float tint_r = float(shape_stream.Load<uint>(tint_word_off * 4 + 0) & 255u) * 0.0039215688593685626983642578125f;
                        float tint_g_raw = float((shape_stream.Load<uint>(tint_word_off * 4 + 0) >> 8u) & 255u);
                        float tint_g = tint_g_raw * 0.0039215688593685626983642578125f;
                        float tint_b_raw = float((shape_stream.Load<uint>(tint_word_off * 4 + 0) >> 16u) & 255u);
                        float tint_b = tint_b_raw * 0.0039215688593685626983642578125f;
                        float tint_a_raw = float((shape_stream.Load<uint>(tint_word_off * 4 + 0) >> 24u) & 255u);
                        float tint_a = tint_a_raw * 0.0039215688593685626983642578125f;
                        float tint_da;
                        if (0.040449999272823333740234375f < tint_a)
                        {
                            tint_da = pow((tint_a + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            tint_da = tint_a_raw * 0.000303526991046965122222900390625f;
                        }
                        float tint_db;
                        if (0.040449999272823333740234375f < tint_b)
                        {
                            tint_db = pow((tint_b + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            tint_db = tint_b_raw * 0.000303526991046965122222900390625f;
                        }
                        float tint_dg;
                        if (0.040449999272823333740234375f < tint_g)
                        {
                            tint_dg = pow((tint_g + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
                        }
                        else
                        {
                            tint_dg = tint_g_raw * 0.000303526991046965122222900390625f;
                        }
                        float3 tint_linear = float3(tint_da, tint_db, tint_dg) * tint_r;
                        case_result = acc * float4(tint_linear.x, tint_linear.y, tint_linear.z, float4(tint_r, tint_g, tint_b, tint_a).x);
                        case_next_op = op + 2u;
                        break;
                    }
                    case 1:
                    {
                        uint tex_idx = (shape_stream.Load<uint>(op_word_off * 4 + 0) >> 8u) & 65535u;
                        float2 nx;
                        float2 sprite_uv;
                        float2 px;
                        switch (int((shape_stream.Load<uint>(op_word_off * 4 + 0) >> 24u) & 3u))
                        {
                            case 0:
                            {
                                nx = (-ddy(uv)) * 0.5f;
                                sprite_uv = uv;
                                px = ddx(uv) * 0.5f;
                                break;
                            }
                            case 1:
                            {
                                uint dummy_mode1;
                                uint2 dims_mode1 = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(tex_idx)], uint(0), dummy_mode1);
                                float2 sprite_uv_a = uv;
                                sprite_uv_a.x = (((uv.x - 0.5f) * (in_var_C1.z / in_var_C1.w)) / (float(dims_mode1.x) / float(dims_mode1.y))) + 0.5f;
                                nx = (-ddy(sprite_uv_a)) * 0.5f;
                                sprite_uv = sprite_uv_a;
                                px = ddx(sprite_uv_a) * 0.5f;
                                break;
                            }
                            case 2:
                            {
                                uint dummy_mode2;
                                uint2 dims_mode2 = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(tex_idx)], uint(0), dummy_mode2);
                                float sprite_u = ((uv.x - 0.5f) * (in_var_C1.z / in_var_C1.w)) / (float(dims_mode2.x) / float(dims_mode2.y));
                                float2 sprite_uv_b = uv;
                                sprite_uv_b.x = sprite_u + 0.5f;
                                float aa = smoothstep(0.0f, 1.0f, max(0.0f, (abs(sprite_u) - 0.4000000059604644775390625f) * 2.0f)) * 0.25f;
                                float2 sprite_n = (-ddy(sprite_uv_b)) * 0.5f;
                                float sprite_n_y = sprite_n.y;
                                sprite_n.y = sprite_n_y + ((float(int(sign(sprite_n_y))) * aa) * 0.5f);
                                nx = sprite_n;
                                sprite_uv = sprite_uv_b + (((((blue_noise_texture.Load(int3((frag_px + uint2(frac(float2(9.55853271484375f, 7.338083744049072265625f)) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx).xy - 0.5f.xx) * aa);
                                px = ddx(sprite_uv_b) * 0.5f;
                                break;
                            }
                            case 3:
                            {
                                uint sprite_dims_dummy_parameter;
                                uint2 sprite_dims = spvTextureSize(bindless_textures_dyn[NonUniformResourceIndex(tex_idx)], uint(0), sprite_dims_dummy_parameter);
                                float2 tile_size = float2(float(sprite_dims.x), float(sprite_dims.y)) * 0.3333300054073333740234375f;
                                float2 uv_rep = in_var_C1.zw;
                                float2 uv_rep_scaled = uv_rep;
                                float2 tile_scale = uv_rep_scaled * max(1.0f, (0.666666686534881591796875f * max(tile_size.x, tile_size.y)) / in_var_C1.z);
                                uv_rep = tile_scale;
                                float2 uv_scaled = uv * tile_scale;
                                float2 block = tile_size * 0.3333333432674407958984375f.xx;
                                for (uint c_axis = 0u; c_axis < 2u; c_axis++)
                                {
                                    if (uv_scaled[c_axis] < block[c_axis])
                                    {
                                        float texel_t = (uv_scaled[c_axis] / block[c_axis]) * 0.3333333432674407958984375f;
                                        texel_a[c_axis][1] = texel_t;
                                        texel_a[c_axis][0] = texel_t;
                                        weight[c_axis][1] = 0.5f;
                                        weight[c_axis][0] = 0.5f;
                                    }
                                    else
                                    {
                                        if (uv_scaled[c_axis] < (uv_rep[c_axis] - block[c_axis]))
                                        {
                                            float d = uv_scaled[c_axis] - block[c_axis];
                                            float denom = uv_rep[c_axis] - (2.0f * block[c_axis]);
                                            float wfrac = denom / block[c_axis];
                                            texel_a[c_axis][0] = (abs(1.0f - (2.0f * frac(0.5f + (((wfrac * d) / denom) * 0.5f)))) * 0.3333333432674407958984375f) + 0.3333333432674407958984375f;
                                            texel_a[c_axis][1] = (abs(1.0f - (2.0f * frac(((wfrac * ((uv_rep[c_axis] - block[c_axis]) - uv_scaled[c_axis])) / denom) * 0.5f))) * 0.3333333432674407958984375f) + 0.3333333432674407958984375f;
                                            weight[c_axis][1] = d / denom;
                                            weight[c_axis][0] = 1.0f - weight[c_axis][1];
                                        }
                                        else
                                        {
                                            float texel_last = (((uv_scaled[c_axis] - (uv_rep[c_axis] - block[c_axis])) / block[c_axis]) * 0.3333333432674407958984375f) + 0.666666686534881591796875f;
                                            texel_a[c_axis][1] = texel_last;
                                            texel_a[c_axis][0] = texel_last;
                                            weight[c_axis][1] = 0.5f;
                                            weight[c_axis][0] = 0.5f;
                                        }
                                    }
                                }
                                float wsum;
                                float4 sum;
                                uint j;
                                wsum = 0.0f;
                                sum = 0.0f.xxxx;
                                j = 0u;
                                float wsum_cur;
                                float4 sum_cur;
                                for (; j < 2u; wsum = wsum_cur, sum = sum_cur, j++)
                                {
                                    wsum_cur = wsum;
                                    sum_cur = sum;
                                    for (uint i = 0u; i < 2u; )
                                    {
                                        float w = weight[0][i] * weight[1][j];
                                        float w_cubed = (w * w) * w;
                                        wsum_cur += w_cubed;
                                        sum_cur += (bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleLevel(sampler_llc, float2(texel_a[0][i], texel_a[1][j]), 0.0f) * w_cubed);
                                        i++;
                                        continue;
                                    }
                                }
                                float4 sprite_avg = sum / wsum.xxxx;
                                float3 sprite_premul_rgb = sprite_avg.xyz * sprite_avg.w;
                                acc_new = acc * float4(sprite_premul_rgb.x, sprite_premul_rgb.y, sprite_premul_rgb.z, sprite_avg.w);
                                op_new = op + 1u;
                                uint count_new = count + 1u;
                                acc = acc_new;
                                op = op_new;
                                count = count_new;
                                continue;
                            }
                            default:
                            {
                                nx = 0.0f.xx;
                                sprite_uv = uv;
                                px = 0.0f.xx;
                                break;
                            }
                        }
                        float4 sprite_samp = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].SampleGrad(sampler_llc, sprite_uv, px, nx);
                        float3 sprite_premul_rgb2 = sprite_samp.xyz * sprite_samp.w;
                        case_result = acc * float4(sprite_premul_rgb2.x, sprite_premul_rgb2.y, sprite_premul_rgb2.z, sprite_samp.w);
                        case_next_op = op + 1u;
                        break;
                    }
                    case 2:
                    {
                        float radius = float((shape_stream.Load<uint>(op_word_off * 4 + 0) >> 8u) & 255u);
                        float2 d = (abs(in_var_C1.xy) - (in_var_C1.zw * 0.5f.xx)) + radius.xx;
                        case_result = acc * clamp((((length(max(d, 0.0f.xx)) + min(max(d.x, d.y), 0.0f)) - radius) - 0.5f) * (-1.0f), 0.0f, 1.0f);
                        case_next_op = op + 1u;
                        break;
                    }
                    default:
                    {
                        case_result = acc;
                        case_next_op = op;
                        break;
                    }
                }
                acc_new = case_result;
                op_new = case_next_op;
                uint count_new = count + 1u;
                acc = acc_new;
                op = op_new;
                count = count_new;
                continue;
            }
            else
            {
                break;
            }
        }
        bool4 count_limit_hit = (count == 256u).xxxx;
        out_result = float4(count_limit_hit.x ? float4(1.0f, 0.0f, 1.0f, 1.0f).x : acc.x, count_limit_hit.y ? float4(1.0f, 0.0f, 1.0f, 1.0f).y : acc.y, count_limit_hit.z ? float4(1.0f, 0.0f, 1.0f, 1.0f).z : acc.z, count_limit_hit.w ? float4(1.0f, 0.0f, 1.0f, 1.0f).w : acc.w);
    }
    else
    {
        float4 glyph_result;
        if (in_var_C3 != 4294967295u)
        {
            float2 n = ((gl_FragCoord.xy / window_size_pixels.y.xx) * 1440.0f) * 0.300000011920928955078125f;
            float2 n2 = n * 2.5f;
            float4 atlas = font_atlas_tex.SampleLevel(sampler_lnc, uv, 0.0f);
            float3 atlas_rgb = atlas.xyz;
            float2 n1_uv = n2 * 0.71920001506805419921875f;
            int2 n1_base = int2(floor(n1_uv));
            float2 n1_f = frac(n1_uv);
            uint2 n1_base_u = uint2(n1_base);
            uint n1_h_y = n1_base_u.y;
            uint n1_h1 = n1_h_y + (n1_h_y << 10u);
            uint n1_h2 = n1_h1 ^ (n1_h1 >> 6u);
            uint n1_h3 = n1_h2 + (n1_h2 << 3u);
            uint n1_h4 = n1_h3 ^ (n1_h3 >> 11u);
            uint n1_h5 = (((n1_base_u.x * 1664525u) + (n1_h4 + (n1_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n1_h6 = n1_h5 ^ (n1_h5 >> 11u);
            uint n1_h7 = n1_h6 ^ ((n1_h6 << 7u) & 2636928640u);
            uint n1_h8 = n1_h7 ^ ((n1_h7 << 15u) & 4022730752u);
            uint2 n1_c10 = uint2(n1_base + int2(1, 0));
            uint n1_c10_y = n1_c10.y;
            uint n1_10_h1 = n1_c10_y + (n1_c10_y << 10u);
            uint n1_10_h2 = n1_10_h1 ^ (n1_10_h1 >> 6u);
            uint n1_10_h3 = n1_10_h2 + (n1_10_h2 << 3u);
            uint n1_10_h4 = n1_10_h3 ^ (n1_10_h3 >> 11u);
            uint n1_10_h5 = (((n1_c10.x * 1664525u) + (n1_10_h4 + (n1_10_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n1_10_h6 = n1_10_h5 ^ (n1_10_h5 >> 11u);
            uint n1_10_h7 = n1_10_h6 ^ ((n1_10_h6 << 7u) & 2636928640u);
            uint n1_10_h8 = n1_10_h7 ^ ((n1_10_h7 << 15u) & 4022730752u);
            uint2 n1_c01 = uint2(n1_base + int2(0, 1));
            uint n1_c01_y = n1_c01.y;
            uint n1_01_h1 = n1_c01_y + (n1_c01_y << 10u);
            uint n1_01_h2 = n1_01_h1 ^ (n1_01_h1 >> 6u);
            uint n1_01_h3 = n1_01_h2 + (n1_01_h2 << 3u);
            uint n1_01_h4 = n1_01_h3 ^ (n1_01_h3 >> 11u);
            uint n1_01_h5 = (((n1_c01.x * 1664525u) + (n1_01_h4 + (n1_01_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n1_01_h6 = n1_01_h5 ^ (n1_01_h5 >> 11u);
            uint n1_01_h7 = n1_01_h6 ^ ((n1_01_h6 << 7u) & 2636928640u);
            uint n1_01_h8 = n1_01_h7 ^ ((n1_01_h7 << 15u) & 4022730752u);
            uint2 n1_c11 = uint2(n1_base + int2(1, 1));
            uint n1_c11_y = n1_c11.y;
            uint n1_11_h1 = n1_c11_y + (n1_c11_y << 10u);
            uint n1_11_h2 = n1_11_h1 ^ (n1_11_h1 >> 6u);
            uint n1_11_h3 = n1_11_h2 + (n1_11_h2 << 3u);
            uint n1_11_h4 = n1_11_h3 ^ (n1_11_h3 >> 11u);
            uint n1_11_h5 = (((n1_c11.x * 1664525u) + (n1_11_h4 + (n1_11_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n1_11_h6 = n1_11_h5 ^ (n1_11_h5 >> 11u);
            uint n1_11_h7 = n1_11_h6 ^ ((n1_11_h6 << 7u) & 2636928640u);
            uint n1_11_h8 = n1_11_h7 ^ ((n1_11_h7 << 15u) & 4022730752u);
            float2 n1_s = (n1_f * n1_f) * (3.0f.xx - (n1_f * 2.0f));
            float n1_sx = n1_s.x;
            float n1_sx_inv = 1.0f - n1_sx;
            float n1_sy = n1_s.y;
            float2 n2_uv = n2 * 0.898119986057281494140625f;
            int2 n2_base = int2(floor(n2_uv));
            float2 n2_f = frac(n2_uv);
            uint2 n2_base_u = uint2(n2_base);
            uint n2_h_y = n2_base_u.y;
            uint n2_h1 = n2_h_y + (n2_h_y << 10u);
            uint n2_h2 = n2_h1 ^ (n2_h1 >> 6u);
            uint n2_h3 = n2_h2 + (n2_h2 << 3u);
            uint n2_h4 = n2_h3 ^ (n2_h3 >> 11u);
            uint n2_h5 = (((n2_base_u.x * 1664525u) + (n2_h4 + (n2_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n2_h6 = n2_h5 ^ (n2_h5 >> 11u);
            uint n2_h7 = n2_h6 ^ ((n2_h6 << 7u) & 2636928640u);
            uint n2_h8 = n2_h7 ^ ((n2_h7 << 15u) & 4022730752u);
            uint2 n2_c10 = uint2(n2_base + int2(1, 0));
            uint n2_c10_y = n2_c10.y;
            uint n2_10_h1 = n2_c10_y + (n2_c10_y << 10u);
            uint n2_10_h2 = n2_10_h1 ^ (n2_10_h1 >> 6u);
            uint n2_10_h3 = n2_10_h2 + (n2_10_h2 << 3u);
            uint n2_10_h4 = n2_10_h3 ^ (n2_10_h3 >> 11u);
            uint n2_10_h5 = (((n2_c10.x * 1664525u) + (n2_10_h4 + (n2_10_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n2_10_h6 = n2_10_h5 ^ (n2_10_h5 >> 11u);
            uint n2_10_h7 = n2_10_h6 ^ ((n2_10_h6 << 7u) & 2636928640u);
            uint n2_10_h8 = n2_10_h7 ^ ((n2_10_h7 << 15u) & 4022730752u);
            uint2 n2_c01 = uint2(n2_base + int2(0, 1));
            uint n2_c01_y = n2_c01.y;
            uint n2_01_h1 = n2_c01_y + (n2_c01_y << 10u);
            uint n2_01_h2 = n2_01_h1 ^ (n2_01_h1 >> 6u);
            uint n2_01_h3 = n2_01_h2 + (n2_01_h2 << 3u);
            uint n2_01_h4 = n2_01_h3 ^ (n2_01_h3 >> 11u);
            uint n2_01_h5 = (((n2_c01.x * 1664525u) + (n2_01_h4 + (n2_01_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n2_01_h6 = n2_01_h5 ^ (n2_01_h5 >> 11u);
            uint n2_01_h7 = n2_01_h6 ^ ((n2_01_h6 << 7u) & 2636928640u);
            uint n2_01_h8 = n2_01_h7 ^ ((n2_01_h7 << 15u) & 4022730752u);
            uint2 n2_c11 = uint2(n2_base + int2(1, 1));
            uint n2_c11_y = n2_c11.y;
            uint n2_11_h1 = n2_c11_y + (n2_c11_y << 10u);
            uint n2_11_h2 = n2_11_h1 ^ (n2_11_h1 >> 6u);
            uint n2_11_h3 = n2_11_h2 + (n2_11_h2 << 3u);
            uint n2_11_h4 = n2_11_h3 ^ (n2_11_h3 >> 11u);
            uint n2_11_h5 = (((n2_c11.x * 1664525u) + (n2_11_h4 + (n2_11_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n2_11_h6 = n2_11_h5 ^ (n2_11_h5 >> 11u);
            uint n2_11_h7 = n2_11_h6 ^ ((n2_11_h6 << 7u) & 2636928640u);
            uint n2_11_h8 = n2_11_h7 ^ ((n2_11_h7 << 15u) & 4022730752u);
            float2 n2_s = (n2_f * n2_f) * (3.0f.xx - (n2_f * 2.0f));
            float n2_sx = n2_s.x;
            float n2_sx_inv = 1.0f - n2_sx;
            float n2_sy = n2_s.y;
            float2 n3_uv = n * 0.131280004978179931640625f;
            int2 n3_base = int2(floor(n3_uv));
            float2 n3_f = frac(n3_uv);
            uint2 n3_base_u = uint2(n3_base);
            uint n3_h_y = n3_base_u.y;
            uint n3_h1 = n3_h_y + (n3_h_y << 10u);
            uint n3_h2 = n3_h1 ^ (n3_h1 >> 6u);
            uint n3_h3 = n3_h2 + (n3_h2 << 3u);
            uint n3_h4 = n3_h3 ^ (n3_h3 >> 11u);
            uint n3_h5 = (((n3_base_u.x * 1664525u) + (n3_h4 + (n3_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n3_h6 = n3_h5 ^ (n3_h5 >> 11u);
            uint n3_h7 = n3_h6 ^ ((n3_h6 << 7u) & 2636928640u);
            uint n3_h8 = n3_h7 ^ ((n3_h7 << 15u) & 4022730752u);
            uint2 n3_c10 = uint2(n3_base + int2(1, 0));
            uint n3_c10_y = n3_c10.y;
            uint n3_10_h1 = n3_c10_y + (n3_c10_y << 10u);
            uint n3_10_h2 = n3_10_h1 ^ (n3_10_h1 >> 6u);
            uint n3_10_h3 = n3_10_h2 + (n3_10_h2 << 3u);
            uint n3_10_h4 = n3_10_h3 ^ (n3_10_h3 >> 11u);
            uint n3_10_h5 = (((n3_c10.x * 1664525u) + (n3_10_h4 + (n3_10_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n3_10_h6 = n3_10_h5 ^ (n3_10_h5 >> 11u);
            uint n3_10_h7 = n3_10_h6 ^ ((n3_10_h6 << 7u) & 2636928640u);
            uint n3_10_h8 = n3_10_h7 ^ ((n3_10_h7 << 15u) & 4022730752u);
            uint2 n3_c01 = uint2(n3_base + int2(0, 1));
            uint n3_c01_y = n3_c01.y;
            uint n3_01_h1 = n3_c01_y + (n3_c01_y << 10u);
            uint n3_01_h2 = n3_01_h1 ^ (n3_01_h1 >> 6u);
            uint n3_01_h3 = n3_01_h2 + (n3_01_h2 << 3u);
            uint n3_01_h4 = n3_01_h3 ^ (n3_01_h3 >> 11u);
            uint n3_01_h5 = (((n3_c01.x * 1664525u) + (n3_01_h4 + (n3_01_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n3_01_h6 = n3_01_h5 ^ (n3_01_h5 >> 11u);
            uint n3_01_h7 = n3_01_h6 ^ ((n3_01_h6 << 7u) & 2636928640u);
            uint n3_01_h8 = n3_01_h7 ^ ((n3_01_h7 << 15u) & 4022730752u);
            uint2 n3_c11 = uint2(n3_base + int2(1, 1));
            uint n3_c11_y = n3_c11.y;
            uint n3_11_h1 = n3_c11_y + (n3_c11_y << 10u);
            uint n3_11_h2 = n3_11_h1 ^ (n3_11_h1 >> 6u);
            uint n3_11_h3 = n3_11_h2 + (n3_11_h2 << 3u);
            uint n3_11_h4 = n3_11_h3 ^ (n3_11_h3 >> 11u);
            uint n3_11_h5 = (((n3_c11.x * 1664525u) + (n3_11_h4 + (n3_11_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n3_11_h6 = n3_11_h5 ^ (n3_11_h5 >> 11u);
            uint n3_11_h7 = n3_11_h6 ^ ((n3_11_h6 << 7u) & 2636928640u);
            uint n3_11_h8 = n3_11_h7 ^ ((n3_11_h7 << 15u) & 4022730752u);
            float2 n3_s = (n3_f * n3_f) * (3.0f.xx - (n3_f * 2.0f));
            float n3_sx = n3_s.x;
            float n3_sx_inv = 1.0f - n3_sx;
            float n3_sy = n3_s.y;
            float4 c = lerp(atlas, min(atlas, font_atlas_tex.SampleLevel(sampler_lnc, uv + (float2(0.008000000379979610443115234375f, -0.008000000379979610443115234375f) * (((((1.0f - n1_sy) * (((asfloat(((n1_h8 ^ (n1_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n1_sx_inv) + ((asfloat(((n1_10_h8 ^ (n1_10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n1_sx))) + (((asfloat(((n1_01_h8 ^ (n1_01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n1_sx_inv) * n1_sy)) + (((asfloat(((n1_11_h8 ^ (n1_11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n1_sx) * n1_sy)) - 0.5f)), 0.0f)), (((((1.0f - n3_sy) * (((asfloat(((n3_h8 ^ (n3_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n3_sx_inv) + ((asfloat(((n3_10_h8 ^ (n3_10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n3_sx))) + (((asfloat(((n3_01_h8 ^ (n3_01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n3_sx_inv) * n3_sy)) + (((asfloat(((n3_11_h8 ^ (n3_11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n3_sx) * n3_sy)) + (-0.4799999892711639404296875f)).xxxx);
            float2 n4_uv = n * (-0.08247999846935272216796875f);
            int2 n4_base = int2(floor(n4_uv));
            float2 n4_f = frac(n4_uv);
            uint2 n4_base_u = uint2(n4_base);
            uint n4_h_y = n4_base_u.y;
            uint n4_h1 = n4_h_y + (n4_h_y << 10u);
            uint n4_h2 = n4_h1 ^ (n4_h1 >> 6u);
            uint n4_h3 = n4_h2 + (n4_h2 << 3u);
            uint n4_h4 = n4_h3 ^ (n4_h3 >> 11u);
            uint n4_h5 = (((n4_base_u.x * 1664525u) + (n4_h4 + (n4_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n4_h6 = n4_h5 ^ (n4_h5 >> 11u);
            uint n4_h7 = n4_h6 ^ ((n4_h6 << 7u) & 2636928640u);
            uint n4_h8 = n4_h7 ^ ((n4_h7 << 15u) & 4022730752u);
            uint2 n4_c10 = uint2(n4_base + int2(1, 0));
            uint n4_c10_y = n4_c10.y;
            uint n4_10_h1 = n4_c10_y + (n4_c10_y << 10u);
            uint n4_10_h2 = n4_10_h1 ^ (n4_10_h1 >> 6u);
            uint n4_10_h3 = n4_10_h2 + (n4_10_h2 << 3u);
            uint n4_10_h4 = n4_10_h3 ^ (n4_10_h3 >> 11u);
            uint n4_10_h5 = (((n4_c10.x * 1664525u) + (n4_10_h4 + (n4_10_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n4_10_h6 = n4_10_h5 ^ (n4_10_h5 >> 11u);
            uint n4_10_h7 = n4_10_h6 ^ ((n4_10_h6 << 7u) & 2636928640u);
            uint n4_10_h8 = n4_10_h7 ^ ((n4_10_h7 << 15u) & 4022730752u);
            uint2 n4_c01 = uint2(n4_base + int2(0, 1));
            uint n4_c01_y = n4_c01.y;
            uint n4_01_h1 = n4_c01_y + (n4_c01_y << 10u);
            uint n4_01_h2 = n4_01_h1 ^ (n4_01_h1 >> 6u);
            uint n4_01_h3 = n4_01_h2 + (n4_01_h2 << 3u);
            uint n4_01_h4 = n4_01_h3 ^ (n4_01_h3 >> 11u);
            uint n4_01_h5 = (((n4_c01.x * 1664525u) + (n4_01_h4 + (n4_01_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n4_01_h6 = n4_01_h5 ^ (n4_01_h5 >> 11u);
            uint n4_01_h7 = n4_01_h6 ^ ((n4_01_h6 << 7u) & 2636928640u);
            uint n4_01_h8 = n4_01_h7 ^ ((n4_01_h7 << 15u) & 4022730752u);
            uint2 n4_c11 = uint2(n4_base + int2(1, 1));
            uint n4_c11_y = n4_c11.y;
            uint n4_11_h1 = n4_c11_y + (n4_c11_y << 10u);
            uint n4_11_h2 = n4_11_h1 ^ (n4_11_h1 >> 6u);
            uint n4_11_h3 = n4_11_h2 + (n4_11_h2 << 3u);
            uint n4_11_h4 = n4_11_h3 ^ (n4_11_h3 >> 11u);
            uint n4_11_h5 = (((n4_c11.x * 1664525u) + (n4_11_h4 + (n4_11_h4 << 15u))) + 1013904223u) * 1664525u;
            uint n4_11_h6 = n4_11_h5 ^ (n4_11_h5 >> 11u);
            uint n4_11_h7 = n4_11_h6 ^ ((n4_11_h6 << 7u) & 2636928640u);
            uint n4_11_h8 = n4_11_h7 ^ ((n4_11_h7 << 15u) & 4022730752u);
            float2 n4_s = (n4_f * n4_f) * (3.0f.xx - (n4_f * 2.0f));
            float n4_sx = n4_s.x;
            float n4_sx_inv = 1.0f - n4_sx;
            float n4_sy = n4_s.y;
            float4 c2 = lerp(c, min(c, font_atlas_tex.SampleLevel(sampler_lnc, uv + (0.0040000001899898052215576171875f.xx * (((((1.0f - n2_sy) * (((asfloat(((n2_h8 ^ (n2_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n2_sx_inv) + ((asfloat(((n2_10_h8 ^ (n2_10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n2_sx))) + (((asfloat(((n2_01_h8 ^ (n2_01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n2_sx_inv) * n2_sy)) + (((asfloat(((n2_11_h8 ^ (n2_11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n2_sx) * n2_sy)) - 0.5f)), 0.0f)), (((((1.0f - n4_sy) * (((asfloat(((n4_h8 ^ (n4_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n4_sx_inv) + ((asfloat(((n4_10_h8 ^ (n4_10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n4_sx))) + (((asfloat(((n4_01_h8 ^ (n4_01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n4_sx_inv) * n4_sy)) + (((asfloat(((n4_11_h8 ^ (n4_11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * n4_sx) * n4_sy)) + (-0.4799999892711639404296875f)).xxxx);
            c2.w = c2.w * 0.9700000286102294921875f;
            float4 clamped = clamp(c2, 0.0f.xxxx, 1.0f.xxxx);
            float3 premul_rgb = clamped.xyz * clamped.w;
            float4 premul = float4(premul_rgb.x, premul_rgb.y, premul_rgb.z, clamped.w);
            float4 glyph_col;
            if (all(bool3(atlas_rgb.x == 1.0f.xxx.x, atlas_rgb.y == 1.0f.xxx.y, atlas_rgb.z == 1.0f.xxx.z)))
            {
                glyph_col = in_var_C2 * premul;
            }
            else
            {
                glyph_col = premul;
            }
            glyph_result = glyph_col;
        }
        else
        {
            glyph_result = in_var_C2;
        }
        out_result = glyph_result;
    }
    out_var_SV_TARGET = out_result;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提取自 _text.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct PrimPacked
{
    float2 pos_min;
    float2 pos_extent;
    float2 uv_min;
    float2 uv_extent;
    uint color;
    uint stream;
    uint pad0;
    uint pad1;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims : packoffset(c0);
    float2 window_size_pixels : packoffset(c0.z);
    uint2 font_atlas_tex_dims : packoffset(c1);
};

ByteAddressBuffer primitives : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;
static float4 out_var_C1;
static float4 out_var_C2;
static uint out_var_C3;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float4 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float4 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] nointerpolation uint out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint vid = uint(gl_VertexIndex) % 6u;
    float4 corner;
    do
    {
        if (vid == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (vid == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (vid == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (vid == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (vid == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
                            break;
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                }
                break; // unreachable workaround
            }
            break; // unreachable workaround
        }
        break; // unreachable workaround
    } while(false);
    float pixel_scale = window_size_pixels.y * 0.001000000047497451305389404296875f;
    uint prim = uint(gl_VertexIndex) / 6u;
    float r = float(primitives.Load<uint>(prim * 48 + 32) & 255u) * 0.0039215688593685626983642578125f;
    float g_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 8u) & 255u);
    float g = g_raw * 0.0039215688593685626983642578125f;
    float b_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 16u) & 255u);
    float b = b_raw * 0.0039215688593685626983642578125f;
    float a_raw = float((primitives.Load<uint>(prim * 48 + 32) >> 24u) & 255u);
    float a = a_raw * 0.0039215688593685626983642578125f;
    float da;
    if (0.040449999272823333740234375f < a)
    {
        da = pow((a + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        da = a_raw * 0.000303526991046965122222900390625f;
    }
    float db;
    if (0.040449999272823333740234375f < b)
    {
        db = pow((b + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        db = b_raw * 0.000303526991046965122222900390625f;
    }
    float dg;
    if (0.040449999272823333740234375f < g)
    {
        dg = pow((g + 0.054999999701976776123046875f) * 0.947867333889007568359375f, 2.400000095367431640625f);
    }
    else
    {
        dg = g_raw * 0.000303526991046965122222900390625f;
    }
    bool snap;
    if (!(primitives.Load<uint>(prim * 48 + 36) == 0u))
    {
        snap = primitives.Load<uint>(prim * 48 + 36) == 4294967295u;
    }
    else
    {
        snap = true;
    }
    float2 pos_min;
    if (snap)
    {
        pos_min = round(primitives.Load<float2>(prim * 48 + 0) * pixel_scale) / pixel_scale.xx;
    }
    else
    {
        pos_min = primitives.Load<float2>(prim * 48 + 0);
    }
    float2 pos = pos_min + (primitives.Load<float2>(prim * 48 + 8) * (corner.xy + 0.5f.xx));
    float2 uv = (primitives.Load<float2>(prim * 48 + 16) + (primitives.Load<float2>(prim * 48 + 24) * corner.zw)) / float2(font_atlas_tex_dims);
    bool2 use_corner_zw = (primitives.Load<uint>(prim * 48 + 36) != 0u).xx;
    gl_Position = float4(((pos / (window_size_pixels / pixel_scale.xx)) - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = float2(use_corner_zw.x ? corner.zw.x : uv.x, use_corner_zw.y ? corner.zw.y : uv.y);
    out_var_C1 = float4(pos - (pos_min + (primitives.Load<float2>(prim * 48 + 8) * 0.5f)), primitives.Load<float2>(prim * 48 + 8));
    out_var_C2 = float4(float3(da, db, dg).xyz * r, r);
    out_var_C3 = primitives.Load<uint>(prim * 48 + 36);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提取自 _text.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
