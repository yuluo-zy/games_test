// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/lilypad_flower.raster.hlsl | F-V | bins: _lilypad_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Lilypad flower: Reed instance xform + water displacement; the vertex is offset
// by the grass_object_push height and the wave displacement. ps samples the
// flower texture, flips the normal toward the viewer, and packs the gbuffer.

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
uint2 lilypad_flower_texture_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
int bool_render_shadows;

// Reed instance transform (packed affine, 48 bytes) stored per instance.
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t18);
// (water_noise_texture moved before the waves.inc include — waves.inc helper bodies reference it)
Texture2D<float4> grass_object_push : register(t16);
Texture2D<float4> lilypad_flower_texture : register(t17);

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
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
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

    pos.y = pos.y + lerp(-0.05f, 0.05f, randv);
    float2 origin_xz = origin.xz;
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    float3 wpos = pos + ((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f)
                 + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f));

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos, 1.0f));
    o.C1 = normalize(mul(m, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wpos;
    o.C3 = wpos.xz * 0.1f;
    o.C4 = vin.Vertex_Color;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float4 flower = lilypad_flower_texture.Sample(sampler_llr, pin.C3);
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 albedo = lerp(lerp(flower.xyz, float3(0.25f, 0.1f, 0.2f), (1.0f - smoothstep(0.05f, 0.1f, pin.C2.y)).xxx), pin.C4,
                         (1.0f - smoothstep(-0.05f, 0.05f, pin.C2.y)).xxx);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(pin.C4 * 0.3f, 0.7f.xxx, (smoothstep(0.0f, 0.9f, pin.C2.y) * 0.3f).xxx);
    }
    float3 normal = pin.C1;
    if (dot(normalize(cam_pos - pin.C2), pin.C1) <= 0.0f)
    {
        normal = -pin.C1;
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f;
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
// lifted from _lilypad_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _145;

static const float _147[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _148[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _149[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_buffer : register(t18, space0);
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
static float2 out_var_C3;
static float3 out_var_C4;

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
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _169;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _169.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _188 = transpose(float4x3(float3(_169.data[0], _169.data[1], _169.data[2]), float3(_169.data[3], _169.data[4], _169.data[5]), float3(_169.data[6], _169.data[7], _169.data[8]), float3(_169.data[9], _169.data[10], _169.data[11])));
    float3 _193 = mul(_188, float4(in_var_Vertex_Position, 1.0f));
    float3 _194 = mul(_188, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _203 = uint(((((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _205 = _203 + (_203 << 10u);
    uint _207 = _205 ^ (_205 >> 6u);
    uint _209 = _207 + (_207 << 3u);
    uint _211 = _209 ^ (_209 >> 11u);
    _193.y = _193.y + lerp(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, asfloat(((_211 + (_211 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float2 _222 = _194.xz;
    float4 _228 = grass_object_push.SampleLevel(sampler_default, (_222 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _424;
    do
    {
        if (frame_constants_glade_theme_id == 4u)
        {
            _424 = 0.0f.xxx;
            break;
        }
        float _250 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float2 _163[4];
        uint _252 = 0u;
        uint _255 = 0u;
        uint _257 = 0u;
        bool _259 = false;
        bool _261 = false;
        for (; _252 < 4u; )
        {
            float _267 = float(_149[_252]);
            int _269 = int(floor(_267));
            float _270 = frac(_267);
            uint _271 = uint(_269);
            uint _273 = _271 + (_271 << 10u);
            uint _275 = _273 ^ (_273 >> 6u);
            uint _277 = _275 + (_275 << 3u);
            uint _279 = _277 ^ (_277 >> 11u);
            bool _282 = _261 ? _261 : true;
            uint _283 = _261 ? _257 : 1664525u;
            bool _284 = _259 ? _259 : true;
            uint _285 = _259 ? _255 : 1013904223u;
            uint _289 = (((_271 * _283) + (_279 + (_279 << 15u))) + _285) * _283;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint _299 = _297 ^ (_297 >> 18u);
            uint _301 = _299 + (_299 << 10u);
            uint _303 = _301 ^ (_301 >> 6u);
            uint _305 = _303 + (_303 << 3u);
            uint _307 = _305 ^ (_305 >> 11u);
            uint _320 = uint(_269 + 1);
            uint _322 = _320 + (_320 << 10u);
            uint _324 = _322 ^ (_322 >> 6u);
            uint _326 = _324 + (_324 << 3u);
            uint _328 = _326 ^ (_326 >> 11u);
            uint _258 = _282 ? _283 : 1664525u;
            uint _256 = _284 ? _285 : 1013904223u;
            uint _334 = (((_320 * _258) + (_328 + (_328 << 15u))) + _256) * _258;
            uint _336 = _334 ^ (_334 >> 11u);
            uint _339 = _336 ^ ((_336 << 7u) & 2636928640u);
            uint _342 = _339 ^ ((_339 << 15u) & 4022730752u);
            uint _344 = _342 ^ (_342 >> 18u);
            uint _346 = _344 + (_344 << 10u);
            uint _348 = _346 ^ (_346 >> 6u);
            uint _350 = _348 + (_348 << 3u);
            uint _352 = _350 ^ (_350 >> 11u);
            float _367 = (_270 * _270) * (3.0f - (2.0f * _270));
            _163[_252] = (normalize(((float2(asfloat((_299 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_307 + (_307 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _367)) + (float2(asfloat((_344 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_352 + (_352 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _367)) - 0.5f.xx) * _250) * 3.0f;
            _252++;
            _255 = _256;
            _257 = _258;
            _259 = _284 ? _284 : true;
            _261 = _282 ? _282 : true;
            continue;
        }
        float3 _378;
        _378 = 0.0f.xxx;
        for (uint _381 = 0u; _381 < 1u; )
        {
            uint _382 = _381 + 1u;
            float _386 = float(_382) * 10.16640758514404296875f;
            float _387 = cos(_386);
            float _388 = sin(_386);
            float2x2 _392 = float2x2(float2(_387, _388), float2(-_388, _387));
            float3 _415 = (((water_noise_texture.SampleLevel(sampler_llr, mul(_392, (_222 * _147[_381]) + _163[_381]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _148[_381]) * float2(1.0f, _147[_381] * 0.01200000010430812835693359375f).xyy;
            float3 _419 = _378;
            _419.x = _378.x + _415.x;
            float2 _423 = _419.yz + mul(_415.yz, _392);
            _378 = float3(_419.x, _423.x, _423.y);
            _381 = _382;
            continue;
        }
        _424 = _378;
        break;
    } while(false);
    float3 _428 = _193 + (((float3(_228.x, 0.0f, _228.y) * ((clamp(_228.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_424.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_428, 1.0f));
    out_var_C1 = normalize(mul(_188, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _428;
    out_var_C3 = _428.xz * 0.100000001490116119384765625f;
    out_var_C4 = in_var_Vertex_Color;
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
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_flower_texture : register(t17, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _104 = lilypad_flower_texture.Sample(sampler_llr, in_var_C3);
    float4 _119 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _136;
    if (frame_constants_glade_theme_id == 4u)
    {
        _136 = lerp(in_var_C4 * 0.300000011920928955078125f, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 0.89999997615814208984375f, in_var_C2.y) * 0.300000011920928955078125f).xxx);
    }
    else
    {
        _136 = lerp(lerp(_104.xyz, float3(0.25f, 0.100000001490116119384765625f, 0.20000000298023223876953125f), (1.0f - smoothstep(0.0500000007450580596923828125f, 0.100000001490116119384765625f, in_var_C2.y)).xxx), in_var_C4, (1.0f - smoothstep(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, in_var_C2.y)).xxx);
    }
    float3 _142;
    if (dot(normalize((_119.xyz / _119.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        _142 = -in_var_C1;
    }
    else
    {
        _142 = in_var_C1;
    }
    float3 _144 = (_142 * 0.5f) + 0.5f.xxx;
    float3 _166 = sqrt(clamp(_136, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_144.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_144.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_144.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_166.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_166.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_166.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
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
// lifted from _lilypad_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint _145;

static const float _147[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _148[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _149[4] = { 17u, 18u, 20u, 21u };

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
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_buffer : register(t18, space0);
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
static float2 out_var_C3;
static float3 out_var_C4;

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
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _169;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _169.data[_1ident] = instance_buffer.Load<float>(_1ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _188 = transpose(float4x3(float3(_169.data[0], _169.data[1], _169.data[2]), float3(_169.data[3], _169.data[4], _169.data[5]), float3(_169.data[6], _169.data[7], _169.data[8]), float3(_169.data[9], _169.data[10], _169.data[11])));
    float3 _193 = mul(_188, float4(in_var_Vertex_Position, 1.0f));
    float3 _194 = mul(_188, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _203 = uint(((((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _205 = _203 + (_203 << 10u);
    uint _207 = _205 ^ (_205 >> 6u);
    uint _209 = _207 + (_207 << 3u);
    uint _211 = _209 ^ (_209 >> 11u);
    _193.y = _193.y + lerp(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, asfloat(((_211 + (_211 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float2 _222 = _194.xz;
    float4 _228 = grass_object_push.SampleLevel(sampler_default, (_222 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _424;
    do
    {
        if (frame_constants_glade_theme_id == 4u)
        {
            _424 = 0.0f.xxx;
            break;
        }
        float _250 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float2 _163[4];
        uint _252 = 0u;
        uint _255 = 0u;
        uint _257 = 0u;
        bool _259 = false;
        bool _261 = false;
        for (; _252 < 4u; )
        {
            float _267 = float(_149[_252]);
            int _269 = int(floor(_267));
            float _270 = frac(_267);
            uint _271 = uint(_269);
            uint _273 = _271 + (_271 << 10u);
            uint _275 = _273 ^ (_273 >> 6u);
            uint _277 = _275 + (_275 << 3u);
            uint _279 = _277 ^ (_277 >> 11u);
            bool _282 = _261 ? _261 : true;
            uint _283 = _261 ? _257 : 1664525u;
            bool _284 = _259 ? _259 : true;
            uint _285 = _259 ? _255 : 1013904223u;
            uint _289 = (((_271 * _283) + (_279 + (_279 << 15u))) + _285) * _283;
            uint _291 = _289 ^ (_289 >> 11u);
            uint _294 = _291 ^ ((_291 << 7u) & 2636928640u);
            uint _297 = _294 ^ ((_294 << 15u) & 4022730752u);
            uint _299 = _297 ^ (_297 >> 18u);
            uint _301 = _299 + (_299 << 10u);
            uint _303 = _301 ^ (_301 >> 6u);
            uint _305 = _303 + (_303 << 3u);
            uint _307 = _305 ^ (_305 >> 11u);
            uint _320 = uint(_269 + 1);
            uint _322 = _320 + (_320 << 10u);
            uint _324 = _322 ^ (_322 >> 6u);
            uint _326 = _324 + (_324 << 3u);
            uint _328 = _326 ^ (_326 >> 11u);
            uint _258 = _282 ? _283 : 1664525u;
            uint _256 = _284 ? _285 : 1013904223u;
            uint _334 = (((_320 * _258) + (_328 + (_328 << 15u))) + _256) * _258;
            uint _336 = _334 ^ (_334 >> 11u);
            uint _339 = _336 ^ ((_336 << 7u) & 2636928640u);
            uint _342 = _339 ^ ((_339 << 15u) & 4022730752u);
            uint _344 = _342 ^ (_342 >> 18u);
            uint _346 = _344 + (_344 << 10u);
            uint _348 = _346 ^ (_346 >> 6u);
            uint _350 = _348 + (_348 << 3u);
            uint _352 = _350 ^ (_350 >> 11u);
            float _367 = (_270 * _270) * (3.0f - (2.0f * _270));
            _163[_252] = (normalize(((float2(asfloat((_299 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_307 + (_307 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - _367)) + (float2(asfloat((_344 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_352 + (_352 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _367)) - 0.5f.xx) * _250) * 3.0f;
            _252++;
            _255 = _256;
            _257 = _258;
            _259 = _284 ? _284 : true;
            _261 = _282 ? _282 : true;
            continue;
        }
        float3 _378;
        _378 = 0.0f.xxx;
        for (uint _381 = 0u; _381 < 1u; )
        {
            uint _382 = _381 + 1u;
            float _386 = float(_382) * 10.16640758514404296875f;
            float _387 = cos(_386);
            float _388 = sin(_386);
            float2x2 _392 = float2x2(float2(_387, _388), float2(-_388, _387));
            float3 _415 = (((water_noise_texture.SampleLevel(sampler_llr, mul(_392, (_222 * _147[_381]) + _163[_381]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _148[_381]) * float2(1.0f, _147[_381] * 0.01200000010430812835693359375f).xyy;
            float3 _419 = _378;
            _419.x = _378.x + _415.x;
            float2 _423 = _419.yz + mul(_415.yz, _392);
            _378 = float3(_419.x, _423.x, _423.y);
            _381 = _382;
            continue;
        }
        _424 = _378;
        break;
    } while(false);
    float3 _428 = _193 + (((float3(_228.x, 0.0f, _228.y) * ((clamp(_228.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_424.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_428, 1.0f));
    out_var_C1 = normalize(mul(_188, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _428;
    out_var_C3 = _428.xz * 0.100000001490116119384765625f;
    out_var_C4 = in_var_Vertex_Color;
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
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
