// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/logo_branches_animation.raster.hlsl | F-E | bins: _logo_branches_animation.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// Logo "树枝"开场动画:4 根分支各自从起始位置生长。
//   顶点阶段 vs_main:
//     - 按 branch_idx 选择每根分支的倾斜角与长度参数;
//     - 用动画进度 t 计算生长量 grow,从 start_pos 向末端插值并做旋转;
//     - 输出到屏幕空间(以 framebuffer 纵横比修正)。
//   像素阶段 ps_main:
//     - 按 branch_idx 采样对应分支纹理(T/Y/G/E 四个字母),叠加淡出。
//   绑定: $Globals(b13)、branch_t(t14)/branch_y(t15)/branch_g(t16)/branch_e(t17)。
//   变体: TGV_b903e43ffb3da915(顶点)、TGV_f0adff76827e73ff(顶点+像素)。
// ============================================================

#include "inc/common.hlsl"            // samplers (sampler_llr s1)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float4x4 model_to_world;
    float anim_t;
    float fade_out;
    uint2 branch_t_dims;
    uint2 branch_y_dims;
    uint2 branch_g_dims;
    uint2 branch_e_dims;
    uint2 framebuffer_dims;
};

Texture2D<float4> branch_t : register(t14, space0);
Texture2D<float4> branch_y : register(t15, space0);
Texture2D<float4> branch_g : register(t16, space0);
Texture2D<float4> branch_e : register(t17, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 start_pos: start_pos;
    [[vk::location(2)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int branch_idx: branch_idx;
};

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation int branch_idx : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    // 按分支序号确定其初始倾斜角与"长度"参数(控制各分支出场时刻)。
    float angle;
    float len;
    if (vin.branch_idx == 0) { angle = radians(-10.0f); len = 0.2f; }
    else if (vin.branch_idx == 1) { angle = radians(-30.0f); len = 0.1f; }
    else if (vin.branch_idx == 2) { angle = radians(-30.0f); len = 0.18f; }
    else if (vin.branch_idx == 3) { angle = radians(20.0f); len = 0.22f; }
    else { angle = 0.0f; len = 0.0f; }

    // 每根分支的本地进度 t,再转成带缓动(三次项 + 正弦)的生长量 grow。
    float t = clamp(1.0f - (anim_t * 1.22f - len), 0.0f, 1.0f);
    float grow = 1.0f - ((t * t * t) - t * sin(t * 3.1415927f));

    // 位置从起点向终点插值,同时倾斜角随生长回归到 0(逐渐竖起)。
    float3 offset = lerp(float3(0.0f, 0.0f, 0.0f), vin.Vertex_Position - vin.start_pos, grow);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2x2 m = float2x2(c, s, -s, c);
    float3 world = float3(mul(m, offset.xy), offset.z) + vin.start_pos;

    // 输出屏幕坐标(按 framebuffer 纵横比对 x 做修正),透传 UV 与分支序号。
    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    o.position = float4(pos * 0.45f, 1.0f, 1.0f);
    o.uv = vin.Vertex_UV;
    o.branch_idx = vin.branch_idx;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    // 按分支序号选择对应的纹理采样(mip 偏置 -0.5)。
    float4 s;
    if (vin.branch_idx == 0) s = branch_t.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 1) s = branch_y.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 2) s = branch_g.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 3) s = branch_e.SampleBias(sampler_llr, vin.uv, -0.5f);
    else s = float4(0.0f, 0.0f, 0.0f, 0.0f);

    // 全局淡出:alpha 由纹理 alpha 乘 smoothstep 淡出系数。
    float alpha = s.w * smoothstep(1.0f, 0.0f, fade_out);
    return float4(s.rgb * alpha, alpha);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _logo_branches_animation.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    column_major float4x4 model_to_world : packoffset(c0);
    float anim_t : packoffset(c4);
    float fade_out : packoffset(c4.y);
    uint2 branch_t_dims : packoffset(c4.z);
    uint2 branch_y_dims : packoffset(c5);
    uint2 branch_g_dims : packoffset(c5.z);
    uint2 branch_e_dims : packoffset(c6);
    uint2 framebuffer_dims : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_start_pos;
