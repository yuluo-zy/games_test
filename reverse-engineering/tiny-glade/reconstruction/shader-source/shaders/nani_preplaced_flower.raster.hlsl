// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_preplaced_flower.raster.hlsl | B nani raster | bins: _nani_preplaced_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani pre-placed flower: VS culls flowers inside building masks (NaN position),
// bends with a 2D wind lattice + grass_object_push; PS samples an albedo map
// unless vertex color overrides it, applies snow tint, packs gbuffer.

// Registerless terrain heightmap global for the inc/terrain_heightmap.inc.hlsl
// terrain_height() helper (INC fix #3); unused here, dxc DCEs it.
Texture2D terrain_heightmap : register(t63);

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
#include "inc/terrain_heightmap.inc.hlsl"
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

uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 grass_object_push_dims;
uint2 albedo_map_dims;
uint2 buildings_mask_non_blurred_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed transform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t18);
Texture2D<float4> albedo_map : register(t19);
Texture2D<float4> buildings_mask_non_blurred : register(t20);

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
    uint instance_id : SV_InstanceID;
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
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].transform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;

    float2 heightmap_uv = (origin_xz * 0.0076923077f) + 0.5f.xx;
    float2 wind_uv = (origin_xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    world.x -= vin.wind_bend * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.175f);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, heightmap_uv, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * vin.wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    float4 clip_pos;
    float3 out_color;
    float3 out_normal;
    float3 out_world;
    float2 out_uv;
    if (buildings_mask_non_blurred.SampleLevel(sampler_default, heightmap_uv, 0.0f).x > 0.5f)
    {
        // Inside a building: NaN position culls the vertex (never rasterizes).
        clip_pos = asfloat(0x7fbfffffu).xxxx;
        out_color = 0.0f.xxx;
        out_normal = 0.0f.xxx;
        out_world = 0.0f.xxx;
        out_uv = 0.0f.xx;
    }
    else
    {
        ViewConstants vc = view_constants[current_view_index];
        clip_pos = mul(vc.world_to_sample, float4(displaced, 1.0f));
        out_color = vin.Vertex_Color;
        out_normal = mul(xform, float4(vin.Vertex_Normal, 0.0f));
        out_world = displaced;
        out_uv = origin_xz * 0.1f;
    }
    o.position = clip_pos;
    o.C0 = out_color;
    o.C1 = out_normal;
    o.C2 = out_world;
    o.C3 = out_uv;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 albedo;
    if (length(pin.C0) == 0.0f)
    {
        albedo = albedo_map.SampleBias(sampler_llr, pin.C3, global_mip_bias).xyz;
    }
    else
    {
        albedo = pin.C0;
    }
    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
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
// lifted from _nani_preplaced_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct PreplacedFlowerInstanceData
{
    Affine3Packed transform;
};

