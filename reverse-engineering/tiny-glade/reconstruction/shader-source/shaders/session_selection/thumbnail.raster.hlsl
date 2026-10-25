// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/session_selection/thumbnail.raster.hlsl | F-E | bins: _session_selection_thumbnail.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Session thumbnail tile: rounded-rect masked preview, hover highlight, and an
// optional black-and-white mode (e.g. for unavailable saves).

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
    float2 thumbnail_size;
    float hover;
    uint thumbnail_index;
    uint is_black_and_white;
};

Texture2D<float4> thumbnail_tex : register(t13, space0);

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

    corner.w = 1.0f - corner.w;
    float2 pos = (corner.xy * 1.05f) * thumbnail_size + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 uv = lerp(corner.zw, float2(0.5f, 0.5f), -0.05f);
    uv.y = 1.0f - uv.y;
    o.position = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = uv;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = thumbnail_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float3 rgb = c.rgb * c.a;

    float2 d = abs(uv - 0.5f) - 0.225f;
    float mask = smoothstep(0.0f, -0.22f, (length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f)) - 0.3f);
    float4 col = float4(rgb, c.a) * mask;

    float3 shaded = col.rgb * lerp(1.0f, 1.2f, hover);
    float4 out_col;
    if (is_black_and_white == 1u)
        out_col = float4((dot(shaded, float3(0.2126f, 0.7152f, 0.0722f)) * 0.8f).xxx, col.a);
    else
        out_col = float4(shaded, col.w);
    return out_col;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _session_selection_thumbnail.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b14, space0)
{
    uint2 framebuffer_dims : packoffset(c0);
    float2 first_thumbnail_center : packoffset(c0.z);
    float2 thumbnail_size : packoffset(c1);
    float hover : packoffset(c1.z);
    uint thumbnail_index : packoffset(c1.w);
    uint is_black_and_white : packoffset(c2);
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
    uint _65 = uint(gl_VertexIndex) % 6u;
    float4 _88;
    do
    {
        if (_65 == 0u)
        {
            _88 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_65 == 1u)
            {
                _88 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_65 == 2u)
                {
                    _88 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_65 == 3u)
                    {
                        _88 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_65 == 4u)
                        {
                            _88 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _88 = float4(0.5f, -0.5f, 1.0f, 0.0f);
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
    float4 _91 = _88;
    _91.w = 1.0f - _88.w;
    float2 _106 = ((_91.xy * 1.0499999523162841796875f) * thumbnail_size) + first_thumbnail_center;
    _106.x = _106.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 _116 = lerp(_91.zw, 0.5f.xx, (-0.0500000007450580596923828125f).xx);
    _116.y = 1.0f - _116.y;
    gl_Position = float4((_106 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = _116;
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
// lifted from _session_selection_thumbnail.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b14, space0)
{
    uint2 framebuffer_dims : packoffset(c0);
    float2 first_thumbnail_center : packoffset(c0.z);
    float2 thumbnail_size : packoffset(c1);
    float hover : packoffset(c1.z);
    uint thumbnail_index : packoffset(c1.w);
    uint is_black_and_white : packoffset(c2);
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
    uint _65 = uint(gl_VertexIndex) % 6u;
    float4 _88;
    do
    {
        if (_65 == 0u)
        {
            _88 = float4(0.5f, -0.5f, 1.0f, 0.0f);
            break;
        }
        else
        {
            if (_65 == 1u)
            {
                _88 = float4(0.5f, 0.5f, 1.0f, 1.0f);
                break;
            }
            else
            {
                if (_65 == 2u)
                {
                    _88 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                    break;
                }
                else
                {
                    if (_65 == 3u)
                    {
                        _88 = float4(-0.5f, 0.5f, 0.0f, 1.0f);
                        break;
                    }
                    else
                    {
                        if (_65 == 4u)
                        {
                            _88 = float4(-0.5f, -0.5f, 0.0f, 0.0f);
                            break;
                        }
                        else
                        {
                            _88 = float4(0.5f, -0.5f, 1.0f, 0.0f);
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
    float4 _91 = _88;
    _91.w = 1.0f - _88.w;
    float2 _106 = ((_91.xy * 1.0499999523162841796875f) * thumbnail_size) + first_thumbnail_center;
    _106.x = _106.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 _116 = lerp(_91.zw, 0.5f.xx, (-0.0500000007450580596923828125f).xx);
    _116.y = 1.0f - _116.y;
    gl_Position = float4((_106 - 0.5f.xx) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    out_var_C0 = _116;
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
// lifted from _session_selection_thumbnail.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
