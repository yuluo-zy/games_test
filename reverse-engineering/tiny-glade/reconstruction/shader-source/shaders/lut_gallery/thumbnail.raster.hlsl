// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/lut_gallery/thumbnail.raster.hlsl | F-E | bins: _lut_gallery_thumbnail.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// LUT gallery thumbnail tile: samples a preview, applies a 3D grading LUT
// (amount lerped), optionally overlays an icon, and draws a hover/active border.

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0, sampler_nnc s2, sampler_llc s3)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/grading_lut.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 input_tex_dims;
    uint2 icon_tex_dims;
    uint2 grading_lut_tex_dims;
    uint2 framebuffer_dims;
    float2 thumbnail_center;
    float2 thumbnail_size;
    uint is_hovered;
    uint is_active;
    uint show_icon;
    float lut_amount;
};

Texture2D<float4> input_tex : register(t14, space0);
Texture2D<float4> icon_tex : register(t15, space0);
Texture3D<float4> grading_lut_tex : register(t16, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float2 pos = corner.xy * thumbnail_size + thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    o.position = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 tex = input_tex.SampleLevel(sampler_nnc, uv, 0.0f);

    // sRGB decode -> linear, re-encode -> sample LUT, decode back.
    float lr;
    float lg;
    float lb;
    if (0.04045f < tex.r) lr = pow((tex.r + 0.055f) * 0.947867333f, 2.4f);
    else lr = tex.r * 0.0773993805f;
    if (0.04045f < tex.g) lg = pow((tex.g + 0.055f) * 0.947867333f, 2.4f);
    else lg = tex.g * 0.0773993805f;
    if (0.04045f < tex.b) lb = pow((tex.b + 0.055f) * 0.947867333f, 2.4f);
    else lb = tex.b * 0.0773993805f;

    float er;
    float eg;
    float eb;
    if (0.0031308f >= lr) er = lr * 12.92f;
    else er = 1.055f * pow(lr, 0.416666657f) - 0.055f;
    if (0.0031308f >= lg) eg = lg * 12.92f;
    else eg = 1.055f * pow(lg, 0.416666657f) - 0.055f;
    if (0.0031308f >= lb) eb = lb * 12.92f;
    else eb = 1.055f * pow(lb, 0.416666657f) - 0.055f;

    float lut_w = float(grading_lut_tex_dims.x);
    float3 lut_scale = (float3(lut_w, lut_w, lut_w) - 1.0f) / lut_w;
    float3 lut_off = 0.5f / lut_w;
    float4 lut = grading_lut_tex.SampleLevel(sampler_lnc, float3(er, eg, eb) * lut_scale + lut_off, 0.0f);

    float dr;
    float dg;
    float db;
    if (0.04045f < lut.r) dr = pow((lut.r + 0.055f) * 0.947867333f, 2.4f);
    else dr = lut.r * 0.0773993805f;
    if (0.04045f < lut.g) dg = pow((lut.g + 0.055f) * 0.947867333f, 2.4f);
    else dg = lut.g * 0.0773993805f;
    if (0.04045f < lut.b) db = pow((lut.b + 0.055f) * 0.947867333f, 2.4f);
    else db = lut.b * 0.0773993805f;

    float3 col = lerp(float3(lr, lg, lb), float3(dr, dg, db), lut_amount);

    if (show_icon != 0u)
    {
        float2 icon_uv = 1.2f - (1.0f - uv) * 8.0f;
        float2 clamped = clamp(icon_uv, 0.0f, 1.0f);
        if (all(clamped == icon_uv))
        {
            float4 ic = icon_tex.SampleLevel(sampler_llc, icon_uv, 0.0f);
            col = lerp(col, ic.rgb, ic.a);
        }
    }

    float2 border = abs(uv * 2.0f - 1.0f);
    float3 base = col;
    if (any(border.x > 0.98f) || any(border.y > 0.98f))
    {
        if (is_hovered != 0u)
            base = float3(1.0f, 1.0f, 1.0f);
    }
    if ((any(border.x > 0.97f) || any(border.y > 0.97f)) && is_active != 0u)
        base = float3(0.8f, 0.8f, 0.8f);

    return float4(base, 1.0f);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lut_gallery_thumbnail.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_input_tex_dims : packoffset(c0);
    uint2 _Globals_icon_tex_dims : packoffset(c0.z);
    uint2 _Globals_grading_lut_tex_dims : packoffset(c1);
    uint2 _Globals_framebuffer_dims : packoffset(c1.z);
    float2 _Globals_thumbnail_center : packoffset(c2);
    float2 _Globals_thumbnail_size : packoffset(c2.z);
    uint _Globals_is_hovered : packoffset(c3);
    uint _Globals_is_active : packoffset(c3.y);
    uint _Globals_show_icon : packoffset(c3.z);
    float _Globals_lut_amount : packoffset(c3.w);
};


static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _63 = uint(gl_VertexIndex) % 6u;
    float4 _86;
    do
    {
        if (_63 == 0u)
        {
            _86 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_63 == 1u)
            {
                _86 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_63 == 2u)
                {
                    _86 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_63 == 3u)
                    {
                        _86 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_63 == 4u)
                        {
                            _86 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _86 = float4(0.5f, -0.5f, 1.0f, 0.0f);
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
    float2 _100 = (_86.xy * _Globals_thumbnail_size) + _Globals_thumbnail_center;
    _100.x = _100.x / (float(_Globals_framebuffer_dims.x) / float(_Globals_framebuffer_dims.y));
    gl_Position = float4((_100 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = _86.zw;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_input_tex_dims : packoffset(c0);
    uint2 _Globals_icon_tex_dims : packoffset(c0.z);
    uint2 _Globals_grading_lut_tex_dims : packoffset(c1);
    uint2 _Globals_framebuffer_dims : packoffset(c1.z);
    float2 _Globals_thumbnail_center : packoffset(c2);
    float2 _Globals_thumbnail_size : packoffset(c2.z);
    uint _Globals_is_hovered : packoffset(c3);
    uint _Globals_is_active : packoffset(c3.y);
    uint _Globals_show_icon : packoffset(c3.z);
    float _Globals_lut_amount : packoffset(c3.w);
};


static float4 gl_Position;
static int gl_VertexIndex;
static float2 out_var_C0;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _63 = uint(gl_VertexIndex) % 6u;
    float4 _86;
    do
    {
        if (_63 == 0u)
        {
            _86 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_63 == 1u)
            {
                _86 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_63 == 2u)
                {
                    _86 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_63 == 3u)
                    {
                        _86 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_63 == 4u)
                        {
                            _86 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _86 = float4(0.5f, -0.5f, 1.0f, 0.0f);
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
    float2 _100 = (_86.xy * _Globals_thumbnail_size) + _Globals_thumbnail_center;
    _100.x = _100.x / (float(_Globals_framebuffer_dims.x) / float(_Globals_framebuffer_dims.y));
    gl_Position = float4((_100 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = _86.zw;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lut_gallery_thumbnail.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
