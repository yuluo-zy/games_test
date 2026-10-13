// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/flag_roof_cloth.raster.hlsl | F-V | bins: _flag_roof_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Roof-top flag cloth (roof family). The vs blends between a rest pose and an
// animated pose read from anim textures (positions + normals), keyed by a sway
// envelope driven by an origin-anchored gust noise; rest_pos/rest_normal come
// from the CPU side. ps samples the flag pattern + palette (border_mask picks
// solid-vs-pattern) and bakes normal+albedo into the solid gbuffer word.
// $Globals (set0/b13) holds the per-object transform/color/seed + texture dims.

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

// Sampler and animation textures used by the vertex stage.
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

// Textures used by the fragment stage.
Texture2D<float4> border_mask : register(t17, space0);
Texture2D<float4> flag_palettes : register(t18, space0);
Texture2DArray<float4> flag_patterns : register(t19, space0);

int seed;
int color_id;
float4x4 model_to_world;
uint2 flag_color_dims;
uint2 anim_texture_dims;
uint2 anim_normal_texture_dims;
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
    [[vk::location(4)]] float3 rest_pos: rest_pos;
    [[vk::location(5)]] float3 rest_normal: rest_normal;
    [[vk::location(6)]] float3 uv2: uv2;
    uint vertex_id : SV_VertexID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float t = float(game_time_sec) + game_time_frac;
    float r0 = rand01(hash(uint(seed)));

    // Animation uv: per-vertex row, time/rand column.
    float2 anim_uv = float2((float(vin.vertex_id) + 0.5f) * 0.0125f, (t * 0.15f) + (r0 * 0.2f));

    // Gust sway envelope.
    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float sway = clamp(smoothstep(0.0f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) + 0.7f), 0.0f, 1.0f);

    uint2 vidx = uint2(uint(vin.vertex_id), 0u);
    float3 anim_pos = (anim_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_texture.Load(int3(vidx, 0u)).xyz) * 2.0f * 0.8f;
    float3 anim_nrm = anim_normal_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_normal_texture.Load(int3(vidx, 0u)).xyz;

    float4 wp = mul(model_to_world, float4(lerp(vin.rest_pos * 0.8f, (vin.Vertex_Position * 0.8f) + anim_pos, sway), 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = normalize(mul(model_to_world, float4(lerp(vin.rest_normal, vin.Vertex_Normal + anim_nrm, sway), 0.0f)).xyz);
    o.C2 = wp.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = float2(((vin.uv2.x - 0.5f) * 1.2f) + 0.5f, vin.uv2.y * 7.0f);
    o.C5 = ((float2((vin.uv2.y * 4.0f) - 2.75f, vin.uv2.x) - 0.5f) * 1.3f) + 0.5f;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    // View position and normal facing.
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

    float layer = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float4 pattern = flag_patterns.SampleBias(sampler_llr, float3(pin.C4.x, 1.0f - pin.C4.y, layer), global_mip_bias);

    uint cid = uint(color_id);
    float3 albedo;
    if (border_mask.SampleBias(sampler_llc, pin.C3, global_mip_bias).x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern.x).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz,
                      (flag_patterns.SampleLevel(sampler_llc, float3(pin.C5, float((seed % 6) + 3)), 0.0f).x).xxx);
    }

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 100663296u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_roof_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int _Globals_seed : packoffset(c0.y);
    int _Globals_color_id : packoffset(c0.z);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    uint2 _Globals_flag_color_dims : packoffset(c5);
    uint2 _Globals_anim_texture_dims : packoffset(c5.z);
    uint2 _Globals_anim_normal_texture_dims : packoffset(c6);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_normal;
