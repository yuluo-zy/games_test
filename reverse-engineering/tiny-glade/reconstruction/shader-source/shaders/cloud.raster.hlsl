// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/cloud.raster.hlsl | F-E | bins: _cloud.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Clouds: instanced billboards on a shell around the camera. The ps samples one
// of 4 cloud textures (by instance quarter) at 7 texel offsets, resolves light
// against the sun/sky, and gates with the day/night window.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 cloud1_dims;
    uint2 cloud2_dims;
    uint2 cloud3_dims;
    uint2 cloud4_dims;
};

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));
    uint h4 = hash(h3 + (h3 << 10u));

    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 5e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.01666667f;
    float3 center = (-facing) * (10.0f * lerp(400.0f, 800.0f, t * t));

    float height;
    if (glade_theme_id == 1u || glade_theme_id == 2u) height = -600.0f;
    else if (glade_theme_id == 3u) height = -500.0f;
    else height = -200.0f;
    if (glade_theme_id == 4u) height = -300.0f;
    center.y += height + 2000.0f * (rand01(h1) * rand01(h1));

    float2 quad_dims = (glade_theme_id == 4u) ? float2(1050.0f, 525.0f) : float2(1400.0f, 700.0f);
    uint type = int(frac(fi * 0.25f) * 4.0f);
    if (type == 1u) quad_dims *= 1.2f;
    else if (type == 2u) quad_dims *= 0.6f;
    else if (type == 3u) quad_dims *= 1.2f;

    float3 right = float3(facing.z, 0.0f, -facing.x);
    float3 world = center + (right * (quad_dims.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (quad_dims.y * quad.y));

    float2 uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    if (rand01(h4) > 0.5f)
    {
        uv.x = 1.0f - float(vertex_id & 1u);
        right = -right;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    if (instance_id > 60u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = uv;
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    uint type = int(frac(float(vin.instance) * 0.25f) * 4.0f);

    float x0 = vin.uv.x * (1.0f / 3.0f);
    float y0 = vin.uv.y * (1.0f / 3.0f);

    float s0, s1, s2, s3, s4, s5, s6;
    if (type == 1u)
    {
        s0 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud2.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud2.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud2.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud2.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 2u)
    {
        s0 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud3.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud3.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud3.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud3.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else if (type == 3u)
    {
        s0 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud4.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud4.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud4.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud4.Sample(sampler_lnc, float2(x0, y0)).x;
    }
    else
    {
        s0 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 2.0f / 3.0f)).x;
        s1 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 2.0f / 3.0f)).x;
        s2 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0 + 1.0f / 3.0f)).x;
        s3 = cloud1.Sample(sampler_lnc, float2(x0, y0 + 1.0f / 3.0f)).x;
        s4 = cloud1.Sample(sampler_lnc, float2(x0 + 2.0f / 3.0f, y0)).x;
        s5 = cloud1.Sample(sampler_lnc, float2(x0 + 1.0f / 3.0f, y0)).x;
        s6 = cloud1.Sample(sampler_lnc, float2(x0, y0)).x;
    }

    if (s4 < 0.001f)
        discard;

    float3 forward = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float4 cam4 = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 camera_world = cam4.xyz / cam4.w;

    float day_night = 1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f);
    float3 sun_light = sun_color.rgb * 0.1f * day_night;
    float3 sky = sky_high_color.rgb * 2.0f * day_night;

    float3 col = sky;
    col += sun_light * s6 * pow(max(dot(forward, vin.right), 0.0f), 2.0f);
    col += sun_light * s5 * pow(abs(min(dot(forward, vin.right), 0.0f)), 2.0f);
    col += sun_light * s3 * pow(max(forward.y, 0.0f), 2.0f);
    col += sun_light * s2 * pow(abs(min(forward.y, 0.0f)), 2.0f);
    col += sun_light * s0 * pow(max(dot(forward, vin.facing), 0.0f), 2.0f);
    col += sun_color.rgb * (smoothstep(0.9f, 1.0f, dot(forward, -normalize((camera_world / camera_world.xxx) - vin.world_pos))) * s1 * 0.5f);

    float alpha = (glade_theme_id == 3u) ? s4 * 0.5f : s4;
    float3 shaded = (glade_theme_id == 3u) ? col * 0.6f : col;
    return float4(shaded * alpha, alpha);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _cloud.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_cloud1_dims : packoffset(c1);
    uint2 _Globals_cloud2_dims : packoffset(c1.z);
    uint2 _Globals_cloud3_dims : packoffset(c2);
    uint2 _Globals_cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _119 = float(uint(gl_VertexIndex) & 1u);
    uint _121 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _125 = (float2(_119, float(_121)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _127 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _129 = _127 + (_127 << 10u);
    uint _131 = _129 ^ (_129 >> 6u);
    uint _133 = _131 + (_131 << 3u);
    uint _135 = _133 ^ (_133 >> 11u);
    uint _137 = _135 + (_135 << 15u);
    float _138 = float(uint(gl_InstanceIndex));
    uint _143 = _137 + (_137 << 10u);
    uint _145 = _143 ^ (_143 >> 6u);
    uint _147 = _145 + (_145 << 3u);
    uint _149 = _147 ^ (_147 >> 11u);
    uint _151 = _149 + (_149 << 15u);
    float _155 = asfloat((_137 & 8388607u) | 1065353216u) - 1.0f;
    uint _157 = _151 + (_151 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    uint _171 = _165 + (_165 << 10u);
    uint _173 = _171 ^ (_171 >> 6u);
    uint _175 = _173 + (_173 << 3u);
    uint _177 = _175 ^ (_175 >> 11u);
    float _187 = (frac(0.300000011920928955078125f + (_138 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 4.9999998736893758177757263183594e-05f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_151 & 8388607u) | 1065353216u) - 1.0f));
    float2 _191 = -float2(cos(_187), sin(_187));
    float _192 = _191.y;
    float _193 = _191.x;
    float _194 = -_193;
    float2 _195 = float2(_192, _194);
    float3 _196 = float3(_193, 0.0f, _192);
    float _199 = 1.0f - (_138 * 0.01666666753590106964111328125f);
    float3 _203 = (-_196) * (10.0f * lerp(400.0f, 800.0f, _199 * _199));
    bool _211;
    if (!(frame_constants_glade_theme_id == 2u))
    {
        _211 = frame_constants_glade_theme_id == 1u;
    }
    else
    {
        _211 = true;
    }
    float _217;
    if (_211)
    {
        _217 = -600.0f;
    }
    else
    {
        _217 = (frame_constants_glade_theme_id == 3u) ? (-500.0f) : (-200.0f);
    }
    bool _218 = frame_constants_glade_theme_id == 4u;
    _203.y = _203.y + ((_218 ? (-300.0f) : _217) + (2000.0f * (_155 * _155)));
    bool2 _226 = _218.xx;
    float2 _227 = float2(_226.x ? float2(1050.0f, 525.0f).x : float2(1400.0f, 700.0f).x, _226.y ? float2(1050.0f, 525.0f).y : float2(1400.0f, 700.0f).y);
    int _231 = int(frac(_138 * 0.25f) * 4.0f);
    float2 _248;
    if (_231 == 1)
    {
        _248 = _227 * 1.2000000476837158203125f;
    }
    else
    {
        float2 _247;
        if (_231 == 2)
        {
            _247 = _227 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _246;
            if (_231 == 3)
            {
                _246 = _227 * 1.2000000476837158203125f;
            }
            else
            {
                _246 = _227;
            }
            _247 = _246;
        }
        _248 = _247;
    }
    float3 _259 = _203 + ((float3(_192, 0.0f, _194) * (_248.x * _125.x)) + (float3(0.0f, 1.0f, 0.0f) * (_248.y * _125.y)));
    float2 _262 = float2(_119, float(1u - _121));
    float2 _273;
    float2 _274;
    if ((asfloat(((_177 + (_177 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _271 = _262;
        _271.x = 1.0f - _119;
        _273 = -_195;
        _274 = _271;
    }
    else
    {
        _273 = _195;
        _274 = _262;
    }
    float4 _284 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_259, 1.0f));
    bool4 _289 = (uint(gl_InstanceIndex) > 60u).xxxx;
    gl_Position = float4(_289.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _284.x, _289.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _284.y, _289.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _284.z, _289.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _284.w);
    out_var_C0 = _274;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _259;
    out_var_C3 = float3(_273.x, 0.0f, _273.y);
    out_var_C4 = _196;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _cloud.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_cloud1_dims : packoffset(c1);
    uint2 _Globals_cloud2_dims : packoffset(c1.z);
    uint2 _Globals_cloud3_dims : packoffset(c2);
    uint2 _Globals_cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _119 = float(uint(gl_VertexIndex) & 1u);
    uint _121 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _125 = (float2(_119, float(_121)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _127 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _129 = _127 + (_127 << 10u);
    uint _131 = _129 ^ (_129 >> 6u);
    uint _133 = _131 + (_131 << 3u);
    uint _135 = _133 ^ (_133 >> 11u);
    uint _137 = _135 + (_135 << 15u);
    float _138 = float(uint(gl_InstanceIndex));
    uint _143 = _137 + (_137 << 10u);
    uint _145 = _143 ^ (_143 >> 6u);
    uint _147 = _145 + (_145 << 3u);
    uint _149 = _147 ^ (_147 >> 11u);
    uint _151 = _149 + (_149 << 15u);
    float _155 = asfloat((_137 & 8388607u) | 1065353216u) - 1.0f;
    uint _157 = _151 + (_151 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    uint _171 = _165 + (_165 << 10u);
    uint _173 = _171 ^ (_171 >> 6u);
    uint _175 = _173 + (_173 << 3u);
    uint _177 = _175 ^ (_175 >> 11u);
    float _187 = (frac(0.300000011920928955078125f + (_138 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 4.9999998736893758177757263183594e-05f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_151 & 8388607u) | 1065353216u) - 1.0f));
    float2 _191 = -float2(cos(_187), sin(_187));
    float _192 = _191.y;
    float _193 = _191.x;
    float _194 = -_193;
    float2 _195 = float2(_192, _194);
    float3 _196 = float3(_193, 0.0f, _192);
    float _199 = 1.0f - (_138 * 0.01666666753590106964111328125f);
    float3 _203 = (-_196) * (10.0f * lerp(400.0f, 800.0f, _199 * _199));
    bool _211;
    if (!(frame_constants_glade_theme_id == 2u))
    {
        _211 = frame_constants_glade_theme_id == 1u;
    }
    else
    {
        _211 = true;
    }
    float _217;
    if (_211)
    {
        _217 = -600.0f;
    }
    else
    {
        _217 = (frame_constants_glade_theme_id == 3u) ? (-500.0f) : (-200.0f);
    }
    bool _218 = frame_constants_glade_theme_id == 4u;
    _203.y = _203.y + ((_218 ? (-300.0f) : _217) + (2000.0f * (_155 * _155)));
    bool2 _226 = _218.xx;
    float2 _227 = float2(_226.x ? float2(1050.0f, 525.0f).x : float2(1400.0f, 700.0f).x, _226.y ? float2(1050.0f, 525.0f).y : float2(1400.0f, 700.0f).y);
    int _231 = int(frac(_138 * 0.25f) * 4.0f);
    float2 _248;
    if (_231 == 1)
    {
        _248 = _227 * 1.2000000476837158203125f;
    }
    else
    {
        float2 _247;
        if (_231 == 2)
        {
            _247 = _227 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _246;
            if (_231 == 3)
            {
                _246 = _227 * 1.2000000476837158203125f;
            }
            else
            {
                _246 = _227;
            }
            _247 = _246;
        }
        _248 = _247;
    }
    float3 _259 = _203 + ((float3(_192, 0.0f, _194) * (_248.x * _125.x)) + (float3(0.0f, 1.0f, 0.0f) * (_248.y * _125.y)));
    float2 _262 = float2(_119, float(1u - _121));
    float2 _273;
    float2 _274;
    if ((asfloat(((_177 + (_177 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _271 = _262;
        _271.x = 1.0f - _119;
        _273 = -_195;
        _274 = _271;
    }
    else
    {
        _273 = _195;
        _274 = _262;
    }
    float4 _284 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_259, 1.0f));
    bool4 _289 = (uint(gl_InstanceIndex) > 60u).xxxx;
    gl_Position = float4(_289.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _284.x, _289.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _284.y, _289.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _284.z, _289.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _284.w);
    out_var_C0 = _274;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _259;
    out_var_C3 = float3(_273.x, 0.0f, _273.y);
    out_var_C4 = _196;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _cloud.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    int _61 = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float _120;
    if (_61 == 1)
    {
        _120 = cloud2.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
    }
    else
    {
        float _119;
        if (_61 == 2)
        {
            _119 = cloud3.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
        }
        else
        {
            float _118;
            if (_61 == 3)
            {
                _118 = cloud4.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            else
            {
                _118 = cloud1.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            _119 = _118;
        }
        _120 = _119;
    }
    if (_120 < 0.001000000047497451305389404296875f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
