// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/reed.raster.hlsl | F-V | bins: _reed.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Reed: like lilypad (Reed instance xform + water displacement + value-noise
// wobble) but with a vertex normal passed to the fragment stage; ps packs the
// solid gbuffer with the theme-4 snow tint.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/utils.hlsl"
#include "inc/wind.hlsl"
#include "water/water_object_push.inc.hlsl"
Texture2D<float4> water_noise_texture : register(t15);
#include "water/waves.inc.hlsl"
#include "inc/pack_unpack.hlsl"
Texture2D<float4> terrain_heightmap;
#include "inc/terrain_heightmap.inc.hlsl"
// Registerless global for inc/terrain_heightmap.inc.hlsl's terrain_height()
// helper; unused here, so dxc DCEs it (no binding emitted).
#include "inc/material/solid_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/material/plant_material.inc.hlsl"

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

// Vegetation shared globals (dims for the textures this shader binds).
uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
int bool_render_shadows;

// Reed instance transform (packed affine, 48 bytes) stored per instance.
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t17);
// (water_noise_texture moved before the waves.inc include — waves.inc helper bodies reference it)
Texture2D<float4> grass_object_push : register(t16);

// Water displacement from waves.inc: per-octave jitter + a sampled wave field.
// The glade-theme-4 case returns immediately (no water motion).
float3 water_displacement(float2 origin_xz, float t)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter[4];
    static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
    static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
    static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };
    uint jit_a = 0u;
    uint jit_b = 0u;
    bool jit_c = false;
    bool jit_d = false;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(wave_seeds[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint s = uint(cell);
        uint s5 = s + (s << 10u);
        s5 ^= (s5 >> 6u);
        s5 += (s5 << 3u);
        s5 ^= (s5 >> 11u);
        bool use_a = jit_d ? jit_d : true;
        uint A = jit_d ? jit_b : 1664525u;
        bool use_b = jit_c ? jit_c : true;
        uint B = jit_c ? jit_a : 1013904223u;
        uint h0 = ((s * A) + (s5 + (s5 << 15u)) + B) * A;
        h0 ^= (h0 >> 11u);
        h0 ^= ((h0 << 7u) & 2636928640u);
        h0 ^= ((h0 << 15u) & 4022730752u);
        uint s1 = s + 1u;
        uint s1_5 = s1 + (s1 << 10u);
        s1_5 ^= (s1_5 >> 6u);
        s1_5 += (s1_5 << 3u);
        s1_5 ^= (s1_5 >> 11u);
        uint A1 = use_a ? A : 1664525u;
        uint B1 = use_b ? B : 1013904223u;
        uint h1 = ((s1 * A1) + (s1_5 + (s1_5 << 15u)) + B1) * A1;
        h1 ^= (h1 >> 11u);
        h1 ^= ((h1 << 7u) & 2636928640u);
        h1 ^= ((h1 << 15u) & 4022730752u);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(h0), rand01(hash(h0 ^ (h0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(h1), rand01(hash(h1 ^ (h1 >> 18u))));
        jitter[oct] = normalize(((a * (1.0f - u)) + (b * u)) - 0.5f) * t * 3.0f;
        jit_a = B1;
        jit_b = A1;
        jit_c = use_b ? use_b : true;
        jit_d = use_a ? use_a : true;
    }
    float3 acc = 0.0f;
    for (uint wave = 0u; wave < 1u; wave++)
    {
        float angle = float(wave + 1u) * (GOLDEN_RATIO * TWO_PI);
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float2 wave_pos = mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.012f;
        float3 sample = (water_noise_texture.SampleLevel(sampler_llr, wave_pos, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f))
                      + float3(-0.5f, -1.0f, -1.0f);
        float3 disp = sample * wave_amps[wave] * float3(1.0f, wave_scales[wave] * 0.012f, wave_scales[wave] * 0.012f);
        acc = float3(acc.x, acc.yz + mul(disp.yz, rot));
    }
    return acc;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    Affine3Packed xform = instance_buffer[vin.instance_id].xform;
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]),
                                    float3(xform.data[3], xform.data[4], xform.data[5]),
                                    float3(xform.data[6], xform.data[7], xform.data[8]),
                                    float3(xform.data[9], xform.data[10], xform.data[11])));

    float3 pos = mul(m, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));

    uint seed = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    float randv = rand01(hash(seed));

    pos.y = pos.y + lerp(-0.025f, 0.0f, randv);
    float2 origin_xz = origin.xz;
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    float2 disp2 = pos.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f) + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos.y, disp2.y);

    float wobble_k = vin.Vertex_Position.y + 0.5f;
    float2 wobble_pos = (origin_xz * 0.1f) + (float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    float2 wobble_u = wobble_frac * wobble_frac * (3.0f - 2.0f * wobble_frac);
    float n00 = incb_perlin_hash01(hash(uint2(wobble_cell)));
    float n10 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 1))));
    float wobble_noise = (1.0f - wobble_u.y) * ((n00 * (1.0f - wobble_u.x)) + (n10 * wobble_u.x))
                       + (n01 * (1.0f - wobble_u.x)) * wobble_u.y
                       + (n11 * wobble_u.x) * wobble_u.y;
    float wobble_rand = lerp(0.7f, 1.0f, rand01(hash(uint(int(randv)))));
    wpos.x = wpos.x - ((wobble_k * wobble_k) * sin((wobble_noise * PI) - HALF_PI + 0.3f) * 0.7f * (glade_theme_id == 4u ? 0.025f : 0.05f)) * wobble_rand;

    float3 color_pow = lerp(1.2f, 2.0f, randv).xxx;
    float3 vertex_color = pow(vin.Vertex_Color, color_pow);
    float3 color;
    if (glade_theme_id == 4u)
    {
        if (vin.Vertex_Color.y > 0.05f)
        {
            color = pow(float3(0.15f, 0.09f, 0.03f), color_pow);
        }
        else
        {
            color = vertex_color;
        }
    }
    else
    {
        color = vertex_color;
    }

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos.x, pos.y, disp2.y, 1.0f));
    o.C1 = normalize(mul(m, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wpos;
    o.C3 = color;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool snow_branch = false;
    if (glade_theme_id == 4u)
    {
        snow_branch = pin.C2.y > 0.0f;
    }
    float3 albedo = pin.C3;
    if (snow_branch)
    {
        albedo = lerp(pin.C3, 0.7f.xxx, smoothstep(-0.2f, 1.0f, clamp(pin.C1.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    uint albedo_word = uint(clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

static uint _150;

static const float _152[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _153[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _154[4] = { 17u, 18u, 20u, 21u };

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
    uint current_view_index : packoffset(c0);
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _255;
    Affine3Packed _182;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _182.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _201 = transpose(float4x3(float3(_182.data[0], _182.data[1], _182.data[2]), float3(_182.data[3], _182.data[4], _182.data[5]), float3(_182.data[6], _182.data[7], _182.data[8]), float3(_182.data[9], _182.data[10], _182.data[11])));
    float3 _206 = mul(_201, float4(in_var_Vertex_Position, 1.0f));
    float3 _207 = mul(_201, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _216 = uint(((((_207.x * 20.0f) - (_207.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    float _230 = asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _233 = _206.y + lerp(-0.02500000037252902984619140625f, 0.0f, _230);
    float3 _234 = _206;
    _234.y = _233;
    float2 _235 = _207.xz;
    float4 _241 = grass_object_push.SampleLevel(sampler_default, (_235 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _433;
    uint _434;
    bool _435;
    bool _436;
    float3 _437;
    do
    {
        _255 = frame_constants_glade_theme_id == 4u;
        if (_255)
        {
            _433 = _150;
            _434 = _150;
            _435 = false;
            _436 = false;
            _437 = 0.0f.xxx;
            break;
        }
        float _263 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _268;
        uint _270;
        bool _272;
        bool _274;
        _268 = _150;
        _270 = _150;
        _272 = false;
        _274 = false;
        float2 _176[4];
        for (uint _265 = 0u; _265 < 4u; )
        {
            float _280 = float(_154[_265]);
            int _282 = int(floor(_280));
            float _283 = frac(_280);
            uint _284 = uint(_282);
            uint _286 = _284 + (_284 << 10u);
            uint _288 = _286 ^ (_286 >> 6u);
            uint _290 = _288 + (_288 << 3u);
            uint _292 = _290 ^ (_290 >> 11u);
            bool _295 = _274 ? _274 : true;
            uint _296 = _274 ? _270 : 1664525u;
            bool _297 = _272 ? _272 : true;
            uint _298 = _272 ? _268 : 1013904223u;
            uint _302 = (((_284 * _296) + (_292 + (_292 << 15u))) + _298) * _296;
            uint _304 = _302 ^ (_302 >> 11u);
            uint _307 = _304 ^ ((_304 << 7u) & 2636928640u);
            uint _310 = _307 ^ ((_307 << 15u) & 4022730752u);
            uint _312 = _310 ^ (_310 >> 18u);
            uint _314 = _312 + (_312 << 10u);
            uint _316 = _314 ^ (_314 >> 6u);
            uint _318 = _316 + (_316 << 3u);
            uint _320 = _318 ^ (_318 >> 11u);
            uint _333 = uint(_282 + 1);
            uint _335 = _333 + (_333 << 10u);
            uint _337 = _335 ^ (_335 >> 6u);
            uint _339 = _337 + (_337 << 3u);
            uint _341 = _339 ^ (_339 >> 11u);
            uint _271 = _295 ? _296 : 1664525u;
            uint _269 = _297 ? _298 : 1013904223u;
            uint _347 = (((_333 * _271) + (_341 + (_341 << 15u))) + _269) * _271;
            uint _349 = _347 ^ (_347 >> 11u);
            uint _352 = _349 ^ ((_349 << 7u) & 2636928640u);
            uint _355 = _352 ^ ((_352 << 15u) & 4022730752u);
            uint _357 = _355 ^ (_355 >> 18u);
            uint _359 = _357 + (_357 << 10u);
            uint _361 = _359 ^ (_359 >> 6u);
            uint _363 = _361 + (_361 << 3u);
            uint _365 = _363 ^ (_363 >> 11u);
            float _380 = (_283 * _283) * (3.0f - (2.0f * _283));
            _176[_265] = (normalize(((float2(asfloat((_312 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_320 + (_320 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _380)) + (float2(asfloat((_357 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_365 + (_365 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _380)) - 0.5f.xx) * _263) * 3.0f;
            _265++;
            _268 = _269;
            _270 = _271;
            _272 = _297 ? _297 : true;
            _274 = _295 ? _295 : true;
            continue;
        }
        float3 _391;
        _391 = 0.0f.xxx;
        for (uint _394 = 0u; _394 < 1u; )
        {
            uint _395 = _394 + 1u;
            float _399 = float(_395) * 10.16640758514404296875f;
            float _400 = cos(_399);
            float _401 = sin(_399);
            float2x2 _405 = float2x2(float2(_400, _401), float2(-_401, _400));
            float2 _432 = _391.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_405, (_235 * _152[_394]) + _176[_394]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _153[_394]) * float2(1.0f, _152[_394] * 0.01200000010430812835693359375f).xyy).yz, _405);
            _391 = float3(_391.x, _432.x, _432.y);
            _394 = _395;
            continue;
        }
        _433 = _268;
        _434 = _270;
        _435 = _272;
        _436 = _274;
        _437 = _391;
        break;
    } while(false);
    float2 _443 = _234.xz + (((float3(_241.x, 0.0f, _241.y) * ((clamp(_241.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_437.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _444 = float3(_443.x, _234.y, _443.y);
    float _446 = in_var_Vertex_Position.y + 0.5f;
    float2 _457 = (_235 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _459 = int2(floor(_457));
    float2 _460 = frac(_457);
    uint2 _461 = uint2(_459);
    uint _463 = _461.y;
    uint _465 = _463 + (_463 << 10u);
    uint _467 = _465 ^ (_465 >> 6u);
    uint _469 = _467 + (_467 << 3u);
    uint _471 = _469 ^ (_469 >> 11u);
    bool _474 = _436 ? _436 : true;
    uint _475 = _436 ? _434 : 1664525u;
    bool _476 = _435 ? _435 : true;
    uint _477 = _435 ? _433 : 1013904223u;
    uint _481 = (((_461.x * _475) + (_471 + (_471 << 15u))) + _477) * _475;
    uint _483 = _481 ^ (_481 >> 11u);
    uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
    uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
    uint2 _497 = uint2(_459 + int2(1, 0));
    uint _499 = _497.y;
    uint _501 = _499 + (_499 << 10u);
    uint _503 = _501 ^ (_501 >> 6u);
    uint _505 = _503 + (_503 << 3u);
    uint _507 = _505 ^ (_505 >> 11u);
    bool _510 = _474 ? _474 : true;
    uint _511 = _474 ? _475 : 1664525u;
    bool _512 = _476 ? _476 : true;
    uint _513 = _476 ? _477 : 1013904223u;
    uint _517 = (((_497.x * _511) + (_507 + (_507 << 15u))) + _513) * _511;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint2 _533 = uint2(_459 + int2(0, 1));
    uint _535 = _533.y;
    uint _537 = _535 + (_535 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint _547 = _510 ? _511 : 1664525u;
    uint _549 = _512 ? _513 : 1013904223u;
    uint _553 = (((_533.x * _547) + (_543 + (_543 << 15u))) + _549) * _547;
    uint _555 = _553 ^ (_553 >> 11u);
    uint _558 = _555 ^ ((_555 << 7u) & 2636928640u);
    uint _561 = _558 ^ ((_558 << 15u) & 4022730752u);
    uint2 _569 = uint2(_459 + int2(1, 1));
    uint _571 = _569.y;
    uint _573 = _571 + (_571 << 10u);
    uint _575 = _573 ^ (_573 >> 6u);
    uint _577 = _575 + (_575 << 3u);
    uint _579 = _577 ^ (_577 >> 11u);
    uint _582 = (_510 ? _510 : true) ? _547 : 1664525u;
    uint _587 = (((_569.x * _582) + (_579 + (_579 << 15u))) + ((_512 ? _512 : true) ? _549 : 1013904223u)) * _582;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    float2 _605 = (_460 * _460) * (3.0f.xx - (_460 * 2.0f));
    float _606 = _605.x;
    float _607 = 1.0f - _606;
    float _609 = _605.y;
    uint _627 = uint(int(_230));
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    float _646 = _443.x - (((((_446 * _446) * (sin((((((1.0f - _609) * (((asfloat(((_489 ^ (_489 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) + ((asfloat(((_525 ^ (_525 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606))) + (((asfloat(((_561 ^ (_561 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) * _609)) + (((asfloat(((_595 ^ (_595 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606) * _609)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_255 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_635 + (_635 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _444.x = _646;
    float3 _656 = lerp(1.2000000476837158203125f, 2.0f, _230).xxx;
    float3 _657 = pow(in_var_Vertex_Color, _656);
    float3 _672;
    if (_255)
    {
        float3 _671;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _671 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _656);
        }
        else
        {
            _671 = _657;
        }
        _672 = _671;
    }
    else
    {
        _672 = _657;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_646, _233, _443.y, 1.0f));
    out_var_C1 = normalize(mul(_201, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _444;
    out_var_C3 = _672;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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


static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    bool _84;
    if (frame_constants_glade_theme_id == 4u)
    {
        _84 = in_var_C2.y > 0.0f;
    }
    else
    {
        _84 = false;
    }
    float3 _92;
    if (_84)
    {
        _92 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _92 = in_var_C3;
    }
    float3 _94 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _116 = sqrt(clamp(_92, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_94.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_94.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_94.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_116.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_116.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_116.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

static uint _150;

static const float _152[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _153[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _154[4] = { 17u, 18u, 20u, 21u };

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
    uint current_view_index : packoffset(c0);
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _255;
    Affine3Packed _182;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _182.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _201 = transpose(float4x3(float3(_182.data[0], _182.data[1], _182.data[2]), float3(_182.data[3], _182.data[4], _182.data[5]), float3(_182.data[6], _182.data[7], _182.data[8]), float3(_182.data[9], _182.data[10], _182.data[11])));
    float3 _206 = mul(_201, float4(in_var_Vertex_Position, 1.0f));
    float3 _207 = mul(_201, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _216 = uint(((((_207.x * 20.0f) - (_207.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    float _230 = asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _233 = _206.y + lerp(-0.02500000037252902984619140625f, 0.0f, _230);
    float3 _234 = _206;
    _234.y = _233;
    float2 _235 = _207.xz;
    float4 _241 = grass_object_push.SampleLevel(sampler_default, (_235 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _433;
    uint _434;
    bool _435;
    bool _436;
    float3 _437;
    do
    {
        _255 = frame_constants_glade_theme_id == 4u;
        if (_255)
        {
            _433 = _150;
            _434 = _150;
            _435 = false;
            _436 = false;
            _437 = 0.0f.xxx;
            break;
        }
        float _263 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _268;
        uint _270;
        bool _272;
        bool _274;
        _268 = _150;
        _270 = _150;
        _272 = false;
        _274 = false;
        float2 _176[4];
        for (uint _265 = 0u; _265 < 4u; )
        {
            float _280 = float(_154[_265]);
            int _282 = int(floor(_280));
            float _283 = frac(_280);
            uint _284 = uint(_282);
            uint _286 = _284 + (_284 << 10u);
            uint _288 = _286 ^ (_286 >> 6u);
            uint _290 = _288 + (_288 << 3u);
            uint _292 = _290 ^ (_290 >> 11u);
            bool _295 = _274 ? _274 : true;
            uint _296 = _274 ? _270 : 1664525u;
            bool _297 = _272 ? _272 : true;
            uint _298 = _272 ? _268 : 1013904223u;
            uint _302 = (((_284 * _296) + (_292 + (_292 << 15u))) + _298) * _296;
            uint _304 = _302 ^ (_302 >> 11u);
            uint _307 = _304 ^ ((_304 << 7u) & 2636928640u);
            uint _310 = _307 ^ ((_307 << 15u) & 4022730752u);
            uint _312 = _310 ^ (_310 >> 18u);
            uint _314 = _312 + (_312 << 10u);
            uint _316 = _314 ^ (_314 >> 6u);
            uint _318 = _316 + (_316 << 3u);
            uint _320 = _318 ^ (_318 >> 11u);
            uint _333 = uint(_282 + 1);
            uint _335 = _333 + (_333 << 10u);
            uint _337 = _335 ^ (_335 >> 6u);
            uint _339 = _337 + (_337 << 3u);
            uint _341 = _339 ^ (_339 >> 11u);
            uint _271 = _295 ? _296 : 1664525u;
            uint _269 = _297 ? _298 : 1013904223u;
            uint _347 = (((_333 * _271) + (_341 + (_341 << 15u))) + _269) * _271;
            uint _349 = _347 ^ (_347 >> 11u);
            uint _352 = _349 ^ ((_349 << 7u) & 2636928640u);
            uint _355 = _352 ^ ((_352 << 15u) & 4022730752u);
            uint _357 = _355 ^ (_355 >> 18u);
            uint _359 = _357 + (_357 << 10u);
            uint _361 = _359 ^ (_359 >> 6u);
            uint _363 = _361 + (_361 << 3u);
            uint _365 = _363 ^ (_363 >> 11u);
            float _380 = (_283 * _283) * (3.0f - (2.0f * _283));
            _176[_265] = (normalize(((float2(asfloat((_312 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_320 + (_320 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _380)) + (float2(asfloat((_357 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_365 + (_365 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _380)) - 0.5f.xx) * _263) * 3.0f;
            _265++;
            _268 = _269;
            _270 = _271;
            _272 = _297 ? _297 : true;
            _274 = _295 ? _295 : true;
            continue;
        }
        float3 _391;
        _391 = 0.0f.xxx;
        for (uint _394 = 0u; _394 < 1u; )
        {
            uint _395 = _394 + 1u;
            float _399 = float(_395) * 10.16640758514404296875f;
            float _400 = cos(_399);
            float _401 = sin(_399);
            float2x2 _405 = float2x2(float2(_400, _401), float2(-_401, _400));
            float2 _432 = _391.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_405, (_235 * _152[_394]) + _176[_394]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _153[_394]) * float2(1.0f, _152[_394] * 0.01200000010430812835693359375f).xyy).yz, _405);
            _391 = float3(_391.x, _432.x, _432.y);
            _394 = _395;
            continue;
        }
        _433 = _268;
        _434 = _270;
        _435 = _272;
        _436 = _274;
        _437 = _391;
        break;
    } while(false);
    float2 _443 = _234.xz + (((float3(_241.x, 0.0f, _241.y) * ((clamp(_241.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_437.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _444 = float3(_443.x, _234.y, _443.y);
    float _446 = in_var_Vertex_Position.y + 0.5f;
    float2 _457 = (_235 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _459 = int2(floor(_457));
    float2 _460 = frac(_457);
    uint2 _461 = uint2(_459);
    uint _463 = _461.y;
    uint _465 = _463 + (_463 << 10u);
    uint _467 = _465 ^ (_465 >> 6u);
    uint _469 = _467 + (_467 << 3u);
    uint _471 = _469 ^ (_469 >> 11u);
    bool _474 = _436 ? _436 : true;
    uint _475 = _436 ? _434 : 1664525u;
    bool _476 = _435 ? _435 : true;
    uint _477 = _435 ? _433 : 1013904223u;
    uint _481 = (((_461.x * _475) + (_471 + (_471 << 15u))) + _477) * _475;
    uint _483 = _481 ^ (_481 >> 11u);
    uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
    uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
    uint2 _497 = uint2(_459 + int2(1, 0));
    uint _499 = _497.y;
    uint _501 = _499 + (_499 << 10u);
    uint _503 = _501 ^ (_501 >> 6u);
    uint _505 = _503 + (_503 << 3u);
    uint _507 = _505 ^ (_505 >> 11u);
    bool _510 = _474 ? _474 : true;
    uint _511 = _474 ? _475 : 1664525u;
    bool _512 = _476 ? _476 : true;
    uint _513 = _476 ? _477 : 1013904223u;
    uint _517 = (((_497.x * _511) + (_507 + (_507 << 15u))) + _513) * _511;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint2 _533 = uint2(_459 + int2(0, 1));
    uint _535 = _533.y;
    uint _537 = _535 + (_535 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint _547 = _510 ? _511 : 1664525u;
    uint _549 = _512 ? _513 : 1013904223u;
    uint _553 = (((_533.x * _547) + (_543 + (_543 << 15u))) + _549) * _547;
    uint _555 = _553 ^ (_553 >> 11u);
    uint _558 = _555 ^ ((_555 << 7u) & 2636928640u);
    uint _561 = _558 ^ ((_558 << 15u) & 4022730752u);
    uint2 _569 = uint2(_459 + int2(1, 1));
    uint _571 = _569.y;
    uint _573 = _571 + (_571 << 10u);
    uint _575 = _573 ^ (_573 >> 6u);
    uint _577 = _575 + (_575 << 3u);
    uint _579 = _577 ^ (_577 >> 11u);
    uint _582 = (_510 ? _510 : true) ? _547 : 1664525u;
    uint _587 = (((_569.x * _582) + (_579 + (_579 << 15u))) + ((_512 ? _512 : true) ? _549 : 1013904223u)) * _582;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    float2 _605 = (_460 * _460) * (3.0f.xx - (_460 * 2.0f));
    float _606 = _605.x;
    float _607 = 1.0f - _606;
    float _609 = _605.y;
    uint _627 = uint(int(_230));
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    float _646 = _443.x - (((((_446 * _446) * (sin((((((1.0f - _609) * (((asfloat(((_489 ^ (_489 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) + ((asfloat(((_525 ^ (_525 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606))) + (((asfloat(((_561 ^ (_561 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) * _609)) + (((asfloat(((_595 ^ (_595 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606) * _609)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_255 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_635 + (_635 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _444.x = _646;
    float3 _656 = lerp(1.2000000476837158203125f, 2.0f, _230).xxx;
    float3 _657 = pow(in_var_Vertex_Color, _656);
    float3 _672;
    if (_255)
    {
        float3 _671;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _671 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _656);
        }
        else
        {
            _671 = _657;
        }
        _672 = _671;
    }
    else
    {
        _672 = _657;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_646, _233, _443.y, 1.0f));
    out_var_C1 = normalize(mul(_201, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _444;
    out_var_C3 = _672;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc factors (a*c) - (b*c) into (a-b)*c, which rounds differently from the original; precise temporaries keep both products.
template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

static uint _150;

static const float _152[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _153[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _154[4] = { 17u, 18u, 20u, 21u };

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
    uint current_view_index : packoffset(c0);
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _255;
    Affine3Packed _182;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _182.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _201 = transpose(float4x3(float3(_182.data[0], _182.data[1], _182.data[2]), float3(_182.data[3], _182.data[4], _182.data[5]), float3(_182.data[6], _182.data[7], _182.data[8]), float3(_182.data[9], _182.data[10], _182.data[11])));
    float3 _206 = mul(_201, float4(in_var_Vertex_Position, 1.0f));
    float3 _207 = mul(_201, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _216 = uint(((spv_fmsub(_207.x, _207.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    float _230 = asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _233 = _206.y + lerp(-0.02500000037252902984619140625f, 0.0f, _230);
    float3 _234 = _206;
    _234.y = _233;
    float2 _235 = _207.xz;
    float4 _241 = grass_object_push.SampleLevel(sampler_default, (_235 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _433;
    uint _434;
    bool _435;
    bool _436;
    float3 _437;
    do
    {
        _255 = frame_constants_glade_theme_id == 4u;
        if (_255)
        {
            _433 = _150;
            _434 = _150;
            _435 = false;
            _436 = false;
            _437 = 0.0f.xxx;
            break;
        }
        float _263 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _268;
        uint _270;
        bool _272;
        bool _274;
        _268 = _150;
        _270 = _150;
        _272 = false;
        _274 = false;
        float2 _176[4];
        for (uint _265 = 0u; _265 < 4u; )
        {
            float _280 = float(_154[_265]);
            int _282 = int(floor(_280));
            float _283 = frac(_280);
            uint _284 = uint(_282);
            uint _286 = _284 + (_284 << 10u);
            uint _288 = _286 ^ (_286 >> 6u);
            uint _290 = _288 + (_288 << 3u);
            uint _292 = _290 ^ (_290 >> 11u);
            bool _295 = _274 ? _274 : true;
            uint _296 = _274 ? _270 : 1664525u;
            bool _297 = _272 ? _272 : true;
            uint _298 = _272 ? _268 : 1013904223u;
            uint _302 = (((_284 * _296) + (_292 + (_292 << 15u))) + _298) * _296;
            uint _304 = _302 ^ (_302 >> 11u);
            uint _307 = _304 ^ ((_304 << 7u) & 2636928640u);
            uint _310 = _307 ^ ((_307 << 15u) & 4022730752u);
            uint _312 = _310 ^ (_310 >> 18u);
            uint _314 = _312 + (_312 << 10u);
            uint _316 = _314 ^ (_314 >> 6u);
            uint _318 = _316 + (_316 << 3u);
            uint _320 = _318 ^ (_318 >> 11u);
            uint _333 = uint(_282 + 1);
            uint _335 = _333 + (_333 << 10u);
            uint _337 = _335 ^ (_335 >> 6u);
            uint _339 = _337 + (_337 << 3u);
            uint _341 = _339 ^ (_339 >> 11u);
            uint _271 = _295 ? _296 : 1664525u;
            uint _269 = _297 ? _298 : 1013904223u;
            uint _347 = (((_333 * _271) + (_341 + (_341 << 15u))) + _269) * _271;
            uint _349 = _347 ^ (_347 >> 11u);
            uint _352 = _349 ^ ((_349 << 7u) & 2636928640u);
            uint _355 = _352 ^ ((_352 << 15u) & 4022730752u);
            uint _357 = _355 ^ (_355 >> 18u);
            uint _359 = _357 + (_357 << 10u);
            uint _361 = _359 ^ (_359 >> 6u);
            uint _363 = _361 + (_361 << 3u);
            uint _365 = _363 ^ (_363 >> 11u);
            float _380 = (_283 * _283) * (3.0f - (2.0f * _283));
            _176[_265] = (normalize(((float2(asfloat((_312 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_320 + (_320 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _380)) + (float2(asfloat((_357 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_365 + (_365 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _380)) - 0.5f.xx) * _263) * 3.0f;
            _265++;
            _268 = _269;
            _270 = _271;
            _272 = _297 ? _297 : true;
            _274 = _295 ? _295 : true;
            continue;
        }
        float3 _391;
        _391 = 0.0f.xxx;
        for (uint _394 = 0u; _394 < 1u; )
        {
            uint _395 = _394 + 1u;
            float _399 = float(_395) * 10.16640758514404296875f;
            float _400 = cos(_399);
            float _401 = sin(_399);
            float2x2 _405 = float2x2(float2(_400, _401), float2(-_401, _400));
            float2 _432 = _391.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_405, (_235 * _152[_394]) + _176[_394]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _153[_394]) * float2(1.0f, _152[_394] * 0.01200000010430812835693359375f).xyy).yz, _405);
            _391 = float3(_391.x, _432.x, _432.y);
            _394 = _395;
            continue;
        }
        _433 = _268;
        _434 = _270;
        _435 = _272;
        _436 = _274;
        _437 = _391;
        break;
    } while(false);
    float2 _443 = _234.xz + (((float3(_241.x, 0.0f, _241.y) * ((clamp(_241.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_437.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _444 = float3(_443.x, _234.y, _443.y);
    float _446 = in_var_Vertex_Position.y + 0.5f;
    float2 _457 = (_235 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _459 = int2(floor(_457));
    float2 _460 = frac(_457);
    uint2 _461 = uint2(_459);
    uint _463 = _461.y;
    uint _465 = _463 + (_463 << 10u);
    uint _467 = _465 ^ (_465 >> 6u);
    uint _469 = _467 + (_467 << 3u);
    uint _471 = _469 ^ (_469 >> 11u);
    bool _474 = _436 ? _436 : true;
    uint _475 = _436 ? _434 : 1664525u;
    bool _476 = _435 ? _435 : true;
    uint _477 = _435 ? _433 : 1013904223u;
    uint _481 = (((_461.x * _475) + (_471 + (_471 << 15u))) + _477) * _475;
    uint _483 = _481 ^ (_481 >> 11u);
    uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
    uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
    uint2 _497 = uint2(_459 + int2(1, 0));
    uint _499 = _497.y;
    uint _501 = _499 + (_499 << 10u);
    uint _503 = _501 ^ (_501 >> 6u);
    uint _505 = _503 + (_503 << 3u);
    uint _507 = _505 ^ (_505 >> 11u);
    bool _510 = _474 ? _474 : true;
    uint _511 = _474 ? _475 : 1664525u;
    bool _512 = _476 ? _476 : true;
    uint _513 = _476 ? _477 : 1013904223u;
    uint _517 = (((_497.x * _511) + (_507 + (_507 << 15u))) + _513) * _511;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint2 _533 = uint2(_459 + int2(0, 1));
    uint _535 = _533.y;
    uint _537 = _535 + (_535 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint _547 = _510 ? _511 : 1664525u;
    uint _549 = _512 ? _513 : 1013904223u;
    uint _553 = (((_533.x * _547) + (_543 + (_543 << 15u))) + _549) * _547;
    uint _555 = _553 ^ (_553 >> 11u);
    uint _558 = _555 ^ ((_555 << 7u) & 2636928640u);
    uint _561 = _558 ^ ((_558 << 15u) & 4022730752u);
    uint2 _569 = uint2(_459 + int2(1, 1));
    uint _571 = _569.y;
    uint _573 = _571 + (_571 << 10u);
    uint _575 = _573 ^ (_573 >> 6u);
    uint _577 = _575 + (_575 << 3u);
    uint _579 = _577 ^ (_577 >> 11u);
    uint _582 = (_510 ? _510 : true) ? _547 : 1664525u;
    uint _587 = (((_569.x * _582) + (_579 + (_579 << 15u))) + ((_512 ? _512 : true) ? _549 : 1013904223u)) * _582;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    float2 _605 = (_460 * _460) * (3.0f.xx - (_460 * 2.0f));
    float _606 = _605.x;
    float _607 = 1.0f - _606;
    float _609 = _605.y;
    uint _627 = uint(int(_230));
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    float _646 = _443.x - (((((_446 * _446) * (sin((((((1.0f - _609) * (((asfloat(((_489 ^ (_489 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) + ((asfloat(((_525 ^ (_525 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606))) + (((asfloat(((_561 ^ (_561 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) * _609)) + (((asfloat(((_595 ^ (_595 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606) * _609)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_255 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_635 + (_635 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _444.x = _646;
    float3 _656 = lerp(1.2000000476837158203125f, 2.0f, _230).xxx;
    float3 _657 = pow(in_var_Vertex_Color, _656);
    float3 _672;
    if (_255)
    {
        float3 _671;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _671 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _656);
        }
        else
        {
            _671 = _657;
        }
        _672 = _671;
    }
    else
    {
        _672 = _657;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_646, _233, _443.y, 1.0f));
    out_var_C1 = normalize(mul(_201, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _444;
    out_var_C3 = _672;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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


static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    bool _84;
    if (frame_constants_glade_theme_id == 4u)
    {
        _84 = in_var_C2.y > 0.0f;
    }
    else
    {
        _84 = false;
    }
    float3 _92;
    if (_84)
    {
        _92 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _92 = in_var_C3;
    }
    float3 _94 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _116 = sqrt(clamp(_92, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_94.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_94.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_94.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_116.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_116.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_116.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc factors (a*c) - (b*c) into (a-b)*c, which rounds differently from the original; precise temporaries keep both products.
template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

struct Affine3Packed
{
    float data[12];
};

struct Reed
{
    Affine3Packed xform;
};

static uint _150;

static const float _152[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _153[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _154[4] = { 17u, 18u, 20u, 21u };

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
    uint current_view_index : packoffset(c0);
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _255;
    Affine3Packed _182;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _182.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _201 = transpose(float4x3(float3(_182.data[0], _182.data[1], _182.data[2]), float3(_182.data[3], _182.data[4], _182.data[5]), float3(_182.data[6], _182.data[7], _182.data[8]), float3(_182.data[9], _182.data[10], _182.data[11])));
    float3 _206 = mul(_201, float4(in_var_Vertex_Position, 1.0f));
    float3 _207 = mul(_201, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _216 = uint(((spv_fmsub(_207.x, _207.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    float _230 = asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _233 = _206.y + lerp(-0.02500000037252902984619140625f, 0.0f, _230);
    float3 _234 = _206;
    _234.y = _233;
    float2 _235 = _207.xz;
    float4 _241 = grass_object_push.SampleLevel(sampler_default, (_235 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _433;
    uint _434;
    bool _435;
    bool _436;
    float3 _437;
    do
    {
        _255 = frame_constants_glade_theme_id == 4u;
        if (_255)
        {
            _433 = _150;
            _434 = _150;
            _435 = false;
            _436 = false;
            _437 = 0.0f.xxx;
            break;
        }
        float _263 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _268;
        uint _270;
        bool _272;
        bool _274;
        _268 = _150;
        _270 = _150;
        _272 = false;
        _274 = false;
        float2 _176[4];
        for (uint _265 = 0u; _265 < 4u; )
        {
            float _280 = float(_154[_265]);
            int _282 = int(floor(_280));
            float _283 = frac(_280);
            uint _284 = uint(_282);
            uint _286 = _284 + (_284 << 10u);
            uint _288 = _286 ^ (_286 >> 6u);
            uint _290 = _288 + (_288 << 3u);
            uint _292 = _290 ^ (_290 >> 11u);
            bool _295 = _274 ? _274 : true;
            uint _296 = _274 ? _270 : 1664525u;
            bool _297 = _272 ? _272 : true;
            uint _298 = _272 ? _268 : 1013904223u;
            uint _302 = (((_284 * _296) + (_292 + (_292 << 15u))) + _298) * _296;
            uint _304 = _302 ^ (_302 >> 11u);
            uint _307 = _304 ^ ((_304 << 7u) & 2636928640u);
            uint _310 = _307 ^ ((_307 << 15u) & 4022730752u);
            uint _312 = _310 ^ (_310 >> 18u);
            uint _314 = _312 + (_312 << 10u);
            uint _316 = _314 ^ (_314 >> 6u);
            uint _318 = _316 + (_316 << 3u);
            uint _320 = _318 ^ (_318 >> 11u);
            uint _333 = uint(_282 + 1);
            uint _335 = _333 + (_333 << 10u);
            uint _337 = _335 ^ (_335 >> 6u);
            uint _339 = _337 + (_337 << 3u);
            uint _341 = _339 ^ (_339 >> 11u);
            uint _271 = _295 ? _296 : 1664525u;
            uint _269 = _297 ? _298 : 1013904223u;
            uint _347 = (((_333 * _271) + (_341 + (_341 << 15u))) + _269) * _271;
            uint _349 = _347 ^ (_347 >> 11u);
            uint _352 = _349 ^ ((_349 << 7u) & 2636928640u);
            uint _355 = _352 ^ ((_352 << 15u) & 4022730752u);
            uint _357 = _355 ^ (_355 >> 18u);
            uint _359 = _357 + (_357 << 10u);
            uint _361 = _359 ^ (_359 >> 6u);
            uint _363 = _361 + (_361 << 3u);
            uint _365 = _363 ^ (_363 >> 11u);
            float _380 = (_283 * _283) * (3.0f - (2.0f * _283));
            _176[_265] = (normalize(((float2(asfloat((_312 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_320 + (_320 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _380)) + (float2(asfloat((_357 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_365 + (_365 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _380)) - 0.5f.xx) * _263) * 3.0f;
            _265++;
            _268 = _269;
            _270 = _271;
            _272 = _297 ? _297 : true;
            _274 = _295 ? _295 : true;
            continue;
        }
        float3 _391;
        _391 = 0.0f.xxx;
        for (uint _394 = 0u; _394 < 1u; )
        {
            uint _395 = _394 + 1u;
            float _399 = float(_395) * 10.16640758514404296875f;
            float _400 = cos(_399);
            float _401 = sin(_399);
            float2x2 _405 = float2x2(float2(_400, _401), float2(-_401, _400));
            float2 _432 = _391.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_405, (_235 * _152[_394]) + _176[_394]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _153[_394]) * float2(1.0f, _152[_394] * 0.01200000010430812835693359375f).xyy).yz, _405);
            _391 = float3(_391.x, _432.x, _432.y);
            _394 = _395;
            continue;
        }
        _433 = _268;
        _434 = _270;
        _435 = _272;
        _436 = _274;
        _437 = _391;
        break;
    } while(false);
    float2 _443 = _234.xz + (((float3(_241.x, 0.0f, _241.y) * ((clamp(_241.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_437.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _444 = float3(_443.x, _234.y, _443.y);
    float _446 = in_var_Vertex_Position.y + 0.5f;
    float2 _457 = (_235 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _459 = int2(floor(_457));
    float2 _460 = frac(_457);
    uint2 _461 = uint2(_459);
    uint _463 = _461.y;
    uint _465 = _463 + (_463 << 10u);
    uint _467 = _465 ^ (_465 >> 6u);
    uint _469 = _467 + (_467 << 3u);
    uint _471 = _469 ^ (_469 >> 11u);
    bool _474 = _436 ? _436 : true;
    uint _475 = _436 ? _434 : 1664525u;
    bool _476 = _435 ? _435 : true;
    uint _477 = _435 ? _433 : 1013904223u;
    uint _481 = (((_461.x * _475) + (_471 + (_471 << 15u))) + _477) * _475;
    uint _483 = _481 ^ (_481 >> 11u);
    uint _486 = _483 ^ ((_483 << 7u) & 2636928640u);
    uint _489 = _486 ^ ((_486 << 15u) & 4022730752u);
    uint2 _497 = uint2(_459 + int2(1, 0));
    uint _499 = _497.y;
    uint _501 = _499 + (_499 << 10u);
    uint _503 = _501 ^ (_501 >> 6u);
    uint _505 = _503 + (_503 << 3u);
    uint _507 = _505 ^ (_505 >> 11u);
    bool _510 = _474 ? _474 : true;
    uint _511 = _474 ? _475 : 1664525u;
    bool _512 = _476 ? _476 : true;
    uint _513 = _476 ? _477 : 1013904223u;
    uint _517 = (((_497.x * _511) + (_507 + (_507 << 15u))) + _513) * _511;
    uint _519 = _517 ^ (_517 >> 11u);
    uint _522 = _519 ^ ((_519 << 7u) & 2636928640u);
    uint _525 = _522 ^ ((_522 << 15u) & 4022730752u);
    uint2 _533 = uint2(_459 + int2(0, 1));
    uint _535 = _533.y;
    uint _537 = _535 + (_535 << 10u);
    uint _539 = _537 ^ (_537 >> 6u);
    uint _541 = _539 + (_539 << 3u);
    uint _543 = _541 ^ (_541 >> 11u);
    uint _547 = _510 ? _511 : 1664525u;
    uint _549 = _512 ? _513 : 1013904223u;
    uint _553 = (((_533.x * _547) + (_543 + (_543 << 15u))) + _549) * _547;
    uint _555 = _553 ^ (_553 >> 11u);
    uint _558 = _555 ^ ((_555 << 7u) & 2636928640u);
    uint _561 = _558 ^ ((_558 << 15u) & 4022730752u);
    uint2 _569 = uint2(_459 + int2(1, 1));
    uint _571 = _569.y;
    uint _573 = _571 + (_571 << 10u);
    uint _575 = _573 ^ (_573 >> 6u);
    uint _577 = _575 + (_575 << 3u);
    uint _579 = _577 ^ (_577 >> 11u);
    uint _582 = (_510 ? _510 : true) ? _547 : 1664525u;
    uint _587 = (((_569.x * _582) + (_579 + (_579 << 15u))) + ((_512 ? _512 : true) ? _549 : 1013904223u)) * _582;
    uint _589 = _587 ^ (_587 >> 11u);
    uint _592 = _589 ^ ((_589 << 7u) & 2636928640u);
    uint _595 = _592 ^ ((_592 << 15u) & 4022730752u);
    float2 _605 = (_460 * _460) * (3.0f.xx - (_460 * 2.0f));
    float _606 = _605.x;
    float _607 = 1.0f - _606;
    float _609 = _605.y;
    uint _627 = uint(int(_230));
    uint _629 = _627 + (_627 << 10u);
    uint _631 = _629 ^ (_629 >> 6u);
    uint _633 = _631 + (_631 << 3u);
    uint _635 = _633 ^ (_633 >> 11u);
    float _646 = _443.x - (((((_446 * _446) * (sin((((((1.0f - _609) * (((asfloat(((_489 ^ (_489 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) + ((asfloat(((_525 ^ (_525 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606))) + (((asfloat(((_561 ^ (_561 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _607) * _609)) + (((asfloat(((_595 ^ (_595 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _606) * _609)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_255 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_635 + (_635 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _444.x = _646;
    float3 _656 = lerp(1.2000000476837158203125f, 2.0f, _230).xxx;
    float3 _657 = pow(in_var_Vertex_Color, _656);
    float3 _672;
    if (_255)
    {
        float3 _671;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _671 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _656);
        }
        else
        {
            _671 = _657;
        }
        _672 = _671;
    }
    else
    {
        _672 = _657;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_646, _233, _443.y, 1.0f));
    out_var_C1 = normalize(mul(_201, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _444;
    out_var_C3 = _672;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