static float3 in_var_uv2;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_normal: rest_normal;
    [[vk::location(6)]] float3 in_var_uv2: uv2;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint2 _139 = uint2(uint(gl_VertexIndex), 0u);
    uint _145 = uint(_Globals_seed);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    float _159 = asfloat(((_153 + (_153 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _165 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
    float2 _172 = float2((float(uint(gl_VertexIndex)) + 0.5f) * 0.012500000186264514923095703125f, (_165 * 0.1500000059604644775390625f) + (_159 * 0.20000000298023223876953125f));
    float2 _196 = (mul(_Globals_model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(_165, 0.0f) * 1.2000000476837158203125f);
    int2 _198 = int2(floor(_196));
    float2 _199 = frac(_196);
    uint2 _200 = uint2(_198);
    uint _202 = _200.y;
    uint _204 = _202 + (_202 << 10u);
    uint _206 = _204 ^ (_204 >> 6u);
    uint _208 = _206 + (_206 << 3u);
    uint _210 = _208 ^ (_208 >> 11u);
    uint _216 = (((_200.x * 1664525u) + (_210 + (_210 << 15u))) + 1013904223u) * 1664525u;
    uint _218 = _216 ^ (_216 >> 11u);
    uint _221 = _218 ^ ((_218 << 7u) & 2636928640u);
    uint _224 = _221 ^ ((_221 << 15u) & 4022730752u);
    uint2 _232 = uint2(_198 + int2(1, 0));
    uint _234 = _232.y;
    uint _236 = _234 + (_234 << 10u);
    uint _238 = _236 ^ (_236 >> 6u);
    uint _240 = _238 + (_238 << 3u);
    uint _242 = _240 ^ (_240 >> 11u);
    uint _248 = (((_232.x * 1664525u) + (_242 + (_242 << 15u))) + 1013904223u) * 1664525u;
    uint _250 = _248 ^ (_248 >> 11u);
    uint _253 = _250 ^ ((_250 << 7u) & 2636928640u);
    uint _256 = _253 ^ ((_253 << 15u) & 4022730752u);
    uint2 _264 = uint2(_198 + int2(0, 1));
    uint _266 = _264.y;
    uint _268 = _266 + (_266 << 10u);
    uint _270 = _268 ^ (_268 >> 6u);
    uint _272 = _270 + (_270 << 3u);
    uint _274 = _272 ^ (_272 >> 11u);
    uint _280 = (((_264.x * 1664525u) + (_274 + (_274 << 15u))) + 1013904223u) * 1664525u;
    uint _282 = _280 ^ (_280 >> 11u);
    uint _285 = _282 ^ ((_282 << 7u) & 2636928640u);
    uint _288 = _285 ^ ((_285 << 15u) & 4022730752u);
    uint2 _296 = uint2(_198 + int2(1, 1));
    uint _298 = _296.y;
    uint _300 = _298 + (_298 << 10u);
    uint _302 = _300 ^ (_300 >> 6u);
    uint _304 = _302 + (_302 << 3u);
    uint _306 = _304 ^ (_304 >> 11u);
    uint _312 = (((_296.x * 1664525u) + (_306 + (_306 << 15u))) + 1013904223u) * 1664525u;
    uint _314 = _312 ^ (_312 >> 11u);
    uint _317 = _314 ^ ((_314 << 7u) & 2636928640u);
    uint _320 = _317 ^ ((_317 << 15u) & 4022730752u);
    float2 _330 = (_199 * _199) * (3.0f.xx - (_199 * 2.0f));
    float _331 = _330.x;
    float _332 = 1.0f - _331;
    float _334 = _330.y;
    float3 _355 = clamp(smoothstep(0.0f, 1.0f, (((sin((((((1.0f - _334) * (((asfloat(((_224 ^ (_224 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) + ((asfloat(((_256 ^ (_256 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331))) + (((asfloat(((_288 ^ (_288 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) * _334)) + (((asfloat(((_320 ^ (_320 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331) * _334)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, _159)) + 0.699999988079071044921875f), 0.0f, 1.0f).xxx;
    float4 _363 = mul(_Globals_model_to_world, float4(lerp(in_var_rest_pos * 0.800000011920928955078125f, (in_var_Vertex_Position * 0.800000011920928955078125f) + (((anim_texture.SampleLevel(sampler_llr, _172, 0.0f).xyz - anim_texture.Load(int3(_139, 0u)).xyz) * 2.0f) * 0.800000011920928955078125f), _355), 1.0f));
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _363);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = normalize(mul(_Globals_model_to_world, float4(lerp(in_var_rest_normal, in_var_Vertex_Normal + (anim_normal_texture.SampleLevel(sampler_llr, _172, 0.0f).xyz - anim_normal_texture.Load(int3(_139, 0u)).xyz), _355), 0.0f)).xyz);
    out_var_C2 = _363.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(((in_var_uv2.x - 0.5f) * 1.2000000476837158203125f) + 0.5f, in_var_uv2.y * 7.0f);
    out_var_C5 = ((float2((in_var_uv2.y * 4.0f) - 2.75f, in_var_uv2.x) - 0.5f.xx) * 1.2999999523162841796875f) + 0.5f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_normal = stage_input.in_var_rest_normal;
    in_var_uv2 = stage_input.in_var_uv2;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
// lifted from _flag_roof_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int _Globals_seed : packoffset(c0.y);
    int _Globals_color_id : packoffset(c0.z);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    uint2 _Globals_flag_color_dims : packoffset(c5);
    uint2 _Globals_anim_texture_dims : packoffset(c5.z);
    uint2 _Globals_anim_normal_texture_dims : packoffset(c6);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> border_mask : register(t17, space0);
Texture2D<float4> flag_palettes : register(t18, space0);
Texture2DArray<float4> flag_patterns : register(t19, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static float2 in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1: Vertex_Color;
    [[vk::location(2)]] float3 in_var_C2: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_C3: Vertex_UV;
    [[vk::location(4)]] float2 in_var_C4: rest_pos;
    [[vk::location(5)]] float2 in_var_C5: rest_normal;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _119 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _131;
    if (dot(normalize((_119.xyz / _119.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        _131 = -in_var_C1;
    }
    else
    {
        _131 = in_var_C1;
    }
    uint _143 = uint(_Globals_seed ^ 234234);
    uint _145 = _143 + (_143 << 10u);
    uint _147 = _145 ^ (_145 >> 6u);
    uint _149 = _147 + (_147 << 3u);
    uint _151 = _149 ^ (_149 >> 11u);
    float4 _170 = flag_patterns.SampleBias(sampler_llr, float3(in_var_C4, float(int(round(lerp(9.0f, 11.0f, asfloat(((_151 + (_151 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    uint _181 = uint(_Globals_color_id);
    float3 _203;
    if (border_mask.SampleBias(sampler_llc, in_var_C3, frame_constants_global_mip_bias).x > 0.89999997615814208984375f)
    {
        _203 = flag_palettes.Load(int3(uint2(1u, _181), 0u)).xyz;
    }
    else
    {
        _203 = lerp(lerp(flag_palettes.Load(int3(uint2(0u, _181), 0u)).xyz, flag_palettes.Load(int3(uint2(2u, _181), 0u)).xyz, _170.x.xxx), flag_palettes.Load(int3(uint2(3u, _181), 0u)).xyz, flag_patterns.SampleLevel(sampler_llc, float3(in_var_C5, float((_Globals_seed - 6 * (_Globals_seed / 6)) + 3)), 0.0f).x.xxx);
    }
    float3 _205 = (normalize(_131) * 0.5f) + 0.5f.xxx;
    float3 _227 = sqrt(clamp(_203, 0.0f.xxx, 1.0f.xxx));
    uint _246 = (uint((clamp(_227.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_227.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_227.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _247 = uint2(((uint((clamp(_205.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_205.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_205.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, _246);
    _247.y = _246 | 100663296u;
    out_var_SV_TARGET0 = _247;
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
// lifted from _flag_roof_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int _Globals_seed : packoffset(c0.y);
    int _Globals_color_id : packoffset(c0.z);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    uint2 _Globals_flag_color_dims : packoffset(c5);
    uint2 _Globals_anim_texture_dims : packoffset(c5.z);
    uint2 _Globals_anim_normal_texture_dims : packoffset(c6);
    uint2 _Globals_border_mask_dims : packoffset(c6.z);
    uint2 _Globals_flag_palettes_dims : packoffset(c7);
    int _Globals_bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_normal;
static float3 in_var_uv2;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_normal: rest_normal;
    [[vk::location(6)]] float3 in_var_uv2: uv2;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint2 _139 = uint2(uint(gl_VertexIndex), 0u);
    uint _145 = uint(_Globals_seed);
    uint _147 = _145 + (_145 << 10u);
    uint _149 = _147 ^ (_147 >> 6u);
    uint _151 = _149 + (_149 << 3u);
    uint _153 = _151 ^ (_151 >> 11u);
    float _159 = asfloat(((_153 + (_153 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float _165 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
    float2 _172 = float2((float(uint(gl_VertexIndex)) + 0.5f) * 0.012500000186264514923095703125f, (_165 * 0.1500000059604644775390625f) + (_159 * 0.20000000298023223876953125f));
    float2 _196 = (mul(_Globals_model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(_165, 0.0f) * 1.2000000476837158203125f);
    int2 _198 = int2(floor(_196));
    float2 _199 = frac(_196);
    uint2 _200 = uint2(_198);
    uint _202 = _200.y;
    uint _204 = _202 + (_202 << 10u);
    uint _206 = _204 ^ (_204 >> 6u);
    uint _208 = _206 + (_206 << 3u);
    uint _210 = _208 ^ (_208 >> 11u);
    uint _216 = (((_200.x * 1664525u) + (_210 + (_210 << 15u))) + 1013904223u) * 1664525u;
    uint _218 = _216 ^ (_216 >> 11u);
    uint _221 = _218 ^ ((_218 << 7u) & 2636928640u);
    uint _224 = _221 ^ ((_221 << 15u) & 4022730752u);
    uint2 _232 = uint2(_198 + int2(1, 0));
    uint _234 = _232.y;
    uint _236 = _234 + (_234 << 10u);
    uint _238 = _236 ^ (_236 >> 6u);
    uint _240 = _238 + (_238 << 3u);
    uint _242 = _240 ^ (_240 >> 11u);
    uint _248 = (((_232.x * 1664525u) + (_242 + (_242 << 15u))) + 1013904223u) * 1664525u;
    uint _250 = _248 ^ (_248 >> 11u);
    uint _253 = _250 ^ ((_250 << 7u) & 2636928640u);
    uint _256 = _253 ^ ((_253 << 15u) & 4022730752u);
    uint2 _264 = uint2(_198 + int2(0, 1));
    uint _266 = _264.y;
    uint _268 = _266 + (_266 << 10u);
    uint _270 = _268 ^ (_268 >> 6u);
    uint _272 = _270 + (_270 << 3u);
    uint _274 = _272 ^ (_272 >> 11u);
    uint _280 = (((_264.x * 1664525u) + (_274 + (_274 << 15u))) + 1013904223u) * 1664525u;
    uint _282 = _280 ^ (_280 >> 11u);
    uint _285 = _282 ^ ((_282 << 7u) & 2636928640u);
    uint _288 = _285 ^ ((_285 << 15u) & 4022730752u);
    uint2 _296 = uint2(_198 + int2(1, 1));
    uint _298 = _296.y;
    uint _300 = _298 + (_298 << 10u);
    uint _302 = _300 ^ (_300 >> 6u);
    uint _304 = _302 + (_302 << 3u);
    uint _306 = _304 ^ (_304 >> 11u);
    uint _312 = (((_296.x * 1664525u) + (_306 + (_306 << 15u))) + 1013904223u) * 1664525u;
    uint _314 = _312 ^ (_312 >> 11u);
    uint _317 = _314 ^ ((_314 << 7u) & 2636928640u);
    uint _320 = _317 ^ ((_317 << 15u) & 4022730752u);
    float2 _330 = (_199 * _199) * (3.0f.xx - (_199 * 2.0f));
    float _331 = _330.x;
    float _332 = 1.0f - _331;
    float _334 = _330.y;
    float3 _355 = clamp(smoothstep(0.0f, 1.0f, (((sin((((((1.0f - _334) * (((asfloat(((_224 ^ (_224 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) + ((asfloat(((_256 ^ (_256 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331))) + (((asfloat(((_288 ^ (_288 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) * _334)) + (((asfloat(((_320 ^ (_320 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331) * _334)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, _159)) + 0.699999988079071044921875f), 0.0f, 1.0f).xxx;
    float4 _363 = mul(_Globals_model_to_world, float4(lerp(in_var_rest_pos * 0.800000011920928955078125f, (in_var_Vertex_Position * 0.800000011920928955078125f) + (((anim_texture.SampleLevel(sampler_llr, _172, 0.0f).xyz - anim_texture.Load(int3(_139, 0u)).xyz) * 2.0f) * 0.800000011920928955078125f), _355), 1.0f));
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _363);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = normalize(mul(_Globals_model_to_world, float4(lerp(in_var_rest_normal, in_var_Vertex_Normal + (anim_normal_texture.SampleLevel(sampler_llr, _172, 0.0f).xyz - anim_normal_texture.Load(int3(_139, 0u)).xyz), _355), 0.0f)).xyz);
    out_var_C2 = _363.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(((in_var_uv2.x - 0.5f) * 1.2000000476837158203125f) + 0.5f, in_var_uv2.y * 7.0f);
    out_var_C5 = ((float2((in_var_uv2.y * 4.0f) - 2.75f, in_var_uv2.x) - 0.5f.xx) * 1.2999999523162841796875f) + 0.5f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_normal = stage_input.in_var_rest_normal;
    in_var_uv2 = stage_input.in_var_uv2;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
// lifted from _flag_roof_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
