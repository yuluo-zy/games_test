// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_flower.raster.hlsl | B nani raster | bins: _nani_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani flower: VS selects the flower color by glade theme + flower type,
// sways the stem with a 2D wind value-noise lattice and applies
// grass_object_push; PS packs (normal, albedo) into the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "garden/defs.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint2 grass_object_push_dims;
int bool_render_shadows;

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<FlowerXform> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t16);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float wind_bend : TEXCOORD3;
    float3 autumn_color : TEXCOORD4;
    float3 flowery_color : TEXCOORD5;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].transform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));

    uint origin_hash = uint((origin.x * 20.0f) - (origin.z * 20.0f) + 2000.0f);
    origin_hash = hash(origin_hash);
    float color_rand = hash2_rand(origin_hash);

    bool theme_a = glade_theme_id == 4u;
    bool theme_b = glade_theme_id == 1u;
    float3 base_color;
    float color_var;
    if (theme_a)
    {
        base_color = lerp(vin.autumn_color, 0.7f.xxx, 0.5f.xxx);
        color_var = color_rand * 0.5f;
    }
    else if (theme_b)
    {
        base_color = vin.flowery_color;
        color_var = color_rand * 0.5f;
    }
    else if (glade_theme_id == 3u && nani_instance_data[idx].flower_type == 0u)
    {
        base_color = vin.Vertex_Color.z > 0.2f
            ? float3(0.21f, 0.21f, 0.35f)
            : vin.Vertex_Color;
        color_var = color_rand;
    }
    else
    {
        base_color = vin.Vertex_Color;
        color_var = color_rand;
    }

    float3 shade_color;
    if (nani_instance_data[idx].color.x > -1.0f)
    {
        shade_color = theme_a
            ? float3(0.207063f, 0.164317f, 0.291863f)
            : (theme_b
                ? float3(0.310594f, 0.246475f, 0.437794f)
                : (glade_theme_id == 3u
                    ? float3(0.310594f, 0.246475f, 0.437794f)
                    : float3(0.126791f, 0.305342f, 0.495884f)));
    }
    else
    {
        shade_color = base_color;
    }

    float2 wind_uv = (origin.xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    world.x -= (vin.wind_bend * vin.wind_bend) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.0735f);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * vin.wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = pow(shade_color, lerp(1.0f, 2.5f, color_var).xxx);
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(pin.C1);
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(pin.C0, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
    }
    else
    {
        albedo = pin.C0;
    }
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 855638016u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 1711276032u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint2 _Globals_grass_object_push_dims : packoffset(c0.y);
    int _Globals_bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Color : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float in_var_wind_bend : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_autumn_color : TEXCOORD4;
    [[vk::location(5)]] float3 in_var_flowery_color : TEXCOORD5;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _174;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _174.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _193 = transpose(float4x3(float3(_174.data[0], _174.data[1], _174.data[2]), float3(_174.data[3], _174.data[4], _174.data[5]), float3(_174.data[6], _174.data[7], _174.data[8]), float3(_174.data[9], _174.data[10], _174.data[11])));
    float3 _194 = mul(_193, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _199 = mul(_193, float4(in_var_Vertex_Position, 1.0f));
    uint _206 = uint(((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f);
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    float _220 = asfloat(((_214 + (_214 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    bool _224 = !(frame_constants_glade_theme_id == 2u);
    bool _228;
    if (_224)
    {
        _228 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        _228 = true;
    }
    float3 _259;
    float _260;
    if (_228)
    {
        float3 _257;
        if (frame_constants_glade_theme_id == 4u)
        {
            _257 = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            _257 = in_var_autumn_color;
        }
        _259 = _257;
        _260 = _220 * 0.5f;
    }
    else
    {
        float3 _251;
        float _252;
        if (frame_constants_glade_theme_id == 1u)
        {
            _251 = in_var_flowery_color;
            _252 = _220 * 0.5f;
        }
        else
        {
            bool _242;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                _242 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                _242 = false;
            }
            float3 _249;
            if (_242)
            {
                bool3 _247 = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                _249 = float3(_247.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, _247.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, _247.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                _249 = in_var_Vertex_Color;
            }
            _251 = _249;
            _252 = _220;
        }
        _259 = _251;
        _260 = _252;
    }
    float3 _282;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool _269;
        if (_224)
        {
            _269 = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            _269 = true;
        }
        float3 _281;
        if (_269)
        {
            _281 = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 _280;
            if (frame_constants_glade_theme_id == 1u)
            {
                _280 = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 _278 = (frame_constants_glade_theme_id == 3u).xxx;
                _280 = float3(_278.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, _278.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, _278.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            _281 = _280;
        }
        _282 = _281;
    }
    else
    {
        _282 = _259;
    }
    float2 _283 = _194.xz;
    float2 _293 = (_283 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _295 = int2(floor(_293));
    float2 _296 = frac(_293);
    uint2 _297 = uint2(_295);
    uint _299 = _297.y;
    uint _301 = _299 + (_299 << 10u);
    uint _303 = _301 ^ (_301 >> 6u);
    uint _305 = _303 + (_303 << 3u);
    uint _307 = _305 ^ (_305 >> 11u);
    uint _313 = (((_297.x * 1664525u) + (_307 + (_307 << 15u))) + 1013904223u) * 1664525u;
    uint _315 = _313 ^ (_313 >> 11u);
    uint _318 = _315 ^ ((_315 << 7u) & 2636928640u);
    uint _321 = _318 ^ ((_318 << 15u) & 4022730752u);
    uint2 _329 = uint2(_295 + int2(1, 0));
    uint _331 = _329.y;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_329.x * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint2 _361 = uint2(_295 + int2(0, 1));
    uint _363 = _361.y;
    uint _365 = _363 + (_363 << 10u);
    uint _367 = _365 ^ (_365 >> 6u);
    uint _369 = _367 + (_367 << 3u);
    uint _371 = _369 ^ (_369 >> 11u);
    uint _377 = (((_361.x * 1664525u) + (_371 + (_371 << 15u))) + 1013904223u) * 1664525u;
    uint _379 = _377 ^ (_377 >> 11u);
    uint _382 = _379 ^ ((_379 << 7u) & 2636928640u);
    uint _385 = _382 ^ ((_382 << 15u) & 4022730752u);
    uint2 _393 = uint2(_295 + int2(1, 1));
    uint _395 = _393.y;
    uint _397 = _395 + (_395 << 10u);
    uint _399 = _397 ^ (_397 >> 6u);
    uint _401 = _399 + (_399 << 3u);
    uint _403 = _401 ^ (_401 >> 11u);
    uint _409 = (((_393.x * 1664525u) + (_403 + (_403 << 15u))) + 1013904223u) * 1664525u;
    uint _411 = _409 ^ (_409 >> 11u);
    uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
    uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
    float2 _427 = (_296 * _296) * (3.0f.xx - (_296 * 2.0f));
    float _428 = _427.x;
    float _429 = 1.0f - _428;
    float _431 = _427.y;
    _199.x = _199.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - _431) * (((asfloat(((_321 ^ (_321 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) + ((asfloat(((_353 ^ (_353 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428))) + (((asfloat(((_385 ^ (_385 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) * _431)) + (((asfloat(((_417 ^ (_417 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) * _431)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _457 = grass_object_push.SampleLevel(sampler_default, (_283 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _459 = clamp(_457.z, 0.0f, 1.0f);
    float3 _469;
    if (_459 > 0.0f)
    {
        float2 _467 = _199.xz + ((_457.xy * _459) * in_var_wind_bend);
        _469 = float3(_467.x, _199.y, _467.y);
    }
    else
    {
        _469 = _199;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_469, 1.0f));
    out_var_C0 = pow(_282, lerp(1.0f, 2.5f, _260).xxx);
    out_var_C1 = mul(_193, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _469;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint2 _Globals_grass_object_push_dims : packoffset(c0.y);
    int _Globals_bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Color : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float in_var_wind_bend : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_autumn_color : TEXCOORD4;
    [[vk::location(5)]] float3 in_var_flowery_color : TEXCOORD5;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _174;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _174.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _193 = transpose(float4x3(float3(_174.data[0], _174.data[1], _174.data[2]), float3(_174.data[3], _174.data[4], _174.data[5]), float3(_174.data[6], _174.data[7], _174.data[8]), float3(_174.data[9], _174.data[10], _174.data[11])));
    float3 _194 = mul(_193, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _199 = mul(_193, float4(in_var_Vertex_Position, 1.0f));
    uint _206 = uint(((_194.x * 20.0f) - (_194.z * 20.0f)) + 2000.0f);
    uint _208 = _206 + (_206 << 10u);
    uint _210 = _208 ^ (_208 >> 6u);
    uint _212 = _210 + (_210 << 3u);
    uint _214 = _212 ^ (_212 >> 11u);
    float _220 = asfloat(((_214 + (_214 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    bool _224 = !(frame_constants_glade_theme_id == 2u);
    bool _228;
    if (_224)
    {
        _228 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        _228 = true;
    }
    float3 _259;
    float _260;
    if (_228)
    {
        float3 _257;
        if (frame_constants_glade_theme_id == 4u)
        {
            _257 = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            _257 = in_var_autumn_color;
        }
        _259 = _257;
        _260 = _220 * 0.5f;
    }
    else
    {
        float3 _251;
        float _252;
        if (frame_constants_glade_theme_id == 1u)
        {
            _251 = in_var_flowery_color;
            _252 = _220 * 0.5f;
        }
        else
        {
            bool _242;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                _242 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                _242 = false;
            }
            float3 _249;
            if (_242)
            {
                bool3 _247 = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                _249 = float3(_247.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, _247.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, _247.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                _249 = in_var_Vertex_Color;
            }
            _251 = _249;
            _252 = _220;
        }
        _259 = _251;
        _260 = _252;
    }
    float3 _282;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool _269;
        if (_224)
        {
            _269 = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            _269 = true;
        }
        float3 _281;
        if (_269)
        {
            _281 = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 _280;
            if (frame_constants_glade_theme_id == 1u)
            {
                _280 = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 _278 = (frame_constants_glade_theme_id == 3u).xxx;
                _280 = float3(_278.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, _278.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, _278.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            _281 = _280;
        }
        _282 = _281;
    }
    else
    {
        _282 = _259;
    }
    float2 _283 = _194.xz;
    float2 _293 = (_283 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _295 = int2(floor(_293));
    float2 _296 = frac(_293);
    uint2 _297 = uint2(_295);
    uint _299 = _297.y;
    uint _301 = _299 + (_299 << 10u);
    uint _303 = _301 ^ (_301 >> 6u);
    uint _305 = _303 + (_303 << 3u);
    uint _307 = _305 ^ (_305 >> 11u);
    uint _313 = (((_297.x * 1664525u) + (_307 + (_307 << 15u))) + 1013904223u) * 1664525u;
    uint _315 = _313 ^ (_313 >> 11u);
    uint _318 = _315 ^ ((_315 << 7u) & 2636928640u);
    uint _321 = _318 ^ ((_318 << 15u) & 4022730752u);
    uint2 _329 = uint2(_295 + int2(1, 0));
    uint _331 = _329.y;
    uint _333 = _331 + (_331 << 10u);
    uint _335 = _333 ^ (_333 >> 6u);
    uint _337 = _335 + (_335 << 3u);
    uint _339 = _337 ^ (_337 >> 11u);
    uint _345 = (((_329.x * 1664525u) + (_339 + (_339 << 15u))) + 1013904223u) * 1664525u;
    uint _347 = _345 ^ (_345 >> 11u);
    uint _350 = _347 ^ ((_347 << 7u) & 2636928640u);
    uint _353 = _350 ^ ((_350 << 15u) & 4022730752u);
    uint2 _361 = uint2(_295 + int2(0, 1));
    uint _363 = _361.y;
    uint _365 = _363 + (_363 << 10u);
    uint _367 = _365 ^ (_365 >> 6u);
    uint _369 = _367 + (_367 << 3u);
    uint _371 = _369 ^ (_369 >> 11u);
    uint _377 = (((_361.x * 1664525u) + (_371 + (_371 << 15u))) + 1013904223u) * 1664525u;
    uint _379 = _377 ^ (_377 >> 11u);
    uint _382 = _379 ^ ((_379 << 7u) & 2636928640u);
    uint _385 = _382 ^ ((_382 << 15u) & 4022730752u);
    uint2 _393 = uint2(_295 + int2(1, 1));
    uint _395 = _393.y;
    uint _397 = _395 + (_395 << 10u);
    uint _399 = _397 ^ (_397 >> 6u);
    uint _401 = _399 + (_399 << 3u);
    uint _403 = _401 ^ (_401 >> 11u);
    uint _409 = (((_393.x * 1664525u) + (_403 + (_403 << 15u))) + 1013904223u) * 1664525u;
    uint _411 = _409 ^ (_409 >> 11u);
    uint _414 = _411 ^ ((_411 << 7u) & 2636928640u);
    uint _417 = _414 ^ ((_414 << 15u) & 4022730752u);
    float2 _427 = (_296 * _296) * (3.0f.xx - (_296 * 2.0f));
    float _428 = _427.x;
    float _429 = 1.0f - _428;
    float _431 = _427.y;
    _199.x = _199.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - _431) * (((asfloat(((_321 ^ (_321 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) + ((asfloat(((_353 ^ (_353 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428))) + (((asfloat(((_385 ^ (_385 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _429) * _431)) + (((asfloat(((_417 ^ (_417 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _428) * _431)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _457 = grass_object_push.SampleLevel(sampler_default, (_283 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _459 = clamp(_457.z, 0.0f, 1.0f);
    float3 _469;
    if (_459 > 0.0f)
    {
        float2 _467 = _199.xz + ((_457.xy * _459) * in_var_wind_bend);
        _469 = float3(_467.x, _199.y, _467.y);
    }
    else
    {
        _469 = _199;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_469, 1.0f));
    out_var_C0 = pow(_282, lerp(1.0f, 2.5f, _260).xxx);
    out_var_C1 = mul(_193, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _469;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
