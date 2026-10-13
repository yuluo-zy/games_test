// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/lilypad.raster.hlsl | F-V | bins: _lilypad.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Lilypad: per-instance Reed affine xform from instance_buffer; the vertex is
// bent by a water-noise displacement (waves.inc), a value-noise wobble along the
// local x axis, and grass_object_push placement height; ps discards by the
// alpha texture and packs the solid gbuffer.

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
uint2 lilypad_normal_dims;
uint2 lilypad_alpha_dims;
int bool_render_shadows;

// Reed instance transform (packed affine, 48 bytes) stored per instance.
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t17);
// (water_noise_texture moved before the waves.inc include — waves.inc helper bodies reference it)
Texture2D<float4> grass_object_push : register(t16);
Texture2D<float4> lilypad_alpha : register(t21);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

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
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = wpos;
    o.C3 = color;
    o.C4 = (vin.Vertex_Position.xz * 1.5f) + 0.5f;
    o.C5 = randv;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (lilypad_alpha.SampleBias(sampler_llr, pin.C4, global_mip_bias).x < lerp(0.0f, 0.9f, pin.C5))
    {
        discard;
    }
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
// lifted from _lilypad.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((((_211.x * 20.0f) - (_211.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool _112;
    if (frame_constants_glade_theme_id == 4u)
    {
        _112 = in_var_C2.y > 0.0f;
    }
    else
    {
        _112 = false;
    }
    float3 _120;
    if (_112)
    {
        _120 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _120 = in_var_C3;
    }
    float3 _122 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _144 = sqrt(clamp(_120, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_122.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_122.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_122.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_144.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_144.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_144.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lilypad.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((((_211.x * 20.0f) - (_211.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lilypad.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((spv_fmsub(_211.x, _211.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool _112;
    if (frame_constants_glade_theme_id == 4u)
    {
        _112 = in_var_C2.y > 0.0f;
    }
    else
    {
        _112 = false;
    }
    float3 _120;
    if (_112)
    {
        _120 = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        _120 = in_var_C3;
    }
    float3 _122 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _144 = sqrt(clamp(_120, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_122.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_122.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_122.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_144.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_144.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_144.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lilypad.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _155;

static const float _157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _259;
    Affine3Packed _186;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        _186.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _205 = transpose(float4x3(float3(_186.data[0], _186.data[1], _186.data[2]), float3(_186.data[3], _186.data[4], _186.data[5]), float3(_186.data[6], _186.data[7], _186.data[8]), float3(_186.data[9], _186.data[10], _186.data[11])));
    float3 _210 = mul(_205, float4(in_var_Vertex_Position, 1.0f));
    float3 _211 = mul(_205, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _220 = uint(((spv_fmsub(_211.x, _211.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _222 = _220 + (_220 << 10u);
    uint _224 = _222 ^ (_222 >> 6u);
    uint _226 = _224 + (_224 << 3u);
    uint _228 = _226 ^ (_226 >> 11u);
    float _234 = asfloat(((_228 + (_228 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _237 = _210.y + lerp(-0.02500000037252902984619140625f, 0.0f, _234);
    float3 _238 = _210;
    _238.y = _237;
    float2 _239 = _211.xz;
    float4 _245 = grass_object_push.SampleLevel(sampler_default, (_239 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint _437;
    uint _438;
    bool _439;
    bool _440;
    float3 _441;
    do
    {
        _259 = frame_constants_glade_theme_id == 4u;
        if (_259)
        {
            _437 = _155;
            _438 = _155;
            _439 = false;
            _440 = false;
            _441 = 0.0f.xxx;
            break;
        }
        float _267 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint _272;
        uint _274;
        bool _276;
        bool _278;
        _272 = _155;
        _274 = _155;
        _276 = false;
        _278 = false;
        float2 _181[4];
        for (uint _269 = 0u; _269 < 4u; )
        {
            float _284 = float(_159[_269]);
            int _286 = int(floor(_284));
            float _287 = frac(_284);
            uint _288 = uint(_286);
            uint _290 = _288 + (_288 << 10u);
            uint _292 = _290 ^ (_290 >> 6u);
            uint _294 = _292 + (_292 << 3u);
            uint _296 = _294 ^ (_294 >> 11u);
            bool _299 = _278 ? _278 : true;
            uint _300 = _278 ? _274 : 1664525u;
            bool _301 = _276 ? _276 : true;
            uint _302 = _276 ? _272 : 1013904223u;
            uint _306 = (((_288 * _300) + (_296 + (_296 << 15u))) + _302) * _300;
            uint _308 = _306 ^ (_306 >> 11u);
            uint _311 = _308 ^ ((_308 << 7u) & 2636928640u);
            uint _314 = _311 ^ ((_311 << 15u) & 4022730752u);
            uint _316 = _314 ^ (_314 >> 18u);
            uint _318 = _316 + (_316 << 10u);
            uint _320 = _318 ^ (_318 >> 6u);
            uint _322 = _320 + (_320 << 3u);
            uint _324 = _322 ^ (_322 >> 11u);
            uint _337 = uint(_286 + 1);
            uint _339 = _337 + (_337 << 10u);
            uint _341 = _339 ^ (_339 >> 6u);
            uint _343 = _341 + (_341 << 3u);
            uint _345 = _343 ^ (_343 >> 11u);
            uint _275 = _299 ? _300 : 1664525u;
            uint _273 = _301 ? _302 : 1013904223u;
            uint _351 = (((_337 * _275) + (_345 + (_345 << 15u))) + _273) * _275;
            uint _353 = _351 ^ (_351 >> 11u);
            uint _356 = _353 ^ ((_353 << 7u) & 2636928640u);
            uint _359 = _356 ^ ((_356 << 15u) & 4022730752u);
            uint _361 = _359 ^ (_359 >> 18u);
            uint _363 = _361 + (_361 << 10u);
            uint _365 = _363 ^ (_363 >> 6u);
            uint _367 = _365 + (_365 << 3u);
            uint _369 = _367 ^ (_367 >> 11u);
            float _384 = (_287 * _287) * (3.0f - (2.0f * _287));
            _181[_269] = (normalize(((float2(asfloat((_316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_324 + (_324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _384)) + (float2(asfloat((_361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_369 + (_369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _384)) - 0.5f.xx) * _267) * 3.0f;
            _269++;
            _272 = _273;
            _274 = _275;
            _276 = _301 ? _301 : true;
            _278 = _299 ? _299 : true;
            continue;
        }
        float3 _395;
        _395 = 0.0f.xxx;
        for (uint _398 = 0u; _398 < 1u; )
        {
            uint _399 = _398 + 1u;
            float _403 = float(_399) * 10.16640758514404296875f;
            float _404 = cos(_403);
            float _405 = sin(_403);
            float2x2 _409 = float2x2(float2(_404, _405), float2(-_405, _404));
            float2 _436 = _395.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_409, (_239 * _157[_398]) + _181[_398]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _158[_398]) * float2(1.0f, _157[_398] * 0.01200000010430812835693359375f).xyy).yz, _409);
            _395 = float3(_395.x, _436.x, _436.y);
            _398 = _399;
            continue;
        }
        _437 = _272;
        _438 = _274;
        _439 = _276;
        _440 = _278;
        _441 = _395;
        break;
    } while(false);
    float2 _447 = _238.xz + (((float3(_245.x, 0.0f, _245.y) * ((clamp(_245.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_441.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 _448 = float3(_447.x, _238.y, _447.y);
    float _450 = in_var_Vertex_Position.y + 0.5f;
    float2 _461 = (_239 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _463 = int2(floor(_461));
    float2 _464 = frac(_461);
    uint2 _465 = uint2(_463);
    uint _467 = _465.y;
    uint _469 = _467 + (_467 << 10u);
    uint _471 = _469 ^ (_469 >> 6u);
    uint _473 = _471 + (_471 << 3u);
    uint _475 = _473 ^ (_473 >> 11u);
    bool _478 = _440 ? _440 : true;
    uint _479 = _440 ? _438 : 1664525u;
    bool _480 = _439 ? _439 : true;
    uint _481 = _439 ? _437 : 1013904223u;
    uint _485 = (((_465.x * _479) + (_475 + (_475 << 15u))) + _481) * _479;
    uint _487 = _485 ^ (_485 >> 11u);
    uint _490 = _487 ^ ((_487 << 7u) & 2636928640u);
    uint _493 = _490 ^ ((_490 << 15u) & 4022730752u);
    uint2 _501 = uint2(_463 + int2(1, 0));
    uint _503 = _501.y;
    uint _505 = _503 + (_503 << 10u);
    uint _507 = _505 ^ (_505 >> 6u);
    uint _509 = _507 + (_507 << 3u);
    uint _511 = _509 ^ (_509 >> 11u);
    bool _514 = _478 ? _478 : true;
    uint _515 = _478 ? _479 : 1664525u;
    bool _516 = _480 ? _480 : true;
    uint _517 = _480 ? _481 : 1013904223u;
    uint _521 = (((_501.x * _515) + (_511 + (_511 << 15u))) + _517) * _515;
    uint _523 = _521 ^ (_521 >> 11u);
    uint _526 = _523 ^ ((_523 << 7u) & 2636928640u);
    uint _529 = _526 ^ ((_526 << 15u) & 4022730752u);
    uint2 _537 = uint2(_463 + int2(0, 1));
    uint _539 = _537.y;
    uint _541 = _539 + (_539 << 10u);
    uint _543 = _541 ^ (_541 >> 6u);
    uint _545 = _543 + (_543 << 3u);
    uint _547 = _545 ^ (_545 >> 11u);
    uint _551 = _514 ? _515 : 1664525u;
    uint _553 = _516 ? _517 : 1013904223u;
    uint _557 = (((_537.x * _551) + (_547 + (_547 << 15u))) + _553) * _551;
    uint _559 = _557 ^ (_557 >> 11u);
    uint _562 = _559 ^ ((_559 << 7u) & 2636928640u);
    uint _565 = _562 ^ ((_562 << 15u) & 4022730752u);
    uint2 _573 = uint2(_463 + int2(1, 1));
    uint _575 = _573.y;
    uint _577 = _575 + (_575 << 10u);
    uint _579 = _577 ^ (_577 >> 6u);
    uint _581 = _579 + (_579 << 3u);
    uint _583 = _581 ^ (_581 >> 11u);
    uint _586 = (_514 ? _514 : true) ? _551 : 1664525u;
    uint _591 = (((_573.x * _586) + (_583 + (_583 << 15u))) + ((_516 ? _516 : true) ? _553 : 1013904223u)) * _586;
    uint _593 = _591 ^ (_591 >> 11u);
    uint _596 = _593 ^ ((_593 << 7u) & 2636928640u);
    uint _599 = _596 ^ ((_596 << 15u) & 4022730752u);
    float2 _609 = (_464 * _464) * (3.0f.xx - (_464 * 2.0f));
    float _610 = _609.x;
    float _611 = 1.0f - _610;
    float _613 = _609.y;
    uint _631 = uint(int(_234));
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    float _650 = _447.x - (((((_450 * _450) * (sin((((((1.0f - _613) * (((asfloat(((_493 ^ (_493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) + ((asfloat(((_529 ^ (_529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610))) + (((asfloat(((_565 ^ (_565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _611) * _613)) + (((asfloat(((_599 ^ (_599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _610) * _613)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (_259 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _448.x = _650;
    float3 _660 = lerp(1.2000000476837158203125f, 2.0f, _234).xxx;
    float3 _661 = pow(in_var_Vertex_Color, _660);
    float3 _673;
    if (_259)
    {
        float3 _672;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            _672 = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), _660);
        }
        else
        {
            _672 = _661;
        }
        _673 = _672;
    }
    else
    {
        _673 = _661;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_650, _237, _447.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _448;
    out_var_C3 = _673;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = _234;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
