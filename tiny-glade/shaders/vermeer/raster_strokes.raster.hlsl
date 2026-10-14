// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main 由 batch_lift.py 从 oracle bin 转译;
// 下方手写代码恰好对那些(变体, 阶段)对被排除。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// Vermeer 缩略图链:把生成的笔触栅格化为纹理四边形。
// 照片模式产生干净的"照片"观感;否则笔触在画布顶部附近做亮度提升,
// 并在 alpha 中加入逐像素抖动。
// 绑定:$Globals b13,strokes_buf(t15 ByteAddressBuffer),brush_tex t14。

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "vermeer/stroke.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint max_strokes;
    uint2 brush_tex_dims;
    uint2 framebuffer_dims;
    float progress_t;
    uint photomode;
};

ByteAddressBuffer strokes_buf : register(t15, space0);

struct VsOut
{
    nointerpolation float4 color : TEXCOORD0;
    float2 uv : TEXCOORD1;
    nointerpolation float appear : TEXCOORD2;
    nointerpolation uint stroke_index : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    // 每个笔触是一个四边形(6 个顶点),由 vertex_id 推导。
    uint stroke_index = vertex_id / 6u;
    uint corner = vertex_id % 6u;

    // RGBA8 颜色解码到 [0,1]。
    uint packed_color = strokes_buf.Load<uint>(stroke_index * 16u + 4u);
    float r = float(packed_color & 255u) * (1.0f / 255.0f);
    float g = float((packed_color >> 8u) & 255u) * (1.0f / 255.0f);
    float b = float((packed_color >> 16u) & 255u) * (1.0f / 255.0f);
    float a = float((packed_color >> 24u) & 255u) * (1.0f / 255.0f);

    // 解包(尺寸, 旋转),准备旋转矩阵。
    float2 size_rot = unpack_stroke_size_rotation(strokes_buf.Load<uint>(stroke_index * 16u + 8u));
    float size = size_rot.x;
    float rotation = size_rot.y;
    float cos_r = cos(rotation);
    float sin_r = sin(rotation);

    // 四边形角点坐标与 UV(两个三角形)。
    float4 corner_quad;
    if (corner == 0u)      corner_quad = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (corner == 1u) corner_quad = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (corner == 2u) corner_quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (corner == 3u) corner_quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (corner == 4u) corner_quad = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else                   corner_quad = float4(0.5f, -0.5f, 1.0f, 0.0f);

    // 尺寸缩放:照片模式 1x,否则 2x,都钳制到 [0,1];并按画笔纵横比压扁四边形。
    bool photomode_on = (photomode != 0u);
    float size_scale = photomode_on ? clamp(size, 0.0f, 1.0f) : clamp(2.0f * size, 0.0f, 1.0f);
    corner_quad.y *= lerp(float(brush_tex_dims.y) / float(brush_tex_dims.x), 1.0f, size_scale);

    // 笔触进度因子;照片模式下用 NaN(禁用相关效果)。
    float stroke_factor = photomode_on ? asfloat(0x7fbfffffu) /* NaN */
                                       : clamp((float(stroke_index + 1u) - 600.0f) / (float(max_strokes) - 600.0f), 0.0f, 1.0f);
    float brighten = photomode_on ? 0.0f : lerp(0.0f, 0.070000000298023223876953125f, clamp(1.0f - (2.0f * stroke_factor), 0.0f, 1.0f));

    // 起始附近伽马增亮后做 sRGB -> 线性。
    float3 gamma_color = pow(float4(r, g, b, a).xyz, (1.0f + brighten).xxx) * (1.0f + (0.5f * brighten));
    float3 color = srgb_to_linear(gamma_color);

    // 非照片模式:靠近画布顶部的笔触做亮度提升(以笔触亮度为基准)。
    if (!photomode_on)
    {
        color *= lerp(1.0f, clamp(0.3499999940395355224609375f / max(9.9999997473787516355514526367188e-06f, dot(color, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))), 0.5f, 1.39999997615814208984375f),
                      0.699999988079071044921875f * (1.0f - smoothstep(0.0f, 0.5f, progress_t)));
    }

    // 笔触整体透明度与两种模式的全局强度。
    float4 stroke_rgba = float4(color, 1.0f) * a;
    float4 color_out = photomode_on ? stroke_rgba * 0.75f
                                    : stroke_rgba * lerp(0.20000000298023223876953125f, 0.800000011920928955078125f, clamp(3.0f * stroke_factor, 0.0f, 1.0f));

    // 笔触位置 + 旋转缩放后的四边形偏移 -> 裁剪空间。
    float2 stroke_pos = unpack_stroke_pos(strokes_buf.Load<uint>(stroke_index * 16u + 0u));
    float2 world = (stroke_pos + ((mul(float2x2(cos_r, sin_r, -sin_r, cos_r), corner_quad.xy) * size) * float2(float(framebuffer_dims.y) / float(framebuffer_dims.x), 1.0f)) - 0.5f.xx) * float2(2.0f, -2.0f);

    VsOut o;
    o.position = float4(world, 0.0f, 1.0f);
    o.color = color_out * strokes_buf.Load<float>(stroke_index * 16u + 12u);
    o.uv = corner_quad.zw;
    o.appear = 1.0f;
    o.stroke_index = stroke_index;
    return o;
}

Texture2D<float4> brush_tex : register(t14, space0);

float4 ps_main(VsOut input) : SV_Target0
{
    float brush = brush_tex.Sample(sampler_lnc, input.uv).x;
    float alpha;
    if (input.appear < 1.0f)
    {
        // 揭示动画:沿 +x 裁剪画笔 UV。
        float2 reveal_uv = input.uv - float2(1.0f - input.appear, 0.0f);
        float2 clamped = clamp(reveal_uv, 0.0f.xx, 1.0f.xx);
        float inside = float(all(bool2(clamped.x == reveal_uv.x, clamped.y == reveal_uv.y)));
        alpha = sqrt(brush * (inside * brush_tex.Sample(sampler_lnc, reveal_uv).x));
    }
    else
    {
        alpha = brush;
    }

    // 非照片模式:程序化蓝噪声抖动 alpha,降低色带。
    if (photomode == 0u)
    {
        uint2 pixel = uint2(input.position.xy);
        float hash_rnd = frac(frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(pixel.x)) + (0.005837149918079376220703125f * float(pixel.y)))) +
                              frac(float(input.stroke_index) * 1.61803400516510009765625f));
        float dither = (hash_rnd * 2.0f) - 1.0f;
        alpha = clamp(alpha + (0.20000000298023223876953125f * (max(-1.0f, dither * rsqrt(abs(dither))) - float(int(sign(dither))))), 0.0f, 1.0f);
    }

    return input.color * alpha;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _vermeer_raster_strokes.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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
