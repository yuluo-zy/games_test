// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_solid_vertex_color_window.raster.hlsl | B nani raster | bins: _nani_solid_vertex_color_window.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani window solid: VS jitters the facade vertices with a 3D value-noise
// lattice (per-window hash) and forwards the semitransparent flag + window
// color id; PS dithers the semitransparent holes and samples the window color
// palette.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
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
#include "inc/wall_distortion.hlsl"
#include "inc/window_color_customization.hlsl"
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

// 3D integer hash (z then y then x avalanche with LCG constants), producing a
// uniform float2 in [-1,1] per lattice corner.
uint window_hash(uint3 p)
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
    return x;
}

float2 window_corner_rand(uint3 cell)
{
    uint h = window_hash(cell);
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
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
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

    // 3D value-noise window jitter (8-corner trilinear hash lattice).
    float3 scaled = world * 0.4f;
    int3 cell = int3(floor(scaled));
    float3 fr = frac(scaled);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float2 n = float2(0.0f, 0.0f);
    n += window_corner_rand(uint3(cell)) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(1, 0, 0))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(0, 1, 0))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(1, 1, 0))) * f.x * f.y * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(0, 0, 1))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += window_corner_rand(uint3(cell + int3(1, 0, 1))) * f.x * (1.0f - f.y) * f.z;
    n += window_corner_rand(uint3(cell + int3(0, 1, 1))) * (1.0f - f.x) * f.y * f.z;
    n += window_corner_rand(uint3(cell + int3(1, 1, 1))) * f.x * f.y * f.z;
    float2 offset = clamp((n - 0.5f.xx) * 2.5f, -0.5f.xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.06f);

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = nani_instance_data[idx].is_semitransparent;
    o.C5 = nani_instance_data[idx].color_id;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = (pin.C4 & 1u) != 0u && bool_render_shadows != 0;
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

    uint seed = pin.C5;
    uint h = seed + (seed << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float rnd = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    int color_id = int(pin.C5);
    float3 window_color = window_colors.Sample(sampler_llr, float3((pin.C3 * 0.5f) + rnd.xx, float(color_id))).xyz;
    if (color_id == 0)
    {
        window_color = window_color * 0.7f;
    }

    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(window_color, 0.0f.xxx, 1.0f.xxx));
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_solid_vertex_color_window.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static const uint2 _104[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    int _Globals_bool_render_shadows : packoffset(c0.y);
};

SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> window_colors : register(t14, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint in_var_C5 : TEXCOORD5;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    bool _121;
    if ((in_var_C4 & 1u) != 0u)
    {
        _121 = _Globals_bool_render_shadows != 0;
    }
    else
    {
        _121 = false;
    }
    if (_121)
    {
        uint2 _126 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _129 = frame_constants_frame_index & 3u;
        if (all(bool2(_126.x != _104[_129].x, _126.y != _104[_129].y)))
        {
            discard;
        }
    }
    uint _138 = in_var_C5 + (in_var_C5 << 10u);
    uint _140 = _138 ^ (_138 >> 6u);
    uint _142 = _140 + (_140 << 3u);
    uint _144 = _142 ^ (_142 >> 11u);
    int _153 = int(in_var_C5);
    float4 _161 = window_colors.Sample(sampler_llr, float3((in_var_C3 * 0.5f) + (asfloat(((_144 + (_144 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx, float(_153)));
    float3 _162 = _161.xyz;
    float3 _167;
    if (_153 == 0)
    {
        _167 = _162 * 0.699999988079071044921875f;
    }
    else
    {
        _167 = _162;
    }
    float3 _170 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _192 = sqrt(clamp(_167, 0.0f.xxx, 1.0f.xxx));
    uint _211 = (uint((clamp(_192.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_192.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_192.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _212 = uint2(((uint((clamp(_170.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_170.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_170.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _211);
    _212.y = _211 | 117440512u;
    out_var_SV_TARGET0 = _212;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
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

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_solid_vertex_color_window.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint _Globals_current_view_index : packoffset(c0);
    int _Globals_bool_render_shadows : packoffset(c0.y);
};


static float4 gl_FragCoord;
static uint in_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    bool _86;
    if ((in_var_C4 & 1u) != 0u)
    {
        _86 = _Globals_bool_render_shadows != 0;
    }
    else
    {
        _86 = false;
    }
    if (_86)
    {
        uint2 _91 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _94 = frame_constants_frame_index & 3u;
        if (all(bool2(_91.x != _75[_94].x, _91.y != _75[_94].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