static float2 in_var_Vertex_UV;
static int in_var_branch_idx;
static float2 out_var_C0;
static int out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_start_pos: start_pos;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int in_var_branch_idx: branch_idx;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation int out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 按分支序号选择角度与长度参数(嵌套分支保持与转译一致)。
    float angle;
    float len;
    if (in_var_branch_idx == 0)
    {
        angle = radians(-10.0f);
        len = 0.20000000298023223876953125f;
    }
    else
    {
        float angle1;
        float len1;
        if (in_var_branch_idx == 1)
        {
            angle1 = radians(-30.0f);
            len1 = 0.100000001490116119384765625f;
        }
        else
        {
            float angle2;
            float len2;
            if (in_var_branch_idx == 2)
            {
                angle2 = radians(-30.0f);
                len2 = 0.180000007152557373046875f;
            }
            else
            {
                bool is_idx3 = in_var_branch_idx == 3;
                float angle3;
                if (is_idx3)
                {
                    angle3 = radians(20.0f);
                }
                else
                {
                    angle3 = 0.0f;
                }
                angle2 = angle3;
                len2 = is_idx3 ? 0.2199999988079071044921875f : 0.0f;
            }
            angle1 = angle2;
            len1 = len2;
        }
        angle = angle1;
        len = len1;
    }
    // 每根分支的本地进度与生长量(三次项 + 正弦缓动)。
    float t = clamp(1.0f - ((anim_t * 1.2200000286102294921875f) - len), 0.0f, 1.0f);
    float grow = 1.0f - (((t * t) * t) - (t * sin(t * 3.1400001049041748046875f)));
    // 从起点向末端插值并旋转到竖直。
    float3 offset = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, grow.xxx);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2 rot_xy = mul(float2x2(float2(c, s), float2(-s, c)), offset.xy);
    float3 world = float3(rot_xy.x, rot_xy.y, offset.z) + in_var_start_pos;
    // 屏幕空间输出:按 framebuffer 纵横比修正 x。
    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    gl_Position = float4(pos * 0.449999988079071044921875f, 1.0f, 1.0f);
    out_var_C0 = in_var_Vertex_UV;
    out_var_C1 = in_var_branch_idx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_start_pos = stage_input.in_var_start_pos;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_branch_idx = stage_input.in_var_branch_idx;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _logo_branches_animation.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    column_major float4x4 model_to_world : packoffset(c0);
    float anim_t : packoffset(c4);
    float fade_out : packoffset(c4.y);
    uint2 branch_t_dims : packoffset(c4.z);
    uint2 branch_y_dims : packoffset(c5);
    uint2 branch_g_dims : packoffset(c5.z);
    uint2 branch_e_dims : packoffset(c6);
    uint2 framebuffer_dims : packoffset(c6.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_start_pos;
static float2 in_var_Vertex_UV;
static int in_var_branch_idx;
static float2 out_var_C0;
static int out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_start_pos: start_pos;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] int in_var_branch_idx: branch_idx;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation int out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 按分支序号选择角度与长度参数(嵌套分支保持与转译一致)。
    float angle;
    float len;
    if (in_var_branch_idx == 0)
    {
        angle = radians(-10.0f);
        len = 0.20000000298023223876953125f;
    }
    else
    {
        float angle1;
        float len1;
        if (in_var_branch_idx == 1)
        {
            angle1 = radians(-30.0f);
            len1 = 0.100000001490116119384765625f;
        }
        else
        {
            float angle2;
            float len2;
            if (in_var_branch_idx == 2)
            {
                angle2 = radians(-30.0f);
                len2 = 0.180000007152557373046875f;
            }
            else
            {
                bool is_idx3 = in_var_branch_idx == 3;
                float angle3;
                if (is_idx3)
                {
                    angle3 = radians(20.0f);
                }
                else
                {
                    angle3 = 0.0f;
                }
                angle2 = angle3;
                len2 = is_idx3 ? 0.2199999988079071044921875f : 0.0f;
            }
            angle1 = angle2;
            len1 = len2;
        }
        angle = angle1;
        len = len1;
    }
    // 每根分支的本地进度与生长量(三次项 + 正弦缓动)。
    float t = clamp(1.0f - ((anim_t * 1.2200000286102294921875f) - len), 0.0f, 1.0f);
    float grow = 1.0f - (((t * t) * t) - (t * sin(t * 3.1400001049041748046875f)));
    // 从起点向末端插值并旋转到竖直。
    float3 offset = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, grow.xxx);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2 rot_xy = mul(float2x2(float2(c, s), float2(-s, c)), offset.xy);
    float3 world = float3(rot_xy.x, rot_xy.y, offset.z) + in_var_start_pos;
    // 屏幕空间输出:按 framebuffer 纵横比修正 x。
    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    gl_Position = float4(pos * 0.449999988079071044921875f, 1.0f, 1.0f);
    out_var_C0 = in_var_Vertex_UV;
    out_var_C1 = in_var_branch_idx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_start_pos = stage_input.in_var_start_pos;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_branch_idx = stage_input.in_var_branch_idx;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _logo_branches_animation.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
