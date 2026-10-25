// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/custom_cursor.raster.hlsl | F-E | bins: _custom_cursor.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Custom OS cursor quad driven by a CPU-written cursor_buf (position px,
// enabled flag, size px). Disabled cursor is moved off-screen via NaN position.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/quad_mesh.hlsl"
#include "inc/uv.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 window_size_pixels;
    uint2 cursor_tex_dims;
};

struct CustomCursor
{
    float2 pos;
    uint enabled;
    float size_px;
};

ByteAddressBuffer cursor_buf : register(t14, space0);
Texture2D<float4> cursor_tex : register(t15, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 1.0f);

    CustomCursor c;
    c.pos = cursor_buf.Load<float2>(0);
    c.enabled = cursor_buf.Load<uint>(8);
    c.size_px = cursor_buf.Load<float>(12);

    float2 screen_pos = (((corner.xy * float2(1.0f, -1.0f) + 0.5f) * c.size_px + (c.pos - 1.0f)) / float2(window_size_pixels)) - 0.5f;
    screen_pos *= float2(2.0f, -2.0f);
    float4 pos = float4(screen_pos, 1.0f, 1.0f);
    if (c.enabled == 0u)
        pos = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = pos;
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = cursor_tex.SampleBias(sampler_llc, uv, -0.5f);
    return float4(c.rgb * c.a, c.a);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _custom_cursor.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct CustomCursor
{
    float2 pos;
    uint enabled;
    float size_px;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 window_size_pixels : packoffset(c0);
    uint2 cursor_tex_dims : packoffset(c0.z);
};

ByteAddressBuffer cursor_buf : register(t14, space0);

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
    float4 _83;
    do
    {
        if (uint(gl_VertexIndex) == 0u)
        {
            _83 = float4(0.5f, -0.5f, 1.0f, 1.0f);
            break;
        }
        else
        {
            if (uint(gl_VertexIndex) == 1u)
            {
                _83 = float4(0.5f, 0.5f, 1.0f, 0.0f);
                break;
            }
            else
            {
                if (uint(gl_VertexIndex) == 2u)
                {
                    _83 = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                    break;
                }
                else
                {
                    if (uint(gl_VertexIndex) == 3u)
                    {
                        _83 = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                        break;
                    }
                    else
                    {
                        if (uint(gl_VertexIndex) == 4u)
                        {
                            _83 = float4(-0.5f, -0.5f, 0.0f, 1.0f);
                            break;
                        }
                        else
                        {
                            _83 = float4(0.5f, -0.5f, 1.0f, 1.0f);
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
    CustomCursor _85;
    _85.pos = cursor_buf.Load<float2>(0);
    _85.enabled = cursor_buf.Load<uint>(8);
    _85.size_px = cursor_buf.Load<float>(12);
    float4 _103 = float4(((((((_83.xy * float2(1.0f, -1.0f)) + 0.5f.xx) * _85.size_px) + (_85.pos - 1.0f.xx)) / float2(window_size_pixels)) - 0.5f.xx) * float2(2.0f, -2.0f), 1.0f, 1.0f);
    bool4 _106 = (0u == _85.enabled).xxxx;
    gl_Position = float4(_106.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _103.x, _106.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _103.y, _106.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _103.z, _106.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _103.w);
    out_var_C0 = _83.zw;
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
// lifted from _custom_cursor.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct CustomCursor
{
    float2 pos;
    uint enabled;
    float size_px;
};

cbuffer $Globals : register(b13, space0)
{
    uint2 window_size_pixels : packoffset(c0);
    uint2 cursor_tex_dims : packoffset(c0.z);
};

ByteAddressBuffer cursor_buf : register(t14, space0);

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
    float4 _83;
    do
    {
        if (uint(gl_VertexIndex) == 0u)
        {
            _83 = float4(0.5f, -0.5f, 1.0f, 1.0f);
            break;
        }
        else
        {
            if (uint(gl_VertexIndex) == 1u)
            {
                _83 = float4(0.5f, 0.5f, 1.0f, 0.0f);
                break;
            }
            else
            {
                if (uint(gl_VertexIndex) == 2u)
                {
                    _83 = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                    break;
                }
                else
                {
                    if (uint(gl_VertexIndex) == 3u)
                    {
                        _83 = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                        break;
                    }
                    else
                    {
                        if (uint(gl_VertexIndex) == 4u)
                        {
                            _83 = float4(-0.5f, -0.5f, 0.0f, 1.0f);
                            break;
                        }
                        else
                        {
                            _83 = float4(0.5f, -0.5f, 1.0f, 1.0f);
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
    CustomCursor _85;
    _85.pos = cursor_buf.Load<float2>(0);
    _85.enabled = cursor_buf.Load<uint>(8);
    _85.size_px = cursor_buf.Load<float>(12);
    float4 _103 = float4(((((((_83.xy * float2(1.0f, -1.0f)) + 0.5f.xx) * _85.size_px) + (_85.pos - 1.0f.xx)) / float2(window_size_pixels)) - 0.5f.xx) * float2(2.0f, -2.0f), 1.0f, 1.0f);
    bool4 _106 = (0u == _85.enabled).xxxx;
    gl_Position = float4(_106.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _103.x, _106.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _103.y, _106.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _103.z, _106.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _103.w);
    out_var_C0 = _83.zw;
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
// lifted from _custom_cursor.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