static float2 _123;
static float3 _124;

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
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    uint2 _Globals_albedo_map_dims : packoffset(c2);
    uint2 _Globals_buildings_mask_non_blurred_dims : packoffset(c2.z);
    int _Globals_bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t20, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Color : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float in_var_wind_bend : TEXCOORD3;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _383;
    float3 _384;
    float3 _385;
    float3 _386;
    float2 _387;
    do
    {
        Affine3Packed _150;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _150.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
        }
        float3x4 _169 = transpose(float4x3(float3(_150.data[0], _150.data[1], _150.data[2]), float3(_150.data[3], _150.data[4], _150.data[5]), float3(_150.data[6], _150.data[7], _150.data[8]), float3(_150.data[9], _150.data[10], _150.data[11])));
        float3 _175 = mul(_169, float4(in_var_Vertex_Position, 1.0f));
        float2 _176 = mul(_169, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
        float2 _178 = (_176 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _178, 0.0f).x > 0.5f)
        {
            _383 = asfloat(0x7fbfffffu /* nan */).xxxx;
            _384 = _124;
            _385 = _124;
            _386 = _124;
            _387 = _123;
            break;
        }
        float2 _187 = _176 * 0.100000001490116119384765625f;
        float2 _196 = _187 + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
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
        _175.x = _175.x - ((in_var_wind_bend * (sin((((((1.0f - _334) * (((asfloat(((_224 ^ (_224 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) + ((asfloat(((_256 ^ (_256 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331))) + (((asfloat(((_288 ^ (_288 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) * _334)) + (((asfloat(((_320 ^ (_320 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331) * _334)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _356 = grass_object_push.SampleLevel(sampler_default, _178, 0.0f);
        float _358 = clamp(_356.z, 0.0f, 1.0f);
        float3 _368;
        if (_358 > 0.0f)
        {
            float2 _366 = _175.xz + ((_356.xy * _358) * in_var_wind_bend);
            _368 = float3(_366.x, _175.y, _366.y);
        }
        else
        {
            _368 = _175;
        }
        _383 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_368, 1.0f));
        _384 = in_var_Vertex_Color;
        _385 = mul(_169, float4(in_var_Vertex_Normal, 0.0f));
        _386 = _368;
        _387 = _187;
        break;
    } while(false);
    gl_Position = _383;
    out_var_C0 = _384;
    out_var_C1 = _385;
    out_var_C2 = _386;
    out_var_C3 = _387;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_preplaced_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct PreplacedFlowerInstanceData
{
    Affine3Packed transform;
};

static float2 _123;
static float3 _124;

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
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    uint2 _Globals_albedo_map_dims : packoffset(c2);
    uint2 _Globals_buildings_mask_non_blurred_dims : packoffset(c2.z);
    int _Globals_bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t20, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Color : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float in_var_wind_bend : TEXCOORD3;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _383;
    float3 _384;
    float3 _385;
    float3 _386;
    float2 _387;
    do
    {
        Affine3Packed _150;
        [unroll]
        for (int _1ident = 0; _1ident < 12; _1ident++)
        {
            _150.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
        }
        float3x4 _169 = transpose(float4x3(float3(_150.data[0], _150.data[1], _150.data[2]), float3(_150.data[3], _150.data[4], _150.data[5]), float3(_150.data[6], _150.data[7], _150.data[8]), float3(_150.data[9], _150.data[10], _150.data[11])));
        float3 _175 = mul(_169, float4(in_var_Vertex_Position, 1.0f));
        float2 _176 = mul(_169, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
        float2 _178 = (_176 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, _178, 0.0f).x > 0.5f)
        {
            _383 = asfloat(0x7fbfffffu /* nan */).xxxx;
            _384 = _124;
            _385 = _124;
            _386 = _124;
            _387 = _123;
            break;
        }
        float2 _187 = _176 * 0.100000001490116119384765625f;
        float2 _196 = _187 + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
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
        _175.x = _175.x - ((in_var_wind_bend * (sin((((((1.0f - _334) * (((asfloat(((_224 ^ (_224 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) + ((asfloat(((_256 ^ (_256 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331))) + (((asfloat(((_288 ^ (_288 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _332) * _334)) + (((asfloat(((_320 ^ (_320 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _331) * _334)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _356 = grass_object_push.SampleLevel(sampler_default, _178, 0.0f);
        float _358 = clamp(_356.z, 0.0f, 1.0f);
        float3 _368;
        if (_358 > 0.0f)
        {
            float2 _366 = _175.xz + ((_356.xy * _358) * in_var_wind_bend);
            _368 = float3(_366.x, _175.y, _366.y);
        }
        else
        {
            _368 = _175;
        }
        _383 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_368, 1.0f));
        _384 = in_var_Vertex_Color;
        _385 = mul(_169, float4(in_var_Vertex_Normal, 0.0f));
        _386 = _368;
        _387 = _187;
        break;
    } while(false);
    gl_Position = _383;
    out_var_C0 = _384;
    out_var_C1 = _385;
    out_var_C2 = _386;
    out_var_C3 = _387;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_preplaced_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
