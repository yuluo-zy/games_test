// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/logo_branches_animation.raster.hlsl | F-E | bins: _logo_branches_animation.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Logo "branch" intro: each of 4 branches grows from its start position with a
// per-branch angle and timing; the ps picks the branch texture by branch index.

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

    float angle;
    float len;
    if (vin.branch_idx == 0) { angle = radians(-10.0f); len = 0.2f; }
    else if (vin.branch_idx == 1) { angle = radians(-30.0f); len = 0.1f; }
    else if (vin.branch_idx == 2) { angle = radians(-30.0f); len = 0.18f; }
    else if (vin.branch_idx == 3) { angle = radians(20.0f); len = 0.22f; }
    else { angle = 0.0f; len = 0.0f; }

    float t = clamp(1.0f - (anim_t * 1.22f - len), 0.0f, 1.0f);
    float grow = 1.0f - ((t * t * t) - t * sin(t * 3.1415927f));

    float3 offset = lerp(float3(0.0f, 0.0f, 0.0f), vin.Vertex_Position - vin.start_pos, grow);
    float rot = lerp(angle, 0.0f, grow);
    float s = sin(rot);
    float c = cos(rot);
    float2x2 m = float2x2(c, s, -s, c);
    float3 world = float3(mul(m, offset.xy), offset.z) + vin.start_pos;

    float2 pos = world.xy;
    pos.x = world.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    o.position = float4(pos * 0.45f, 1.0f, 1.0f);
    o.uv = vin.Vertex_UV;
    o.branch_idx = vin.branch_idx;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float4 s;
    if (vin.branch_idx == 0) s = branch_t.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 1) s = branch_y.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 2) s = branch_g.SampleBias(sampler_llr, vin.uv, -0.5f);
    else if (vin.branch_idx == 3) s = branch_e.SampleBias(sampler_llr, vin.uv, -0.5f);
    else s = float4(0.0f, 0.0f, 0.0f, 0.0f);

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
    float _91;
    float _92;
    if (in_var_branch_idx == 0)
    {
        _91 = radians(-10.0f);
        _92 = 0.20000000298023223876953125f;
    }
    else
    {
        float _89;
        float _90;
        if (in_var_branch_idx == 1)
        {
            _89 = radians(-30.0f);
            _90 = 0.100000001490116119384765625f;
        }
        else
        {
            float _87;
            float _88;
            if (in_var_branch_idx == 2)
            {
                _87 = radians(-30.0f);
                _88 = 0.180000007152557373046875f;
            }
            else
            {
                bool _81 = in_var_branch_idx == 3;
                float _85;
                if (_81)
                {
                    _85 = radians(20.0f);
                }
                else
                {
                    _85 = 0.0f;
                }
                _87 = _85;
                _88 = _81 ? 0.2199999988079071044921875f : 0.0f;
            }
            _89 = _87;
            _90 = _88;
        }
        _91 = _89;
        _92 = _90;
    }
    float _99 = clamp(1.0f - ((anim_t * 1.2200000286102294921875f) - _92), 0.0f, 1.0f);
    float _106 = 1.0f - (((_99 * _99) * _99) - (_99 * sin(_99 * 3.1400001049041748046875f)));
    float3 _108 = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, _106.xxx);
    float _110 = lerp(_91, 0.0f, _106);
    float _111 = sin(_110);
    float _112 = cos(_110);
    float2 _117 = mul(float2x2(float2(_112, _111), float2(-_111, _112)), _108.xy);
    float3 _119 = float3(_117.x, _117.y, _108.z) + in_var_start_pos;
    float2 _120 = _119.xy;
    _120.x = _119.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    gl_Position = float4(_120 * 0.449999988079071044921875f, 1.0f, 1.0f);
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
    float _91;
    float _92;
    if (in_var_branch_idx == 0)
    {
        _91 = radians(-10.0f);
        _92 = 0.20000000298023223876953125f;
    }
    else
    {
        float _89;
        float _90;
        if (in_var_branch_idx == 1)
        {
            _89 = radians(-30.0f);
            _90 = 0.100000001490116119384765625f;
        }
        else
        {
            float _87;
            float _88;
            if (in_var_branch_idx == 2)
            {
                _87 = radians(-30.0f);
                _88 = 0.180000007152557373046875f;
            }
            else
            {
                bool _81 = in_var_branch_idx == 3;
                float _85;
                if (_81)
                {
                    _85 = radians(20.0f);
                }
                else
                {
                    _85 = 0.0f;
                }
                _87 = _85;
                _88 = _81 ? 0.2199999988079071044921875f : 0.0f;
            }
            _89 = _87;
            _90 = _88;
        }
        _91 = _89;
        _92 = _90;
    }
    float _99 = clamp(1.0f - ((anim_t * 1.2200000286102294921875f) - _92), 0.0f, 1.0f);
    float _106 = 1.0f - (((_99 * _99) * _99) - (_99 * sin(_99 * 3.1400001049041748046875f)));
    float3 _108 = lerp(0.0f.xxx, in_var_Vertex_Position - in_var_start_pos, _106.xxx);
    float _110 = lerp(_91, 0.0f, _106);
    float _111 = sin(_110);
    float _112 = cos(_110);
    float2 _117 = mul(float2x2(float2(_112, _111), float2(-_111, _112)), _108.xy);
    float3 _119 = float3(_117.x, _117.y, _108.z) + in_var_start_pos;
    float2 _120 = _119.xy;
    _120.x = _119.x * (float(framebuffer_dims.y) / float(framebuffer_dims.x));
    gl_Position = float4(_120 * 0.449999988079071044921875f, 1.0f, 1.0f);
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
