// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 自定义 OS 光标四边形:由 CPU 写入的 cursor_buf(像素位置 pos、enabled 标志、
//   大小 size_px)驱动。禁用时通过 NaN 位置把光标移出屏幕。
// 入口: vs_main / ps_main。变体: b903e43ffb3da915 与 f0adff76827e73ff;
//   vs 均由 lifted 段提供,f0ad 的 ps 为空函数。
// 绑定: cursor_buf t14 (ByteAddressBuffer), cursor_tex t15, $Globals b13, sampler_llc s3。
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

    // 按顶点 ID 取四边形角点(位置 + UV)。
    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 1.0f);

    // 读取 CPU 写的光标状态。
    CustomCursor c;
    c.pos = cursor_buf.Load<float2>(0);
    c.enabled = cursor_buf.Load<uint>(8);
    c.size_px = cursor_buf.Load<float>(12);

    // 光标中心为 pos-1,尺寸 size_px,换算到窗口 UV 再映射到裁剪空间。
    float2 screen_pos = (((corner.xy * float2(1.0f, -1.0f) + 0.5f) * c.size_px + (c.pos - 1.0f)) / float2(window_size_pixels)) - 0.5f;
    screen_pos *= float2(2.0f, -2.0f);
    float4 pos = float4(screen_pos, 1.0f, 1.0f);
    // 禁用时置 NaN,移出屏幕。
    if (c.enabled == 0u)
        pos = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = pos;
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    // 采样光标纹理(mip bias -0.5),预乘 alpha。
    float4 c = cursor_tex.SampleBias(sampler_llc, uv, -0.5f);
    return float4(c.rgb * c.a, c.a);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _custom_cursor.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// b903 变体的 vs 段:与手写 vs 语义一致(角点选择、光标状态读取、NaN 禁用)。
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
    // 按顶点 ID 选角点(位置 + UV)。
    float4 corner;
    do
    {
        if (uint(gl_VertexIndex) == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
            break;
        }
        else
        {
            if (uint(gl_VertexIndex) == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
                break;
            }
            else
            {
                if (uint(gl_VertexIndex) == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                    break;
                }
                else
                {
                    if (uint(gl_VertexIndex) == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                        break;
                    }
                    else
                    {
                        if (uint(gl_VertexIndex) == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
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
    // 读取光标状态。
    CustomCursor cursor;
    cursor.pos = cursor_buf.Load<float2>(0);
    cursor.enabled = cursor_buf.Load<uint>(8);
    cursor.size_px = cursor_buf.Load<float>(12);
    // 光标四边形 -> 窗口 UV -> 裁剪空间。
    float4 pos = float4(((((((corner.xy * float2(1.0f, -1.0f)) + 0.5f.xx) * cursor.size_px) + (cursor.pos - 1.0f.xx)) / float2(window_size_pixels)) - 0.5f.xx) * float2(2.0f, -2.0f), 1.0f, 1.0f);
    // 禁用时置 NaN。
    bool4 disabled = (0u == cursor.enabled).xxxx;
    gl_Position = float4(disabled.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : pos.x, disabled.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : pos.y, disabled.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : pos.z, disabled.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : pos.w);
    out_var_C0 = corner.zw;
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
// === 中文说明 ===
// f0ad 变体的 vs 段:与 b903 变体内容一致(变量名已语义化)。
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
    // 按顶点 ID 选角点(位置 + UV)。
    float4 corner;
    do
    {
        if (uint(gl_VertexIndex) == 0u)
        {
            corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
            break;
        }
        else
        {
            if (uint(gl_VertexIndex) == 1u)
            {
                corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
                break;
            }
            else
            {
                if (uint(gl_VertexIndex) == 2u)
                {
                    corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                    break;
                }
                else
                {
                    if (uint(gl_VertexIndex) == 3u)
                    {
                        corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
                        break;
                    }
                    else
                    {
                        if (uint(gl_VertexIndex) == 4u)
                        {
                            corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
                            break;
                        }
                        else
                        {
                            corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
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
    // 读取光标状态。
    CustomCursor cursor;
    cursor.pos = cursor_buf.Load<float2>(0);
    cursor.enabled = cursor_buf.Load<uint>(8);
    cursor.size_px = cursor_buf.Load<float>(12);
    // 光标四边形 -> 窗口 UV -> 裁剪空间。
    float4 pos = float4(((((((corner.xy * float2(1.0f, -1.0f)) + 0.5f.xx) * cursor.size_px) + (cursor.pos - 1.0f.xx)) / float2(window_size_pixels)) - 0.5f.xx) * float2(2.0f, -2.0f), 1.0f, 1.0f);
    // 禁用时置 NaN。
    bool4 disabled = (0u == cursor.enabled).xxxx;
    gl_Position = float4(disabled.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : pos.x, disabled.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : pos.y, disabled.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : pos.z, disabled.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : pos.w);
    out_var_C0 = corner.zw;
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
// === 中文说明 ===
// f0ad 变体的片元段:目标 bin 为空函数(无输出)。
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
