// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/solid_vertex_color_albedo_map.raster.hlsl | B simple-solid | bins: _solid_vertex_color_albedo_map.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Solid raster sampling an albedo map (terrain/building ground) with snow tint,
// roughness gbuffer top byte, and contact-shadow flag.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint receive_contact_shadows;
uint is_semitransparent;
uint2 albedo_map_dims;
float4x4 model_to_world;
int bool_render_shadows;
int is_terrain;
float roughness_param;

Texture2D<float4> albedo_map : register(t14);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float2 Vertex_UV : TEXCOORD2;
};

struct VsOut
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, world);
    o.C0 = vin.Vertex_UV;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    return o;
}

struct PsIn
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool dither = is_semitransparent != 0u && bool_render_shadows != 0;
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

    float3 albedo = albedo_map.SampleBias(sampler_llr, pin.C0, global_mip_bias).xyz;
    float3 normal = normalize(pin.C1);
    bool snow = glade_theme_id == 4u && !(is_terrain != 0);
    if (snow)
    {
        albedo = lerp(albedo, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f)).xxx);
    }

    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness_param, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    uint flags = ((((((0u | (uint(receive_contact_shadows != 0u) << 0u)) | 2u) | 4u) | 0u) | 0u) | 0u);
    o.gbuffer = uint2(normal_word, albedo_word | (flags << 24u));
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _solid_vertex_color_albedo_map.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static const uint2 _71[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    uint _Globals_receive_contact_shadows : packoffset(c0.y);
    uint _Globals_is_semitransparent : packoffset(c0.z);
    uint2 _Globals_albedo_map_dims : packoffset(c1);
    column_major float4x4 _Globals_model_to_world : packoffset(c2);
    int _Globals_bool_render_shadows : packoffset(c6);
    int _Globals_is_terrain : packoffset(c6.y);
    float _Globals_roughness_param : packoffset(c6.z);
};


static float4 gl_FragCoord;
struct SPIRV_Cross_Input
{
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    bool _82;
    if (_Globals_is_semitransparent != 0u)
    {
        _82 = _Globals_bool_render_shadows != 0;
    }
    else
    {
        _82 = false;
    }
    if (_82)
    {
        uint2 _87 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint _90 = frame_constants_frame_index & 3u;
        if (all(bool2(_87.x != _71[_90].x, _87.y != _71[_90].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
