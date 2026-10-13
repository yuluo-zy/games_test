// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/billboard_forest.raster.hlsl
// Evidence: _billboard_forest.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/framebuffer_dims/cloud1..4_dims),
//   frame_constants set1/b0. (ps): sampler_lnc s0, cloud1..4 t14..17, color_library b18,
//   frame_constants set1/b0. vs_main: procedural forest billboard quads (golden-ratio angle,
//   distance ring, 4 cloud-size variants, NaN cull past 500). ps_main: 5-tap cloud-cluster
//   normal from the per-type cloud texture, theme-colored albedo, packed uint2 target.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"

// color_library.hlsl is guard-skipped; flat-member cbuffer at set0/b18.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b18, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/sh.hlsl"
// NOTE: inc/lighting.hlsl is omitted: it references color_library.indirect_light_tint
// (nested-struct form) which conflicts with this shader's flat-member `color_library`
// cbuffer (embedded target is flat, 37 members). Its functions are unused here; the
// embedded lighting.hlsl OpString is recorded only in the original build (residual).

// $Globals members (implicit cbuffer at set0/b13).
uint2 framebuffer_dims;
uint2 cloud1_dims;
uint2 cloud2_dims;
uint2 cloud3_dims;
uint2 cloud4_dims;

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    float vx = float(vertex_id & 1u);
    float vy = float((vertex_id >> 1u) & 1u);
    float2 quad = (float2(vx, vy) - 0.5f) * float2(1.0f, 2.0f);
    uint seed = (instance_id * 12u) + 25u;
    float inst = float(instance_id);

    float angle = hash_angle(seed);
    float2 facing = -float2(cos(angle), sin(angle));
    float2 facing_perp = float2(facing.y, -facing.x);
    float3 dir = float3(facing.x, 0.0f, facing.y);
    float dist_f = 1.0f - (inst * 0.002f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y -= lerp(35.0f, 45.0f, dist2);

    uint h1 = hash(seed);
    uint h2 = hash(h1);
    uint h3 = hash(h2);
    uint h4 = hash(h3);
    float2 size = float2(36.0f, 18.0f) * lerp(0.9f, 1.2f, rand01(h4));
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        size2 = size * 1.2f;
        size2.y *= 0.7f;
    }
    else if (type == 2)
    {
        size2 = size * 0.6f;
    }
    else if (type == 3)
    {
        size2 = size * 1.0f;
        size2.y *= 0.9f;
    }
    else
    {
        size2 = size;
    }

    uint h5 = hash(h4);
    uint h6 = hash(h5);
    uint h7 = hash(h6);
    float3 pos = base
        + (float3(facing.y, 0.0f, -facing.x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, rand01(h6))))
        + (float3(0.0f, lerp(1.5f, 2.5f, rand01(h5)), 0.0f) * (size2.y * quad.y));

    float2 uv_base = float2(vx, 1.0f - vy);
    float2 uv_out;
    float2 facing_out;
    if (rand01(h7) > 0.5f)
    {
        uv_out = float2(1.0f - vx, uv_base.y);
        facing_out = -facing_perp;
    }
    else
    {
        uv_out = uv_base;
        facing_out = facing_perp;
    }

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    if (instance_id > 500u)
    {
        clip = asfloat(0x7fbfffffu);
    }
    o.position = clip;
    o.C0 = uv_out;
    o.C1 = instance_id;
    o.C2 = pos;
    o.C3 = float3(facing_out.x, 0.0f, facing_out.y);
    o.C4 = dir;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    int type = int(frac(float(i.C1) * 0.25f) * 4.0f);
    float s;
    float t;
    float u;
    float v;
    float w;
    if (type == 1)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud2.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud2.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud2.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 2)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud3.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud3.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud3.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 3)
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud4.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud4.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud4.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else
    {
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud1.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud1.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud1.Sample(sampler_lnc, float2(sx, sy)).x;
    }

    float w2 = w * w;
    float v2 = v * v;
    float v3 = (v2 * v2) * v;
    float u2 = u * u;
    float t2 = t * t;
    float s2 = s * s;
    float3 n = normalize(float3(((w2 * w2) * w) - (v3 * v3), (t2 * t2) * t, 1.0f - ((s2 * s2) * s)));
    if (smoothstep(0.0f, 0.5f, (u2 * u2) * u) < 0.5f)
    {
        discard;
    }

    float3 albedo;
    if (glade_theme_id == 2u)
    {
        albedo = pow(canopy_color_1.xyz, 1.5f) * 0.9f;
    }
    else if (glade_theme_id == 4u)
    {
        albedo = lerp(pow(canopy_color_1.xyz, 1.5f), 1.0f, 0.5f);
    }
    else
    {
        albedo = float3(0.024f, 0.054f, 0.018f);
    }

    float3 world_n = (normalize((i.C3 * n.x) + (float3(0.0f, 1.0f, 0.0f) * n.y) + (i.C4 * n.z)) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(lerp(albedo * 0.5f, sky_low_color.xyz * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f)), 0.02f), 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(world_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(world_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(world_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xFF000000u;
    o.target = uint2(normal_packed, albedo_packed | 0x46000000u);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _billboard_forest.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _113 = float(uint(gl_VertexIndex) & 1u);
    uint _115 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _119 = (float2(_113, float(_115)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _121 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    uint _131 = _129 + (_129 << 15u);
    float _132 = float(uint(gl_InstanceIndex));
    uint _137 = _131 + (_131 << 10u);
    uint _139 = _137 ^ (_137 >> 6u);
    uint _141 = _139 + (_139 << 3u);
    uint _143 = _141 ^ (_141 >> 11u);
    uint _145 = _143 + (_143 << 15u);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    uint _155 = _153 + (_153 << 15u);
    uint _157 = _155 + (_155 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    float _166 = frac(0.89999997615814208984375f + (_132 * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 _170 = -float2(cos(_166), sin(_166));
    float _171 = _170.y;
    float _172 = _170.x;
    float _173 = -_172;
    float2 _174 = float2(_171, _173);
    float3 _175 = float3(_172, 0.0f, _171);
    float _178 = 1.0f - (_132 * 0.00200000009499490261077880859375f);
    float _179 = _178 * _178;
    float3 _181 = (-_175) * lerp(450.0f, 550.0f, _179);
    _181.y = _181.y - lerp(35.0f, 45.0f, _179);
    uint _187 = _165 + (_165 << 10u);
    uint _189 = _187 ^ (_187 >> 6u);
    uint _191 = _189 + (_189 << 3u);
    uint _193 = _191 ^ (_191 >> 11u);
    uint _195 = _193 + (_193 << 15u);
    float2 _201 = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((_165 & 8388607u) | 1065353216u) - 1.0f);
    int _205 = int(frac(_132 * 0.25f) * 4.0f);
    float2 _228;
    if (_205 == 1)
    {
        float2 _210 = _201 * 1.2000000476837158203125f;
        _210.y = _210.y * 0.699999988079071044921875f;
        _228 = _210;
    }
    else
    {
        float2 _227;
        if (_205 == 2)
        {
            _227 = _201 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _226;
            if (_205 == 3)
            {
                float2 _222 = _201 * 1.0f;
                _222.y = _222.y * 0.89999997615814208984375f;
                _226 = _222;
            }
            else
            {
                _226 = _201;
            }
            _227 = _226;
        }
        _228 = _227;
    }
    uint _230 = _195 + (_195 << 10u);
    uint _232 = _230 ^ (_230 >> 6u);
    uint _234 = _232 + (_232 << 3u);
    uint _236 = _234 ^ (_234 >> 11u);
    uint _238 = _236 + (_236 << 15u);
    uint _245 = _238 + (_238 << 10u);
    uint _247 = _245 ^ (_245 >> 6u);
    uint _249 = _247 + (_247 << 3u);
    uint _251 = _249 ^ (_249 >> 11u);
    float3 _271 = _181 + ((float3(_171, 0.0f, _173) * ((_228.x * _119.x) * lerp(1.5f, 2.0f, asfloat((_238 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((_195 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (_228.y * _119.y)));
    float2 _274 = float2(_113, float(1u - _115));
    float2 _285;
    float2 _286;
    if ((asfloat(((_251 + (_251 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _283 = _274;
        _283.x = 1.0f - _113;
        _285 = -_174;
        _286 = _283;
    }
    else
    {
        _285 = _174;
        _286 = _274;
    }
    float4 _296 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_271, 1.0f));
    bool4 _301 = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(_301.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _296.x, _301.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _296.y, _301.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _296.z, _301.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _296.w);
    out_var_C0 = _286;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _271;
    out_var_C3 = float3(_285.x, 0.0f, _285.y);
    out_var_C4 = _175;
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _billboard_forest.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

cbuffer color_library : register(b18, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;
static float3 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    int _118 = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float _261;
    float _262;
    float _263;
    float _264;
    float _265;
    if (_118 == 1)
    {
        float _126 = in_var_C0.x * 0.3333333432674407958984375f;
        float _128 = in_var_C0.y * 0.3333333432674407958984375f;
        _261 = cloud2.Sample(sampler_lnc, float2(_126, _128 + 0.666666686534881591796875f)).x;
        _262 = cloud2.Sample(sampler_lnc, float2(_126, _128 + 0.3333333432674407958984375f)).x;
        _263 = cloud2.Sample(sampler_lnc, float2(_126 + 0.666666686534881591796875f, _128)).x;
        _264 = cloud2.Sample(sampler_lnc, float2(_126 + 0.3333333432674407958984375f, _128)).x;
        _265 = cloud2.Sample(sampler_lnc, float2(_126, _128)).x;
    }
    else
    {
        float _256;
        float _257;
        float _258;
        float _259;
        float _260;
        if (_118 == 2)
        {
            float _160 = in_var_C0.x * 0.3333333432674407958984375f;
            float _162 = in_var_C0.y * 0.3333333432674407958984375f;
            _256 = cloud3.Sample(sampler_lnc, float2(_160, _162 + 0.666666686534881591796875f)).x;
            _257 = cloud3.Sample(sampler_lnc, float2(_160, _162 + 0.3333333432674407958984375f)).x;
            _258 = cloud3.Sample(sampler_lnc, float2(_160 + 0.666666686534881591796875f, _162)).x;
            _259 = cloud3.Sample(sampler_lnc, float2(_160 + 0.3333333432674407958984375f, _162)).x;
            _260 = cloud3.Sample(sampler_lnc, float2(_160, _162)).x;
        }
        else
        {
            float _251;
            float _252;
            float _253;
            float _254;
            float _255;
            if (_118 == 3)
            {
                float _194 = in_var_C0.x * 0.3333333432674407958984375f;
                float _196 = in_var_C0.y * 0.3333333432674407958984375f;
                _251 = cloud4.Sample(sampler_lnc, float2(_194, _196 + 0.666666686534881591796875f)).x;
                _252 = cloud4.Sample(sampler_lnc, float2(_194, _196 + 0.3333333432674407958984375f)).x;
                _253 = cloud4.Sample(sampler_lnc, float2(_194 + 0.666666686534881591796875f, _196)).x;
                _254 = cloud4.Sample(sampler_lnc, float2(_194 + 0.3333333432674407958984375f, _196)).x;
                _255 = cloud4.Sample(sampler_lnc, float2(_194, _196)).x;
            }
            else
            {
                float _224 = in_var_C0.x * 0.3333333432674407958984375f;
                float _226 = in_var_C0.y * 0.3333333432674407958984375f;
                _251 = cloud1.Sample(sampler_lnc, float2(_224, _226 + 0.666666686534881591796875f)).x;
                _252 = cloud1.Sample(sampler_lnc, float2(_224, _226 + 0.3333333432674407958984375f)).x;
                _253 = cloud1.Sample(sampler_lnc, float2(_224 + 0.666666686534881591796875f, _226)).x;
                _254 = cloud1.Sample(sampler_lnc, float2(_224 + 0.3333333432674407958984375f, _226)).x;
                _255 = cloud1.Sample(sampler_lnc, float2(_224, _226)).x;
            }
            _256 = _251;
            _257 = _252;
            _258 = _253;
            _259 = _254;
            _260 = _255;
        }
        _261 = _256;
        _262 = _257;
        _263 = _258;
        _264 = _259;
        _265 = _260;
    }
    float _266 = _265 * _265;
    float _269 = _264 * _264;
    float _271 = (_269 * _269) * _264;
    float _272 = _263 * _263;
    float _275 = _262 * _262;
    float _278 = _261 * _261;
    float3 _285 = normalize(float3(((_266 * _266) * _265) - (_271 * _271), (_275 * _275) * _262, 1.0f - ((_278 * _278) * _261)));
    if (smoothstep(0.0f, 0.5f, (_272 * _272) * _263) < 0.5f)
    {
        discard;
    }
    float3 _320;
    if (frame_constants_glade_theme_id == 2u)
    {
        _320 = pow(color_library_canopy_color_1.xyz, 1.5f.xxx) * 0.89999997615814208984375f;
    }
    else
    {
        float3 _319;
        if (frame_constants_glade_theme_id == 4u)
        {
            _319 = lerp(pow(color_library_canopy_color_1.xyz, 1.5f.xxx), 1.0f.xxx, 0.5f.xxx);
        }
        else
        {
            _319 = float3(0.0240000002086162567138671875f, 0.053999997675418853759765625f, 0.01800000108778476715087890625f);
        }
        _320 = _319;
    }
    float3 _335 = (normalize(((in_var_C3 * _285.x) + (float3(0.0f, 1.0f, 0.0f) * _285.y)) + (in_var_C4 * _285.z)) * 0.5f) + 0.5f.xxx;
    float3 _357 = sqrt(clamp(lerp(_320 * 0.5f, frame_constants_sky_low_color.xyz * (1.0f - ((smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, frame_constants_time_of_day) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, frame_constants_time_of_day)) * 0.75f)), 0.0199999995529651641845703125f.xxx), 0.0f.xxx, 1.0f.xxx));
    uint _376 = (uint((clamp(_357.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_357.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_357.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _377 = uint2(((uint((clamp(_335.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_335.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_335.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, _376);
    _377.y = _376 | 1174405120u;
    out_var_SV_TARGET0 = _377;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _billboard_forest.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _113 = float(uint(gl_VertexIndex) & 1u);
    uint _115 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _119 = (float2(_113, float(_115)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _121 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    uint _131 = _129 + (_129 << 15u);
    float _132 = float(uint(gl_InstanceIndex));
    uint _137 = _131 + (_131 << 10u);
    uint _139 = _137 ^ (_137 >> 6u);
    uint _141 = _139 + (_139 << 3u);
    uint _143 = _141 ^ (_141 >> 11u);
    uint _145 = _143 + (_143 << 15u);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    uint _155 = _153 + (_153 << 15u);
    uint _157 = _155 + (_155 << 10u);
    uint _159 = _157 ^ (_157 >> 6u);
    uint _161 = _159 + (_159 << 3u);
    uint _163 = _161 ^ (_161 >> 11u);
    uint _165 = _163 + (_163 << 15u);
    float _166 = frac(0.89999997615814208984375f + (_132 * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 _170 = -float2(cos(_166), sin(_166));
    float _171 = _170.y;
    float _172 = _170.x;
    float _173 = -_172;
    float2 _174 = float2(_171, _173);
    float3 _175 = float3(_172, 0.0f, _171);
    float _178 = 1.0f - (_132 * 0.00200000009499490261077880859375f);
    float _179 = _178 * _178;
    float3 _181 = (-_175) * lerp(450.0f, 550.0f, _179);
    _181.y = _181.y - lerp(35.0f, 45.0f, _179);
    uint _187 = _165 + (_165 << 10u);
    uint _189 = _187 ^ (_187 >> 6u);
    uint _191 = _189 + (_189 << 3u);
    uint _193 = _191 ^ (_191 >> 11u);
    uint _195 = _193 + (_193 << 15u);
    float2 _201 = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((_165 & 8388607u) | 1065353216u) - 1.0f);
    int _205 = int(frac(_132 * 0.25f) * 4.0f);
    float2 _228;
    if (_205 == 1)
    {
        float2 _210 = _201 * 1.2000000476837158203125f;
        _210.y = _210.y * 0.699999988079071044921875f;
        _228 = _210;
    }
    else
    {
        float2 _227;
        if (_205 == 2)
        {
            _227 = _201 * 0.60000002384185791015625f;
        }
        else
        {
            float2 _226;
            if (_205 == 3)
            {
                float2 _222 = _201 * 1.0f;
                _222.y = _222.y * 0.89999997615814208984375f;
                _226 = _222;
            }
            else
            {
                _226 = _201;
            }
            _227 = _226;
        }
        _228 = _227;
    }
    uint _230 = _195 + (_195 << 10u);
    uint _232 = _230 ^ (_230 >> 6u);
    uint _234 = _232 + (_232 << 3u);
    uint _236 = _234 ^ (_234 >> 11u);
    uint _238 = _236 + (_236 << 15u);
    uint _245 = _238 + (_238 << 10u);
    uint _247 = _245 ^ (_245 >> 6u);
    uint _249 = _247 + (_247 << 3u);
    uint _251 = _249 ^ (_249 >> 11u);
    float3 _271 = _181 + ((float3(_171, 0.0f, _173) * ((_228.x * _119.x) * lerp(1.5f, 2.0f, asfloat((_238 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((_195 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (_228.y * _119.y)));
    float2 _274 = float2(_113, float(1u - _115));
    float2 _285;
    float2 _286;
    if ((asfloat(((_251 + (_251 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 _283 = _274;
        _283.x = 1.0f - _113;
        _285 = -_174;
        _286 = _283;
    }
    else
    {
        _285 = _174;
        _286 = _274;
    }
    float4 _296 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_271, 1.0f));
    bool4 _301 = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(_301.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _296.x, _301.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _296.y, _301.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _296.z, _301.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _296.w);
    out_var_C0 = _286;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _271;
    out_var_C3 = float3(_285.x, 0.0f, _285.y);
    out_var_C4 = _175;
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
// lifted from _billboard_forest.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int _68 = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float _127;
    if (_68 == 1)
    {
        _127 = cloud2.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
    }
    else
    {
        float _126;
        if (_68 == 2)
        {
            _126 = cloud3.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
        }
        else
        {
            float _125;
            if (_68 == 3)
            {
                _125 = cloud4.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            else
            {
                _125 = cloud1.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            _126 = _125;
        }
        _127 = _126;
    }
    float _128 = _127 * _127;
    if (smoothstep(0.0f, 0.5f, (_128 * _128) * _127) < 0.5f)
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
