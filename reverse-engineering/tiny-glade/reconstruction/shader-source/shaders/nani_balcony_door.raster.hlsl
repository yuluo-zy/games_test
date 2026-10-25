// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_balcony_door.raster.hlsl | B nani raster | bins: _nani_balcony_door.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani balcony door: VS jitters the door surface with a 3D value-noise lattice;
// PS dithers semitransparent doors, colors the door (window palette for the
// wood part) and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/color_library.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/window_color_customization.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
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

int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    uint color_id;
    uint pad0;
    uint pad1;
};

Texture2DArray<float4> window_colors : register(t14);
StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);

uint hash3(uint3 p)
{
    p.y *= 1664525u;
    uint yz = p.z;
    yz += (yz << 10u);
    yz ^= (yz >> 6u);
    yz += (yz << 3u);
    yz ^= (yz >> 11u);
    p.y += yz + (yz << 15u);
    p.y += 1013904223u;
    p.y *= 1664525u;
    uint y = p.y ^ (p.y >> 11u);
    y = y ^ ((y << 7u) & 2636928640u);
    y = y ^ ((y << 15u) & 4022730752u);
    p.x *= 1664525u;
    p.x += y ^ (y >> 18u);
    p.x += 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    x = x ^ (x >> 18u);
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

float2 corner_rand(uint3 cell)
{
    uint h = hash3(cell);
    float r1 = asfloat((h & 8388607u) | 1065353216u) - 1.0f;
    h = h + (h << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float r2 = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    return float2(r1, r2);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int is_metal_part: is_metal_part;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));

    // 3D value-noise door jitter (8-corner trilinear hash lattice).
    float3 scaled = world * 0.4f;
    int3 cell = int3(floor(scaled));
    float3 fr = frac(scaled);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float2 n = float2(0.0f, 0.0f);
    n += corner_rand(uint3(cell)) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 0, 0))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 1, 0))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 1, 0))) * f.x * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 0, 1))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(1, 0, 1))) * f.x * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(0, 1, 1))) * (1.0f - f.x) * f.y * f.z;
    n += corner_rand(uint3(cell + int3(1, 1, 1))) * f.x * f.y * f.z;
    float2 offset = clamp((n - 0.5f.xx) * 2.5f, -0.5f.xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.06f);

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = vin.is_metal_part;
    o.C5 = int(nani_instance_data[idx].color_id);
    o.C6 = nani_instance_data[idx].is_semitransparent;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = pin.C6 != 0u && bool_render_shadows != 0;
    if (dither)
    {
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    float3 albedo;
    if (pin.C4 != 1)
    {
        uint seed = uint(pin.C5);
        seed = hash(seed);
        float rnd = asfloat((seed & 8388607u) | 1065353216u) - 1.0f;
        float3 door_color = window_colors.Sample(sampler_llr, float3((pin.C3 + rnd.xx) + (pin.C2.xz * 0.1f), float(pin.C5))).xyz;
        if (pin.C5 == 0)
        {
            door_color = door_color * 0.7f;
        }
        albedo = pin.C5 == 0 ? lerp(door_color, pin.C0, 0.3f.xxx) : door_color;
    }
    else
    {
        albedo = pin.C0;
    }

    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_balcony_door.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _165 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _170 = mul(_165, float4(in_var_Vertex_Position, 1.0f));
    float3 _171 = _170 * 0.4000000059604644775390625f;
    int3 _173 = int3(floor(_171));
    float3 _174 = frac(_171);
    uint3 _175 = uint3(_173);
    uint _178 = _175.z;
    uint _180 = _178 + (_178 << 10u);
    uint _182 = _180 ^ (_180 >> 6u);
    uint _184 = _182 + (_182 << 3u);
    uint _186 = _184 ^ (_184 >> 11u);
    uint _192 = (((_175.y * 1664525u) + (_186 + (_186 << 15u))) + 1013904223u) * 1664525u;
    uint _194 = _192 ^ (_192 >> 11u);
    uint _197 = _194 ^ ((_194 << 7u) & 2636928640u);
    uint _200 = _197 ^ ((_197 << 15u) & 4022730752u);
    uint _206 = (((_175.x * 1664525u) + (_200 ^ (_200 >> 18u))) + 1013904223u) * 1664525u;
    uint _208 = _206 ^ (_206 >> 11u);
    uint _211 = _208 ^ ((_208 << 7u) & 2636928640u);
    uint _214 = _211 ^ ((_211 << 15u) & 4022730752u);
    uint _216 = _214 ^ (_214 >> 18u);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    uint3 _237 = uint3(_173 + int3(1, 0, 0));
    uint _240 = _237.z;
    uint _242 = _240 + (_240 << 10u);
    uint _244 = _242 ^ (_242 >> 6u);
    uint _246 = _244 + (_244 << 3u);
    uint _248 = _246 ^ (_246 >> 11u);
    uint _254 = (((_237.y * 1664525u) + (_248 + (_248 << 15u))) + 1013904223u) * 1664525u;
    uint _256 = _254 ^ (_254 >> 11u);
    uint _259 = _256 ^ ((_256 << 7u) & 2636928640u);
    uint _262 = _259 ^ ((_259 << 15u) & 4022730752u);
    uint _268 = (((_237.x * 1664525u) + (_262 ^ (_262 >> 18u))) + 1013904223u) * 1664525u;
    uint _270 = _268 ^ (_268 >> 11u);
    uint _273 = _270 ^ ((_270 << 7u) & 2636928640u);
    uint _276 = _273 ^ ((_273 << 15u) & 4022730752u);
    uint _278 = _276 ^ (_276 >> 18u);
    uint _280 = _278 + (_278 << 10u);
    uint _282 = _280 ^ (_280 >> 6u);
    uint _284 = _282 + (_282 << 3u);
    uint _286 = _284 ^ (_284 >> 11u);
    uint3 _299 = uint3(_173 + int3(0, 1, 0));
    uint _302 = _299.z;
    uint _304 = _302 + (_302 << 10u);
    uint _306 = _304 ^ (_304 >> 6u);
    uint _308 = _306 + (_306 << 3u);
    uint _310 = _308 ^ (_308 >> 11u);
    uint _316 = (((_299.y * 1664525u) + (_310 + (_310 << 15u))) + 1013904223u) * 1664525u;
    uint _318 = _316 ^ (_316 >> 11u);
    uint _321 = _318 ^ ((_318 << 7u) & 2636928640u);
    uint _324 = _321 ^ ((_321 << 15u) & 4022730752u);
    uint _330 = (((_299.x * 1664525u) + (_324 ^ (_324 >> 18u))) + 1013904223u) * 1664525u;
    uint _332 = _330 ^ (_330 >> 11u);
    uint _335 = _332 ^ ((_332 << 7u) & 2636928640u);
    uint _338 = _335 ^ ((_335 << 15u) & 4022730752u);
    uint _340 = _338 ^ (_338 >> 18u);
    uint _342 = _340 + (_340 << 10u);
    uint _344 = _342 ^ (_342 >> 6u);
    uint _346 = _344 + (_344 << 3u);
    uint _348 = _346 ^ (_346 >> 11u);
    uint3 _361 = uint3(_173 + int3(1, 1, 0));
    uint _364 = _361.z;
    uint _366 = _364 + (_364 << 10u);
    uint _368 = _366 ^ (_366 >> 6u);
    uint _370 = _368 + (_368 << 3u);
    uint _372 = _370 ^ (_370 >> 11u);
    uint _378 = (((_361.y * 1664525u) + (_372 + (_372 << 15u))) + 1013904223u) * 1664525u;
    uint _380 = _378 ^ (_378 >> 11u);
    uint _383 = _380 ^ ((_380 << 7u) & 2636928640u);
    uint _386 = _383 ^ ((_383 << 15u) & 4022730752u);
    uint _392 = (((_361.x * 1664525u) + (_386 ^ (_386 >> 18u))) + 1013904223u) * 1664525u;
    uint _394 = _392 ^ (_392 >> 11u);
    uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
    uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
    uint _402 = _400 ^ (_400 >> 18u);
    uint _404 = _402 + (_402 << 10u);
    uint _406 = _404 ^ (_404 >> 6u);
    uint _408 = _406 + (_406 << 3u);
    uint _410 = _408 ^ (_408 >> 11u);
    uint3 _423 = uint3(_173 + int3(0, 0, 1));
    uint _426 = _423.z;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_423.y * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint _454 = (((_423.x * 1664525u) + (_448 ^ (_448 >> 18u))) + 1013904223u) * 1664525u;
    uint _456 = _454 ^ (_454 >> 11u);
    uint _459 = _456 ^ ((_456 << 7u) & 2636928640u);
    uint _462 = _459 ^ ((_459 << 15u) & 4022730752u);
    uint _464 = _462 ^ (_462 >> 18u);
    uint _466 = _464 + (_464 << 10u);
    uint _468 = _466 ^ (_466 >> 6u);
    uint _470 = _468 + (_468 << 3u);
    uint _472 = _470 ^ (_470 >> 11u);
    uint3 _485 = uint3(_173 + int3(1, 0, 1));
    uint _488 = _485.z;
    uint _490 = _488 + (_488 << 10u);
    uint _492 = _490 ^ (_490 >> 6u);
    uint _494 = _492 + (_492 << 3u);
    uint _496 = _494 ^ (_494 >> 11u);
    uint _502 = (((_485.y * 1664525u) + (_496 + (_496 << 15u))) + 1013904223u) * 1664525u;
    uint _504 = _502 ^ (_502 >> 11u);
    uint _507 = _504 ^ ((_504 << 7u) & 2636928640u);
    uint _510 = _507 ^ ((_507 << 15u) & 4022730752u);
    uint _516 = (((_485.x * 1664525u) + (_510 ^ (_510 >> 18u))) + 1013904223u) * 1664525u;
    uint _518 = _516 ^ (_516 >> 11u);
    uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
    uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
    uint _526 = _524 ^ (_524 >> 18u);
    uint _528 = _526 + (_526 << 10u);
    uint _530 = _528 ^ (_528 >> 6u);
    uint _532 = _530 + (_530 << 3u);
    uint _534 = _532 ^ (_532 >> 11u);
    uint3 _547 = uint3(_173 + int3(0, 1, 1));
    uint _550 = _547.z;
    uint _552 = _550 + (_550 << 10u);
    uint _554 = _552 ^ (_552 >> 6u);
    uint _556 = _554 + (_554 << 3u);
    uint _558 = _556 ^ (_556 >> 11u);
    uint _564 = (((_547.y * 1664525u) + (_558 + (_558 << 15u))) + 1013904223u) * 1664525u;
    uint _566 = _564 ^ (_564 >> 11u);
    uint _569 = _566 ^ ((_566 << 7u) & 2636928640u);
    uint _572 = _569 ^ ((_569 << 15u) & 4022730752u);
    uint _578 = (((_547.x * 1664525u) + (_572 ^ (_572 >> 18u))) + 1013904223u) * 1664525u;
    uint _580 = _578 ^ (_578 >> 11u);
    uint _583 = _580 ^ ((_580 << 7u) & 2636928640u);
    uint _586 = _583 ^ ((_583 << 15u) & 4022730752u);
    uint _588 = _586 ^ (_586 >> 18u);
    uint _590 = _588 + (_588 << 10u);
    uint _592 = _590 ^ (_590 >> 6u);
    uint _594 = _592 + (_592 << 3u);
    uint _596 = _594 ^ (_594 >> 11u);
    uint3 _609 = uint3(_173 + int3(1, 1, 1));
    uint _612 = _609.z;
    uint _614 = _612 + (_612 << 10u);
    uint _616 = _614 ^ (_614 >> 6u);
    uint _618 = _616 + (_616 << 3u);
    uint _620 = _618 ^ (_618 >> 11u);
    uint _626 = (((_609.y * 1664525u) + (_620 + (_620 << 15u))) + 1013904223u) * 1664525u;
    uint _628 = _626 ^ (_626 >> 11u);
    uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
    uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
    uint _640 = (((_609.x * 1664525u) + (_634 ^ (_634 >> 18u))) + 1013904223u) * 1664525u;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    uint _650 = _648 ^ (_648 >> 18u);
    uint _652 = _650 + (_650 << 10u);
    uint _654 = _652 ^ (_652 >> 6u);
    uint _656 = _654 + (_654 << 3u);
    uint _658 = _656 ^ (_656 >> 11u);
    float3 _673 = (_174 * _174) * (3.0f.xxx - (_174 * 2.0f));
    float _674 = _673.x;
    float _675 = 1.0f - _674;
    float _677 = _673.y;
    float _678 = 1.0f - _677;
    float _680 = _673.z;
    float _681 = 1.0f - _680;
    float2 _698 = ((((((float2(asfloat((_216 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _681) + (((float2(asfloat((_278 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_286 + (_286 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _681)) + (((float2(asfloat((_340 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_348 + (_348 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _681)) + (((float2(asfloat((_402 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_410 + (_410 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _681)) + (((float2(asfloat((_464 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _680);
    float2 _713 = clamp(((((_698 + (((float2(asfloat((_526 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_534 + (_534 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _680)) + (((float2(asfloat((_588 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _680)) + (((float2(asfloat((_650 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_658 + (_658 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _680)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _718 = _170 + (float3(_713.x, 0.0f, _713.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_718, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_165, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _718;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
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

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_balcony_door.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _142;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _142.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 _165 = transpose(float4x3(float3(_142.data[0], _142.data[1], _142.data[2]), float3(_142.data[3], _142.data[4], _142.data[5]), float3(_142.data[6], _142.data[7], _142.data[8]), float3(_142.data[9], _142.data[10], _142.data[11])));
    float3 _170 = mul(_165, float4(in_var_Vertex_Position, 1.0f));
    float3 _171 = _170 * 0.4000000059604644775390625f;
    int3 _173 = int3(floor(_171));
    float3 _174 = frac(_171);
    uint3 _175 = uint3(_173);
    uint _178 = _175.z;
    uint _180 = _178 + (_178 << 10u);
    uint _182 = _180 ^ (_180 >> 6u);
    uint _184 = _182 + (_182 << 3u);
    uint _186 = _184 ^ (_184 >> 11u);
    uint _192 = (((_175.y * 1664525u) + (_186 + (_186 << 15u))) + 1013904223u) * 1664525u;
    uint _194 = _192 ^ (_192 >> 11u);
    uint _197 = _194 ^ ((_194 << 7u) & 2636928640u);
    uint _200 = _197 ^ ((_197 << 15u) & 4022730752u);
    uint _206 = (((_175.x * 1664525u) + (_200 ^ (_200 >> 18u))) + 1013904223u) * 1664525u;
    uint _208 = _206 ^ (_206 >> 11u);
    uint _211 = _208 ^ ((_208 << 7u) & 2636928640u);
    uint _214 = _211 ^ ((_211 << 15u) & 4022730752u);
    uint _216 = _214 ^ (_214 >> 18u);
    uint _218 = _216 + (_216 << 10u);
    uint _220 = _218 ^ (_218 >> 6u);
    uint _222 = _220 + (_220 << 3u);
    uint _224 = _222 ^ (_222 >> 11u);
    uint3 _237 = uint3(_173 + int3(1, 0, 0));
    uint _240 = _237.z;
    uint _242 = _240 + (_240 << 10u);
    uint _244 = _242 ^ (_242 >> 6u);
    uint _246 = _244 + (_244 << 3u);
    uint _248 = _246 ^ (_246 >> 11u);
    uint _254 = (((_237.y * 1664525u) + (_248 + (_248 << 15u))) + 1013904223u) * 1664525u;
    uint _256 = _254 ^ (_254 >> 11u);
    uint _259 = _256 ^ ((_256 << 7u) & 2636928640u);
    uint _262 = _259 ^ ((_259 << 15u) & 4022730752u);
    uint _268 = (((_237.x * 1664525u) + (_262 ^ (_262 >> 18u))) + 1013904223u) * 1664525u;
    uint _270 = _268 ^ (_268 >> 11u);
    uint _273 = _270 ^ ((_270 << 7u) & 2636928640u);
    uint _276 = _273 ^ ((_273 << 15u) & 4022730752u);
    uint _278 = _276 ^ (_276 >> 18u);
    uint _280 = _278 + (_278 << 10u);
    uint _282 = _280 ^ (_280 >> 6u);
    uint _284 = _282 + (_282 << 3u);
    uint _286 = _284 ^ (_284 >> 11u);
    uint3 _299 = uint3(_173 + int3(0, 1, 0));
    uint _302 = _299.z;
    uint _304 = _302 + (_302 << 10u);
    uint _306 = _304 ^ (_304 >> 6u);
    uint _308 = _306 + (_306 << 3u);
    uint _310 = _308 ^ (_308 >> 11u);
    uint _316 = (((_299.y * 1664525u) + (_310 + (_310 << 15u))) + 1013904223u) * 1664525u;
    uint _318 = _316 ^ (_316 >> 11u);
    uint _321 = _318 ^ ((_318 << 7u) & 2636928640u);
    uint _324 = _321 ^ ((_321 << 15u) & 4022730752u);
    uint _330 = (((_299.x * 1664525u) + (_324 ^ (_324 >> 18u))) + 1013904223u) * 1664525u;
    uint _332 = _330 ^ (_330 >> 11u);
    uint _335 = _332 ^ ((_332 << 7u) & 2636928640u);
    uint _338 = _335 ^ ((_335 << 15u) & 4022730752u);
    uint _340 = _338 ^ (_338 >> 18u);
    uint _342 = _340 + (_340 << 10u);
    uint _344 = _342 ^ (_342 >> 6u);
    uint _346 = _344 + (_344 << 3u);
    uint _348 = _346 ^ (_346 >> 11u);
    uint3 _361 = uint3(_173 + int3(1, 1, 0));
    uint _364 = _361.z;
    uint _366 = _364 + (_364 << 10u);
    uint _368 = _366 ^ (_366 >> 6u);
    uint _370 = _368 + (_368 << 3u);
    uint _372 = _370 ^ (_370 >> 11u);
    uint _378 = (((_361.y * 1664525u) + (_372 + (_372 << 15u))) + 1013904223u) * 1664525u;
    uint _380 = _378 ^ (_378 >> 11u);
    uint _383 = _380 ^ ((_380 << 7u) & 2636928640u);
    uint _386 = _383 ^ ((_383 << 15u) & 4022730752u);
    uint _392 = (((_361.x * 1664525u) + (_386 ^ (_386 >> 18u))) + 1013904223u) * 1664525u;
    uint _394 = _392 ^ (_392 >> 11u);
    uint _397 = _394 ^ ((_394 << 7u) & 2636928640u);
    uint _400 = _397 ^ ((_397 << 15u) & 4022730752u);
    uint _402 = _400 ^ (_400 >> 18u);
    uint _404 = _402 + (_402 << 10u);
    uint _406 = _404 ^ (_404 >> 6u);
    uint _408 = _406 + (_406 << 3u);
    uint _410 = _408 ^ (_408 >> 11u);
    uint3 _423 = uint3(_173 + int3(0, 0, 1));
    uint _426 = _423.z;
    uint _428 = _426 + (_426 << 10u);
    uint _430 = _428 ^ (_428 >> 6u);
    uint _432 = _430 + (_430 << 3u);
    uint _434 = _432 ^ (_432 >> 11u);
    uint _440 = (((_423.y * 1664525u) + (_434 + (_434 << 15u))) + 1013904223u) * 1664525u;
    uint _442 = _440 ^ (_440 >> 11u);
    uint _445 = _442 ^ ((_442 << 7u) & 2636928640u);
    uint _448 = _445 ^ ((_445 << 15u) & 4022730752u);
    uint _454 = (((_423.x * 1664525u) + (_448 ^ (_448 >> 18u))) + 1013904223u) * 1664525u;
    uint _456 = _454 ^ (_454 >> 11u);
    uint _459 = _456 ^ ((_456 << 7u) & 2636928640u);
    uint _462 = _459 ^ ((_459 << 15u) & 4022730752u);
    uint _464 = _462 ^ (_462 >> 18u);
    uint _466 = _464 + (_464 << 10u);
    uint _468 = _466 ^ (_466 >> 6u);
    uint _470 = _468 + (_468 << 3u);
    uint _472 = _470 ^ (_470 >> 11u);
    uint3 _485 = uint3(_173 + int3(1, 0, 1));
    uint _488 = _485.z;
    uint _490 = _488 + (_488 << 10u);
    uint _492 = _490 ^ (_490 >> 6u);
    uint _494 = _492 + (_492 << 3u);
    uint _496 = _494 ^ (_494 >> 11u);
    uint _502 = (((_485.y * 1664525u) + (_496 + (_496 << 15u))) + 1013904223u) * 1664525u;
    uint _504 = _502 ^ (_502 >> 11u);
    uint _507 = _504 ^ ((_504 << 7u) & 2636928640u);
    uint _510 = _507 ^ ((_507 << 15u) & 4022730752u);
    uint _516 = (((_485.x * 1664525u) + (_510 ^ (_510 >> 18u))) + 1013904223u) * 1664525u;
    uint _518 = _516 ^ (_516 >> 11u);
    uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
    uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
    uint _526 = _524 ^ (_524 >> 18u);
    uint _528 = _526 + (_526 << 10u);
    uint _530 = _528 ^ (_528 >> 6u);
    uint _532 = _530 + (_530 << 3u);
    uint _534 = _532 ^ (_532 >> 11u);
    uint3 _547 = uint3(_173 + int3(0, 1, 1));
    uint _550 = _547.z;
    uint _552 = _550 + (_550 << 10u);
    uint _554 = _552 ^ (_552 >> 6u);
    uint _556 = _554 + (_554 << 3u);
    uint _558 = _556 ^ (_556 >> 11u);
    uint _564 = (((_547.y * 1664525u) + (_558 + (_558 << 15u))) + 1013904223u) * 1664525u;
    uint _566 = _564 ^ (_564 >> 11u);
    uint _569 = _566 ^ ((_566 << 7u) & 2636928640u);
    uint _572 = _569 ^ ((_569 << 15u) & 4022730752u);
    uint _578 = (((_547.x * 1664525u) + (_572 ^ (_572 >> 18u))) + 1013904223u) * 1664525u;
    uint _580 = _578 ^ (_578 >> 11u);
    uint _583 = _580 ^ ((_580 << 7u) & 2636928640u);
    uint _586 = _583 ^ ((_583 << 15u) & 4022730752u);
    uint _588 = _586 ^ (_586 >> 18u);
    uint _590 = _588 + (_588 << 10u);
    uint _592 = _590 ^ (_590 >> 6u);
    uint _594 = _592 + (_592 << 3u);
    uint _596 = _594 ^ (_594 >> 11u);
    uint3 _609 = uint3(_173 + int3(1, 1, 1));
    uint _612 = _609.z;
    uint _614 = _612 + (_612 << 10u);
    uint _616 = _614 ^ (_614 >> 6u);
    uint _618 = _616 + (_616 << 3u);
    uint _620 = _618 ^ (_618 >> 11u);
    uint _626 = (((_609.y * 1664525u) + (_620 + (_620 << 15u))) + 1013904223u) * 1664525u;
    uint _628 = _626 ^ (_626 >> 11u);
    uint _631 = _628 ^ ((_628 << 7u) & 2636928640u);
    uint _634 = _631 ^ ((_631 << 15u) & 4022730752u);
    uint _640 = (((_609.x * 1664525u) + (_634 ^ (_634 >> 18u))) + 1013904223u) * 1664525u;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    uint _650 = _648 ^ (_648 >> 18u);
    uint _652 = _650 + (_650 << 10u);
    uint _654 = _652 ^ (_652 >> 6u);
    uint _656 = _654 + (_654 << 3u);
    uint _658 = _656 ^ (_656 >> 11u);
    float3 _673 = (_174 * _174) * (3.0f.xxx - (_174 * 2.0f));
    float _674 = _673.x;
    float _675 = 1.0f - _674;
    float _677 = _673.y;
    float _678 = 1.0f - _677;
    float _680 = _673.z;
    float _681 = 1.0f - _680;
    float2 _698 = ((((((float2(asfloat((_216 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_224 + (_224 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _681) + (((float2(asfloat((_278 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_286 + (_286 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _681)) + (((float2(asfloat((_340 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_348 + (_348 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _681)) + (((float2(asfloat((_402 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_410 + (_410 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _681)) + (((float2(asfloat((_464 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_472 + (_472 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _678) * _680);
    float2 _713 = clamp(((((_698 + (((float2(asfloat((_526 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_534 + (_534 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _678) * _680)) + (((float2(asfloat((_588 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_596 + (_596 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _675) * _677) * _680)) + (((float2(asfloat((_650 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_658 + (_658 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _674) * _677) * _680)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _718 = _170 + (float3(_713.x, 0.0f, _713.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_718, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_165, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _718;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
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
// lifted from _nani_balcony_door.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static const uint2 _75[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    int bool_render_shadows : packoffset(c0.y);
};


static float4 gl_FragCoord;
static uint in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    bool _85;
    if (in_var_C6 != 0u)
    {
        _85 = bool_render_shadows != 0;
    }
    else
    {
        _85 = false;
    }
    if (_85)
    {
        uint2 _90 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _93 = frame_constants_frame_index & 3u;
        if (all(bool2(_90.x != _75[_93].x, _90.y != _75[_93].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
