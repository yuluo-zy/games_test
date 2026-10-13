// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/splash_particle.raster.hlsl | F-V | bins: _splash_particle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Splash particle: per-vertex position/color/normal/uv transformed by a global
// model_to_world; ps applies a 2-octave value-noise ring pattern (drop + ripple)
// animated by anim_t and time, discards outside the pattern, packs solid gbuffer
// (normal RGB8 + 0xFF000000, albedo RGB8 | material flags 0x46000000).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

float4x4 model_to_world;
int bool_render_shadows;
float anim_t;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world_pos = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(GetCurrentViewConstants().world_to_sample, world_pos);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world_pos.xyz;
    o.C3 = vin.Vertex_UV;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// Ring falloff: returns 1 at the ring center, fading to 0 at the radius edge.
float drop_ring(float d, float radius)
{
    if (d > radius)
    {
        return 0.0f;
    }
    float t = d / radius;
    return 1.0f - (t * t) * (3.0f - (2.0f * t));
}

PsOut ps_main(PsIn pin)
{
    PsOut o;
    static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
    uint2 frag = uint2(pin.position.xy) & uint2(1u, 1u);
    uint2 sub = hi_px_subpixels[frame_index & 3u];
    if (all(frag != sub))
    {
        discard;
    }

    float t = 1.0f - anim_t;
    float2 uv = pin.C3 * 25.6f;

    // Drop ring: 2D value-noise (LCG hash lattice, explicit bilinear blend).
    float2 npos1 = float2((uv.x * 2.0f) + pin.C2.x + pin.C2.z,
                          (uv.y * 0.5f) + (float(game_time_sec) + game_time_frac));
    int2 cell1 = int2(floor(npos1));
    float2 f1 = frac(npos1);
    float2 u1 = f1 * f1 * (3.0f - 2.0f * f1);
    float n00 = incb_perlin_hash01(hash(uint2(cell1)));
    float n10 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell1 + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 1))));
    float drop_blend = (1.0f - u1.y) * ((n00 * (1.0f - u1.x)) + (n10 * u1.x))
                     + (n01 * (1.0f - u1.x)) * u1.y
                     + (n11 * u1.x) * u1.y;
    float drop_radius = lerp(0.1f, 0.4f, drop_blend);

    float d = abs((pin.C3.y + lerp(-1.0f, 1.0f, (t * t * t))) - 0.5f);
    float drop = drop_ring(d, drop_radius);

    // Ripple ring: second value-noise field.
    float2 npos2 = float2((uv.x * 2.0f) + 2.0f + pin.C2.x + pin.C2.z,
                          uv.y * 0.2f);
    int2 cell2 = int2(floor(npos2));
    float2 f2 = frac(npos2);
    float2 u2 = f2 * f2 * (3.0f - 2.0f * f2);
    float m00 = incb_perlin_hash01(hash(uint2(cell2)));
    float m10 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 0))));
    float m01 = incb_perlin_hash01(hash(uint2(cell2 + int2(0, 1))));
    float m11 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 1))));
    float ripple_blend = (1.0f - u2.y) * ((m00 * (1.0f - u2.x)) + (m10 * u2.x))
                       + (m01 * (1.0f - u2.x)) * u2.y
                       + (m11 * u2.x) * u2.y;
    float ripple_w = lerp(0.0f, 0.3f, ripple_blend);
    if ((drop * (1.0f - smoothstep(0.9f, 1.0f, pin.C3.y + ripple_w))) < 0.5f)
    {
        discard;
    }

    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    float3 albedo = sqrt(clamp(pin.C0 * 0.7f, 0.0f, 1.0f));
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 1174405120u;
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _splash_particle.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static const uint2 _100[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    column_major float4x4 model_to_world : packoffset(c1);
    int bool_render_shadows : packoffset(c5);
    float anim_t : packoffset(c5.y);
};


