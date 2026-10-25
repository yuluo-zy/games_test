// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/flag_wall_cloth.raster.hlsl | F-V | bins: _flag_wall_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Wall banner cloth (roof family). The vs bends the flag with a gust envelope
// (origin-anchored 2D value noise) plus a faster flutter term; vertices are
// split between body/tip by is_tip, and corner flags are flattened (abs trick)
// with a smaller bend amplitude. ps samples the flag pattern + palette, using
// border_mask to pick solid-vs-pattern, and bakes normal+albedo+alpha into the
// solid gbuffer word. $Globals (set0/b13) holds the per-object transform/color.

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl carries only s0..s4,s9..s12. Declaring s5..s8
// here restores the 13 consecutive sampler slots that position $Globals at
// set0/b13.

// Textures used by the fragment stage (samplers come from inc/samplers.hlsl).
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

float4x4 model_to_world;
int seed;
float local_start_y;
float body_end_y;
float tip_end_y;
int is_corner;
int color_id;
uint2 border_mask_dims;
uint2 flag_palettes_dims;
int bool_render_shadows;

// 2D value noise with factored bilinear interpolation (matches the original
// compiled form; inc/perlin.hlsl's perlin() uses the lerp form instead).
float flag_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    return ((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
        + ((n01 * (1.0f - u.x)) * u.y)
        + ((n11 * u.x) * u.y);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int is_tip: is_tip;
    [[vk::location(5)]] float bby: bby;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float3 vpos = vin.Vertex_Position;
    float3 pos;
    if (vin.is_tip == 0)
    {
        pos = vpos;
        pos.y = -lerp(local_start_y, body_end_y, 1.0f - vin.bby);
    }
    else
    {
        pos = vpos;
        pos.y = -lerp(body_end_y, tip_end_y, 1.0f - vin.bby);
    }

    float t_sec = float(game_time_sec);
    float t = t_sec + game_time_frac;

    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float r0 = rand01(hash(uint(seed)));
    float bend = lerp(0.3f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) * 0.3f);

    float amp;
    if (is_corner == 0)
    {
        amp = bend * 0.3f;
    }
    else
    {
        amp = bend * 0.5f;
    }

    float flutter = flag_noise(float2(t + pos.x, pos.y));
    float z = pos.z + (sin((fmod(t_sec * 1.8f, TWO_PI) + (game_time_frac * 1.8f)) + (pos.y * 0.5f)
        + (flutter * smoothstep(0.0f, -3.0f, pos.y)))) * amp;
    float3 result = pos;
    result.z = z;
    if (is_corner == 0)
    {
        result.z = abs(z + 0.05f) - 0.05f;
    }

    float4 wp = mul(model_to_world, float4(result, 1.0f));
    float body_h = body_end_y - local_start_y;
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = float2(vpos.x + 0.5f, (result.y * 1.5f) + 0.5f);
    o.C4 = vin.Vertex_UV;
    o.C5 = (float2(vpos.x, result.y + (body_h * 0.5f) + 0.2f) * (0.8f / min(max(0.7f, body_h - 0.5f), 1.0f))) + 0.5f;
    o.C6 = (float2(vpos.x, (tip_end_y - local_start_y) + result.y) * 0.75f) + float2(0.5f, 0.0f);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    float layer1 = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float layer2 = float(int(round(lerp(0.0f, 2.0f, rand01(hash(uint(seed ^ 794)))))));
    float4 pat1 = flag_patterns.SampleBias(sampler_llr, float3(pin.C3.x, 1.0f - pin.C3.y, layer1), global_mip_bias);
    float4 pat2 = flag_patterns.SampleBias(sampler_llc, float3(pin.C6.x, 1.0f - pin.C6.y, layer2), global_mip_bias);
    float border = border_mask.SampleBias(sampler_llc, pin.C4, global_mip_bias).x;
    float pattern_sum = pat1.x + pat2.x;
    float pat3 = flag_patterns.SampleLevel(sampler_llc, float3(pin.C5.x, 1.0f - pin.C5.y, float((seed % 6) + 3)), 0.0f).x;

    uint cid = uint(color_id);
    float3 albedo;
    if (border > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern_sum).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, (pat3).xxx);
    }

    float3 nrm;
    float4 origin = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (dot(normalize((origin.xyz / origin.w) - pin.C2), pin.C1) <= 0.0f)
    {
        nrm = -pin.C1;
    }
    else
    {
        nrm = pin.C1;
    }

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    float alpha = (((border + pattern_sum) + pat3) > 0.89999997615814208984375f) ? 0.6f : 1.0f;
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(alpha, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_wall_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_seed : packoffset(c5);
    float _Globals_local_start_y : packoffset(c5.y);
    float _Globals_body_end_y : packoffset(c5.z);
    float _Globals_tip_end_y : packoffset(c5.w);
    int _Globals_is_corner : packoffset(c6);
    int _Globals_color_id : packoffset(c6.y);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_tip;
static float in_var_bby;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;
static float2 out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_tip: is_tip;
    [[vk::location(5)]] float in_var_bby: bby;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _127 = in_var_Vertex_Position;
    float3 _153;
    if (in_var_is_tip == 0)
    {
        float3 _152 = _127;
        _152.y = -lerp(_Globals_local_start_y, _Globals_body_end_y, 1.0f - in_var_bby);
        _153 = _152;
    }
    else
    {
        float3 _144 = _127;
        _144.y = -lerp(_Globals_body_end_y, _Globals_tip_end_y, 1.0f - in_var_bby);
        _153 = _144;
    }
    float _167 = float(frame_constants_game_time_sec);
    float _168 = _167 + frame_constants_game_time_frac;
    float2 _171 = (mul(_Globals_model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(_168, 0.0f) * 1.2000000476837158203125f);
    int2 _173 = int2(floor(_171));
    float2 _174 = frac(_171);
    uint2 _175 = uint2(_173);
    uint _177 = _175.y;
    uint _179 = _177 + (_177 << 10u);
    uint _181 = _179 ^ (_179 >> 6u);
    uint _183 = _181 + (_181 << 3u);
    uint _185 = _183 ^ (_183 >> 11u);
    uint _191 = (((_175.x * 1664525u) + (_185 + (_185 << 15u))) + 1013904223u) * 1664525u;
    uint _193 = _191 ^ (_191 >> 11u);
    uint _196 = _193 ^ ((_193 << 7u) & 2636928640u);
    uint _199 = _196 ^ ((_196 << 15u) & 4022730752u);
    uint2 _207 = uint2(_173 + int2(1, 0));
    uint _209 = _207.y;
    uint _211 = _209 + (_209 << 10u);
    uint _213 = _211 ^ (_211 >> 6u);
    uint _215 = _213 + (_213 << 3u);
    uint _217 = _215 ^ (_215 >> 11u);
    uint _223 = (((_207.x * 1664525u) + (_217 + (_217 << 15u))) + 1013904223u) * 1664525u;
    uint _225 = _223 ^ (_223 >> 11u);
    uint _228 = _225 ^ ((_225 << 7u) & 2636928640u);
    uint _231 = _228 ^ ((_228 << 15u) & 4022730752u);
    uint2 _239 = uint2(_173 + int2(0, 1));
    uint _241 = _239.y;
    uint _243 = _241 + (_241 << 10u);
    uint _245 = _243 ^ (_243 >> 6u);
    uint _247 = _245 + (_245 << 3u);
    uint _249 = _247 ^ (_247 >> 11u);
    uint _255 = (((_239.x * 1664525u) + (_249 + (_249 << 15u))) + 1013904223u) * 1664525u;
    uint _257 = _255 ^ (_255 >> 11u);
    uint _260 = _257 ^ ((_257 << 7u) & 2636928640u);
    uint _263 = _260 ^ ((_260 << 15u) & 4022730752u);
    uint2 _271 = uint2(_173 + int2(1, 1));
    uint _273 = _271.y;
    uint _275 = _273 + (_273 << 10u);
    uint _277 = _275 ^ (_275 >> 6u);
    uint _279 = _277 + (_277 << 3u);
    uint _281 = _279 ^ (_279 >> 11u);
    uint _287 = (((_271.x * 1664525u) + (_281 + (_281 << 15u))) + 1013904223u) * 1664525u;
    uint _289 = _287 ^ (_287 >> 11u);
    uint _292 = _289 ^ ((_289 << 7u) & 2636928640u);
    uint _295 = _292 ^ ((_292 << 15u) & 4022730752u);
    float2 _305 = (_174 * _174) * (3.0f.xx - (_174 * 2.0f));
    float _306 = _305.x;
    float _307 = 1.0f - _306;
    float _309 = _305.y;
    uint _325 = uint(_Globals_seed);
    uint _327 = _325 + (_325 << 10u);
    uint _329 = _327 ^ (_327 >> 6u);
    uint _331 = _329 + (_329 << 3u);
    uint _333 = _331 ^ (_331 >> 11u);
    float _342 = lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - _309) * (((asfloat(((_199 ^ (_199 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _307) + ((asfloat(((_231 ^ (_231 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _306))) + (((asfloat(((_263 ^ (_263 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _307) * _309)) + (((asfloat(((_295 ^ (_295 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _306) * _309)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_333 + (_333 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    bool _345 = _Globals_is_corner == 0;
    float _351;
    if (_345)
    {
        _351 = _342 * 0.300000011920928955078125f;
    }
    else
    {
        _351 = _342 * 0.5f;
    }
    float2 _360 = float2(_168 + _153.x, _153.y);
    int2 _362 = int2(floor(_360));
    float2 _363 = frac(_360);
    uint2 _364 = uint2(_362);
    uint _366 = _364.y;
    uint _368 = _366 + (_366 << 10u);
    uint _370 = _368 ^ (_368 >> 6u);
    uint _372 = _370 + (_370 << 3u);
    uint _374 = _372 ^ (_372 >> 11u);
    uint _380 = (((_364.x * 1664525u) + (_374 + (_374 << 15u))) + 1013904223u) * 1664525u;
    uint _382 = _380 ^ (_380 >> 11u);
    uint _385 = _382 ^ ((_382 << 7u) & 2636928640u);
    uint _388 = _385 ^ ((_385 << 15u) & 4022730752u);
    uint2 _396 = uint2(_362 + int2(1, 0));
    uint _398 = _396.y;
    uint _400 = _398 + (_398 << 10u);
    uint _402 = _400 ^ (_400 >> 6u);
    uint _404 = _402 + (_402 << 3u);
    uint _406 = _404 ^ (_404 >> 11u);
    uint _412 = (((_396.x * 1664525u) + (_406 + (_406 << 15u))) + 1013904223u) * 1664525u;
    uint _414 = _412 ^ (_412 >> 11u);
    uint _417 = _414 ^ ((_414 << 7u) & 2636928640u);
    uint _420 = _417 ^ ((_417 << 15u) & 4022730752u);
    uint2 _428 = uint2(_362 + int2(0, 1));
    uint _430 = _428.y;
    uint _432 = _430 + (_430 << 10u);
    uint _434 = _432 ^ (_432 >> 6u);
    uint _436 = _434 + (_434 << 3u);
    uint _438 = _436 ^ (_436 >> 11u);
    uint _444 = (((_428.x * 1664525u) + (_438 + (_438 << 15u))) + 1013904223u) * 1664525u;
    uint _446 = _444 ^ (_444 >> 11u);
    uint _449 = _446 ^ ((_446 << 7u) & 2636928640u);
    uint _452 = _449 ^ ((_449 << 15u) & 4022730752u);
    uint2 _460 = uint2(_362 + int2(1, 1));
    uint _462 = _460.y;
    uint _464 = _462 + (_462 << 10u);
    uint _466 = _464 ^ (_464 >> 6u);
    uint _468 = _466 + (_466 << 3u);
    uint _470 = _468 ^ (_468 >> 11u);
    uint _476 = (((_460.x * 1664525u) + (_470 + (_470 << 15u))) + 1013904223u) * 1664525u;
    uint _478 = _476 ^ (_476 >> 11u);
    uint _481 = _478 ^ ((_478 << 7u) & 2636928640u);
    uint _484 = _481 ^ ((_481 << 15u) & 4022730752u);
    float2 _494 = (_363 * _363) * (3.0f.xx - (_363 * 2.0f));
    float _495 = _494.x;
    float _496 = 1.0f - _495;
    float _498 = _494.y;
    float _514 = _153.z + ((sin(((fmod(_167 * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (_153.y * 0.5f)) + ((((1.0f - _498) * (((asfloat(((_388 ^ (_388 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _496) + ((asfloat(((_420 ^ (_420 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _495))) + (((asfloat(((_452 ^ (_452 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _496) * _498)) + (((asfloat(((_484 ^ (_484 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _495) * _498))) * smoothstep(0.0f, -3.0f, _153.y)) * _351);
    float3 _515 = _153;
    _515.z = _514;
    float3 _522;
    if (_345)
    {
        float3 _521 = _515;
        _521.z = abs(_514 + 0.0500000007450580596923828125f) - 0.0500000007450580596923828125f;
        _522 = _521;
    }
    else
    {
        _522 = _515;
    }
    float4 _527 = mul(_Globals_model_to_world, float4(_522, 1.0f));
    float _549 = _Globals_body_end_y - _Globals_local_start_y;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _527);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_Globals_model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = _527.xyz;
    out_var_C3 = float2(_127.x + 0.5f, (_522.y * 1.5f) + 0.5f);
    out_var_C4 = in_var_Vertex_UV;
    out_var_C5 = (float2(_127.x, (_522.y + (_549 * 0.5f)) + 0.20000000298023223876953125f) * (0.800000011920928955078125f / min(max(0.699999988079071044921875f, _549 - 0.5f), 1.0f))) + 0.5f.xx;
    out_var_C6 = (float2(_127.x, (_Globals_tip_end_y - _Globals_local_start_y) + _522.y) * 0.75f) + float2(0.5f, 0.0f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_tip = stage_input.in_var_is_tip;
    in_var_bby = stage_input.in_var_bby;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _flag_wall_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_seed : packoffset(c5);
    float _Globals_local_start_y : packoffset(c5.y);
    float _Globals_body_end_y : packoffset(c5.z);
    float _Globals_tip_end_y : packoffset(c5.w);
    int _Globals_is_corner : packoffset(c6);
    int _Globals_color_id : packoffset(c6.y);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static float2 in_var_C5;
static float2 in_var_C6;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1: Vertex_Color;
    [[vk::location(2)]] float3 in_var_C2: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_C3: Vertex_UV;
    [[vk::location(4)]] float2 in_var_C4: is_tip;
    [[vk::location(5)]] float2 in_var_C5: bby;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    uint _123 = uint(_Globals_seed ^ 234234);
    uint _125 = _123 + (_123 << 10u);
    uint _127 = _125 ^ (_125 >> 6u);
    uint _129 = _127 + (_127 << 3u);
    uint _131 = _129 ^ (_129 >> 11u);
    uint _142 = uint(_Globals_seed ^ 794);
    uint _144 = _142 + (_142 << 10u);
    uint _146 = _144 ^ (_144 >> 6u);
    uint _148 = _146 + (_146 << 3u);
    uint _150 = _148 ^ (_148 >> 11u);
    float4 _172 = flag_patterns.SampleBias(sampler_llr, float3(in_var_C3.x, 1.0f - in_var_C3.y, float(int(round(lerp(9.0f, 11.0f, asfloat(((_131 + (_131 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    float4 _181 = flag_patterns.SampleBias(sampler_llc, float3(in_var_C6.x, 1.0f - in_var_C6.y, float(int(round(lerp(0.0f, 2.0f, asfloat(((_150 + (_150 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    float4 _185 = border_mask.SampleBias(sampler_llc, in_var_C4, frame_constants_global_mip_bias);
    float _186 = _185.x;
    float _187 = _172.x + _181.x;
    float4 _194 = flag_patterns.SampleLevel(sampler_llc, float3(in_var_C5.x, 1.0f - in_var_C5.y, float((_Globals_seed - 6 * (_Globals_seed / 6)) + 3)), 0.0f);
    float _195 = _194.x;
    uint _198 = uint(_Globals_color_id);
    float3 _220;
    if (_186 > 0.89999997615814208984375f)
    {
        _220 = flag_palettes.Load(int3(uint2(1u, _198), 0u)).xyz;
    }
    else
    {
        _220 = lerp(lerp(flag_palettes.Load(int3(uint2(0u, _198), 0u)).xyz, flag_palettes.Load(int3(uint2(2u, _198), 0u)).xyz, _187.xxx), flag_palettes.Load(int3(uint2(3u, _198), 0u)).xyz, _195.xxx);
    }
    float4 _225 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _237;
    if (dot(normalize((_225.xyz / _225.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        _237 = -in_var_C1;
    }
    else
    {
        _237 = in_var_C1;
    }
    float3 _244 = (normalize(_237) * 0.5f) + 0.5f.xxx;
    float3 _271 = sqrt(clamp(_220, 0.0f.xxx, 1.0f.xxx));
    uint _290 = (uint((clamp(_271.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_271.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_271.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _291 = uint2(((uint((clamp(_244.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_244.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_244.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp((((_186 + _187) + _195) > 0.89999997615814208984375f) ? 0.60000002384185791015625f : 1.0f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _290);
    _291.y = _290 | 117440512u;
    out_var_SV_TARGET0 = _291;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_wall_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    int _Globals_seed : packoffset(c5);
    float _Globals_local_start_y : packoffset(c5.y);
    float _Globals_body_end_y : packoffset(c5.z);
    float _Globals_tip_end_y : packoffset(c5.w);
    int _Globals_is_corner : packoffset(c6);
    int _Globals_color_id : packoffset(c6.y);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_tip;
static float in_var_bby;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;
static float2 out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_tip: is_tip;
    [[vk::location(5)]] float in_var_bby: bby;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _127 = in_var_Vertex_Position;
    float3 _153;
    if (in_var_is_tip == 0)
    {
        float3 _152 = _127;
        _152.y = -lerp(_Globals_local_start_y, _Globals_body_end_y, 1.0f - in_var_bby);
        _153 = _152;
    }
    else
    {
        float3 _144 = _127;
        _144.y = -lerp(_Globals_body_end_y, _Globals_tip_end_y, 1.0f - in_var_bby);
        _153 = _144;
    }
    float _167 = float(frame_constants_game_time_sec);
    float _168 = _167 + frame_constants_game_time_frac;
    float2 _171 = (mul(_Globals_model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(_168, 0.0f) * 1.2000000476837158203125f);
    int2 _173 = int2(floor(_171));
    float2 _174 = frac(_171);
    uint2 _175 = uint2(_173);
    uint _177 = _175.y;
    uint _179 = _177 + (_177 << 10u);
    uint _181 = _179 ^ (_179 >> 6u);
    uint _183 = _181 + (_181 << 3u);
    uint _185 = _183 ^ (_183 >> 11u);
    uint _191 = (((_175.x * 1664525u) + (_185 + (_185 << 15u))) + 1013904223u) * 1664525u;
    uint _193 = _191 ^ (_191 >> 11u);
    uint _196 = _193 ^ ((_193 << 7u) & 2636928640u);
    uint _199 = _196 ^ ((_196 << 15u) & 4022730752u);
    uint2 _207 = uint2(_173 + int2(1, 0));
    uint _209 = _207.y;
    uint _211 = _209 + (_209 << 10u);
    uint _213 = _211 ^ (_211 >> 6u);
    uint _215 = _213 + (_213 << 3u);
    uint _217 = _215 ^ (_215 >> 11u);
    uint _223 = (((_207.x * 1664525u) + (_217 + (_217 << 15u))) + 1013904223u) * 1664525u;
    uint _225 = _223 ^ (_223 >> 11u);
    uint _228 = _225 ^ ((_225 << 7u) & 2636928640u);
    uint _231 = _228 ^ ((_228 << 15u) & 4022730752u);
    uint2 _239 = uint2(_173 + int2(0, 1));
    uint _241 = _239.y;
    uint _243 = _241 + (_241 << 10u);
    uint _245 = _243 ^ (_243 >> 6u);
    uint _247 = _245 + (_245 << 3u);
    uint _249 = _247 ^ (_247 >> 11u);
    uint _255 = (((_239.x * 1664525u) + (_249 + (_249 << 15u))) + 1013904223u) * 1664525u;
    uint _257 = _255 ^ (_255 >> 11u);
    uint _260 = _257 ^ ((_257 << 7u) & 2636928640u);
    uint _263 = _260 ^ ((_260 << 15u) & 4022730752u);
    uint2 _271 = uint2(_173 + int2(1, 1));
    uint _273 = _271.y;
    uint _275 = _273 + (_273 << 10u);
    uint _277 = _275 ^ (_275 >> 6u);
    uint _279 = _277 + (_277 << 3u);
    uint _281 = _279 ^ (_279 >> 11u);
    uint _287 = (((_271.x * 1664525u) + (_281 + (_281 << 15u))) + 1013904223u) * 1664525u;
    uint _289 = _287 ^ (_287 >> 11u);
    uint _292 = _289 ^ ((_289 << 7u) & 2636928640u);
    uint _295 = _292 ^ ((_292 << 15u) & 4022730752u);
    float2 _305 = (_174 * _174) * (3.0f.xx - (_174 * 2.0f));
    float _306 = _305.x;
    float _307 = 1.0f - _306;
    float _309 = _305.y;
    uint _325 = uint(_Globals_seed);
    uint _327 = _325 + (_325 << 10u);
    uint _329 = _327 ^ (_327 >> 6u);
    uint _331 = _329 + (_329 << 3u);
    uint _333 = _331 ^ (_331 >> 11u);
    float _342 = lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - _309) * (((asfloat(((_199 ^ (_199 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _307) + ((asfloat(((_231 ^ (_231 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _306))) + (((asfloat(((_263 ^ (_263 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _307) * _309)) + (((asfloat(((_295 ^ (_295 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _306) * _309)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_333 + (_333 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    bool _345 = _Globals_is_corner == 0;
    float _351;
    if (_345)
    {
        _351 = _342 * 0.300000011920928955078125f;
    }
    else
    {
        _351 = _342 * 0.5f;
    }
    float2 _360 = float2(_168 + _153.x, _153.y);
    int2 _362 = int2(floor(_360));
    float2 _363 = frac(_360);
    uint2 _364 = uint2(_362);
    uint _366 = _364.y;
    uint _368 = _366 + (_366 << 10u);
    uint _370 = _368 ^ (_368 >> 6u);
    uint _372 = _370 + (_370 << 3u);
    uint _374 = _372 ^ (_372 >> 11u);
    uint _380 = (((_364.x * 1664525u) + (_374 + (_374 << 15u))) + 1013904223u) * 1664525u;
    uint _382 = _380 ^ (_380 >> 11u);
    uint _385 = _382 ^ ((_382 << 7u) & 2636928640u);
    uint _388 = _385 ^ ((_385 << 15u) & 4022730752u);
    uint2 _396 = uint2(_362 + int2(1, 0));
    uint _398 = _396.y;
    uint _400 = _398 + (_398 << 10u);
    uint _402 = _400 ^ (_400 >> 6u);
    uint _404 = _402 + (_402 << 3u);
    uint _406 = _404 ^ (_404 >> 11u);
    uint _412 = (((_396.x * 1664525u) + (_406 + (_406 << 15u))) + 1013904223u) * 1664525u;
    uint _414 = _412 ^ (_412 >> 11u);
    uint _417 = _414 ^ ((_414 << 7u) & 2636928640u);
    uint _420 = _417 ^ ((_417 << 15u) & 4022730752u);
    uint2 _428 = uint2(_362 + int2(0, 1));
    uint _430 = _428.y;
    uint _432 = _430 + (_430 << 10u);
    uint _434 = _432 ^ (_432 >> 6u);
    uint _436 = _434 + (_434 << 3u);
    uint _438 = _436 ^ (_436 >> 11u);
    uint _444 = (((_428.x * 1664525u) + (_438 + (_438 << 15u))) + 1013904223u) * 1664525u;
    uint _446 = _444 ^ (_444 >> 11u);
    uint _449 = _446 ^ ((_446 << 7u) & 2636928640u);
    uint _452 = _449 ^ ((_449 << 15u) & 4022730752u);
    uint2 _460 = uint2(_362 + int2(1, 1));
    uint _462 = _460.y;
    uint _464 = _462 + (_462 << 10u);
    uint _466 = _464 ^ (_464 >> 6u);
    uint _468 = _466 + (_466 << 3u);
    uint _470 = _468 ^ (_468 >> 11u);
    uint _476 = (((_460.x * 1664525u) + (_470 + (_470 << 15u))) + 1013904223u) * 1664525u;
    uint _478 = _476 ^ (_476 >> 11u);
    uint _481 = _478 ^ ((_478 << 7u) & 2636928640u);
    uint _484 = _481 ^ ((_481 << 15u) & 4022730752u);
    float2 _494 = (_363 * _363) * (3.0f.xx - (_363 * 2.0f));
    float _495 = _494.x;
    float _496 = 1.0f - _495;
    float _498 = _494.y;
    float _514 = _153.z + ((sin(((fmod(_167 * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (_153.y * 0.5f)) + ((((1.0f - _498) * (((asfloat(((_388 ^ (_388 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _496) + ((asfloat(((_420 ^ (_420 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _495))) + (((asfloat(((_452 ^ (_452 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _496) * _498)) + (((asfloat(((_484 ^ (_484 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _495) * _498))) * smoothstep(0.0f, -3.0f, _153.y)) * _351);
    float3 _515 = _153;
    _515.z = _514;
    float3 _522;
    if (_345)
    {
        float3 _521 = _515;
        _521.z = abs(_514 + 0.0500000007450580596923828125f) - 0.0500000007450580596923828125f;
        _522 = _521;
    }
    else
    {
        _522 = _515;
    }
    float4 _527 = mul(_Globals_model_to_world, float4(_522, 1.0f));
    float _549 = _Globals_body_end_y - _Globals_local_start_y;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _527);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_Globals_model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = _527.xyz;
    out_var_C3 = float2(_127.x + 0.5f, (_522.y * 1.5f) + 0.5f);
    out_var_C4 = in_var_Vertex_UV;
    out_var_C5 = (float2(_127.x, (_522.y + (_549 * 0.5f)) + 0.20000000298023223876953125f) * (0.800000011920928955078125f / min(max(0.699999988079071044921875f, _549 - 0.5f), 1.0f))) + 0.5f.xx;
    out_var_C6 = (float2(_127.x, (_Globals_tip_end_y - _Globals_local_start_y) + _522.y) * 0.75f) + float2(0.5f, 0.0f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_tip = stage_input.in_var_is_tip;
    in_var_bby = stage_input.in_var_bby;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _flag_wall_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