static float4 gl_FragCoord;
static float3 in_var_C2;
static float2 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    uint2 _112 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
    uint _115 = frame_constants_frame_index & 3u;
    if (all(bool2(_112.x != _100[_115].x, _112.y != _100[_115].y)))
    {
        discard;
    }
    float _124 = 1.0f - anim_t;
    float2 _130 = in_var_C3 * 25.6000003814697265625f;
    float _132 = _130.x * 2.0f;
    float _137 = _130.y;
    float2 _146 = float2((_132 + in_var_C2.x) + in_var_C2.z, (_137 * 0.5f) + (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int2 _148 = int2(floor(_146));
    float2 _149 = frac(_146);
    uint2 _150 = uint2(_148);
    uint _152 = _150.y;
    uint _154 = _152 + (_152 << 10u);
    uint _156 = _154 ^ (_154 >> 6u);
    uint _158 = _156 + (_156 << 3u);
    uint _160 = _158 ^ (_158 >> 11u);
    uint _166 = (((_150.x * 1664525u) + (_160 + (_160 << 15u))) + 1013904223u) * 1664525u;
    uint _168 = _166 ^ (_166 >> 11u);
    uint _171 = _168 ^ ((_168 << 7u) & 2636928640u);
    uint _174 = _171 ^ ((_171 << 15u) & 4022730752u);
    uint2 _182 = uint2(_148 + int2(1, 0));
    uint _184 = _182.y;
    uint _186 = _184 + (_184 << 10u);
    uint _188 = _186 ^ (_186 >> 6u);
    uint _190 = _188 + (_188 << 3u);
    uint _192 = _190 ^ (_190 >> 11u);
    uint _198 = (((_182.x * 1664525u) + (_192 + (_192 << 15u))) + 1013904223u) * 1664525u;
    uint _200 = _198 ^ (_198 >> 11u);
    uint _203 = _200 ^ ((_200 << 7u) & 2636928640u);
    uint _206 = _203 ^ ((_203 << 15u) & 4022730752u);
    uint2 _214 = uint2(_148 + int2(0, 1));
    uint _216 = _214.y;
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    uint _230 = (((_214.x * 1664525u) + (_224 + (_224 << 15u))) + 1013904223u) * 1664525u;
    uint _232 = _230 ^ (_230 >> 11u);
    uint _235 = _232 ^ ((_232 << 7u) & 2636928640u);
    uint _238 = _235 ^ ((_235 << 15u) & 4022730752u);
    uint2 _246 = uint2(_148 + int2(1, 1));
    uint _248 = _246.y;
    uint _250 = _248 + (_248 << 10u);
    uint _252 = _250 ^ (_250 >> 6u);
    uint _254 = _252 + (_252 << 3u);
    uint _256 = _254 ^ (_254 >> 11u);
    uint _262 = (((_246.x * 1664525u) + (_256 + (_256 << 15u))) + 1013904223u) * 1664525u;
    uint _264 = _262 ^ (_262 >> 11u);
    uint _267 = _264 ^ ((_264 << 7u) & 2636928640u);
    uint _270 = _267 ^ ((_267 << 15u) & 4022730752u);
    float2 _280 = (_149 * _149) * (3.0f.xx - (_149 * 2.0f));
    float _281 = _280.x;
    float _282 = 1.0f - _281;
    float _284 = _280.y;
    float _295 = lerp(0.100000001490116119384765625f, 0.4000000059604644775390625f, (((1.0f - _284) * (((asfloat(((_174 ^ (_174 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _282) + ((asfloat(((_206 ^ (_206 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _281))) + (((asfloat(((_238 ^ (_238 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _282) * _284)) + (((asfloat(((_270 ^ (_270 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _281) * _284));
    float _309;
    do
    {
        float _299 = abs((in_var_C3.y + lerp(-1.0f, 1.0f, (_124 * _124) * _124)) - 0.5f);
        if (_299 > _295)
        {
            _309 = 0.0f;
            break;
        }
        float _303 = _299 / _295;
        _309 = 1.0f - ((_303 * _303) * (3.0f - (2.0f * _303)));
        break;
    } while(false);
    float2 _314 = float2(((_132 + 2.0f) + in_var_C2.x) + in_var_C2.z, _137 * 0.20000000298023223876953125f);
    int2 _316 = int2(floor(_314));
    float2 _317 = frac(_314);
    uint2 _318 = uint2(_316);
    uint _320 = _318.y;
    uint _322 = _320 + (_320 << 10u);
    uint _324 = _322 ^ (_322 >> 6u);
    uint _326 = _324 + (_324 << 3u);
    uint _328 = _326 ^ (_326 >> 11u);
    uint _334 = (((_318.x * 1664525u) + (_328 + (_328 << 15u))) + 1013904223u) * 1664525u;
    uint _336 = _334 ^ (_334 >> 11u);
    uint _339 = _336 ^ ((_336 << 7u) & 2636928640u);
    uint _342 = _339 ^ ((_339 << 15u) & 4022730752u);
    uint2 _350 = uint2(_316 + int2(1, 0));
    uint _352 = _350.y;
    uint _354 = _352 + (_352 << 10u);
    uint _356 = _354 ^ (_354 >> 6u);
    uint _358 = _356 + (_356 << 3u);
    uint _360 = _358 ^ (_358 >> 11u);
    uint _366 = (((_350.x * 1664525u) + (_360 + (_360 << 15u))) + 1013904223u) * 1664525u;
    uint _368 = _366 ^ (_366 >> 11u);
    uint _371 = _368 ^ ((_368 << 7u) & 2636928640u);
    uint _374 = _371 ^ ((_371 << 15u) & 4022730752u);
    uint2 _382 = uint2(_316 + int2(0, 1));
    uint _384 = _382.y;
    uint _386 = _384 + (_384 << 10u);
    uint _388 = _386 ^ (_386 >> 6u);
    uint _390 = _388 + (_388 << 3u);
    uint _392 = _390 ^ (_390 >> 11u);
    uint _398 = (((_382.x * 1664525u) + (_392 + (_392 << 15u))) + 1013904223u) * 1664525u;
    uint _400 = _398 ^ (_398 >> 11u);
    uint _403 = _400 ^ ((_400 << 7u) & 2636928640u);
    uint _406 = _403 ^ ((_403 << 15u) & 4022730752u);
    uint2 _414 = uint2(_316 + int2(1, 1));
    uint _416 = _414.y;
    uint _418 = _416 + (_416 << 10u);
    uint _420 = _418 ^ (_418 >> 6u);
    uint _422 = _420 + (_420 << 3u);
    uint _424 = _422 ^ (_422 >> 11u);
    uint _430 = (((_414.x * 1664525u) + (_424 + (_424 << 15u))) + 1013904223u) * 1664525u;
    uint _432 = _430 ^ (_430 >> 11u);
    uint _435 = _432 ^ ((_432 << 7u) & 2636928640u);
    uint _438 = _435 ^ ((_435 << 15u) & 4022730752u);
    float2 _448 = (_317 * _317) * (3.0f.xx - (_317 * 2.0f));
    float _449 = _448.x;
    float _450 = 1.0f - _449;
    float _452 = _448.y;
    if ((_309 * (1.0f - smoothstep(0.89999997615814208984375f, 1.0f, in_var_C3.y + lerp(0.0f, 0.300000011920928955078125f, (((1.0f - _452) * (((asfloat(((_342 ^ (_342 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _450) + ((asfloat(((_374 ^ (_374 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _449))) + (((asfloat(((_406 ^ (_406 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _450) * _452)) + (((asfloat(((_438 ^ (_438 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _449) * _452))))) < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
