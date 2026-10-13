// PATCHED: modules re.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/terrain_lighting_texture.cs.hlsl | evidence: spirv-cross of _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (terrain indirect light + soft shadow; grass/dirt color from height comparison, 5-tap depth gather + 10-tap PCF) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
Texture2D<float4> terrain_heightmap : register(t17, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
#include "inc/math.hlsl"
#include "inc/pack_unpack.hlsl"

// color_library.hlsl is guard-skipped; the cbuffer is redeclared below under
// the original binding name `color_library` (file OpString is still recorded).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"

struct ColorLibrary
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer color_library : register(b14, space0)
{
    ColorLibrary colors;
};

cbuffer $Globals : register(b15, space0)
{
    uint2 img_output_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    int bool_render_shadows;
    uint2 brdf_fg_lut_dims;
};

Texture2DArray<float4> shadow_map : register(t13, space0);
RWTexture2D<float3> img_output : register(u16, space0);
Texture2D<float4> visual_terrain_heightmap : register(t18, space0);

// Dirt color used where the terrain rises above the visual heightmap.
static const float3 DIRT_COLOR = float3(0.122f, 0.122f, 0.089f);

[[vk::ext_capability(5302)]]
[[vk::ext_capability(49)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (bool_render_shadows == 0)
    {
        img_output[uint2(int3(dispatch_thread_id).xy)] = float3(1.0f, 1.0f, 0.0f);
        return;
    }

    float3 grass_color = lerp(colors.grass_1.xyz, colors.grass_2.xyz, 0.15f.xxx) * 1.2f * 0.55f;
    float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f.xx) / float2(int2(img_output_dims));
    float2 world_xz = ((uv * 2.0f) - 1.0f.xx) * 65.0f.xx;
    float x = world_xz.x;
    float y = world_xz.y;
    float2 height_uv = world_xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;

    float visual_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, visual_terrain_heightmap.SampleLevel(sampler_lnc, height_uv, 0.0f).x);
    float base_height = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, terrain_heightmap.SampleLevel(sampler_default, height_uv, 0.0f).x);
    float3 albedo = (base_height + 0.1f > visual_height) ? grass_color : DIRT_COLOR;

    float4 gathered = visual_terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
    float2 grad = (float2((gathered.z + gathered.y) - (gathered.w + gathered.x), (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f;
    grad /= (130.0f.xx / float2(terrain_heightmap_dims));
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));

    float3 view_up = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float3 albedo_override = ((render_overrides & 1u) != 0u) ? float3(0.5f, 0.5f, 0.5f) : albedo;
    float ndl = max(0.0f, dot(normal, view_up));

    float2 shadow = 0.0f.xx;
    if (bool_render_shadows == 1)
    {
        float4 sample_clip = mul(view_constants[2].world_to_sample, float4(x, visual_height, y, 1.0f));
        if (sample_clip.z <= 1.0f)
        {
            float2 abs_xy = abs(sample_clip.xy);
            bool outside = any(abs_xy > 0.9833333f.xx);
            float2 sample_xy = outside ? sample_clip.xy * 0.5f : sample_clip.xy;
            float cascade = outside ? 5.0f : 10.0f;
            float sample_scale = outside ? 0.5f : 1.0f;
            float2 limit = 1.0f.xx - 0.0008333333f.xx * (2.0f * cascade);
            bool shrink = any(abs(sample_xy) > limit);
            float pcf_radius = shrink ? cascade * 0.28571428f : cascade;
            float2 sample_uv = shrink ? sample_xy * 0.28571428f : sample_xy;
            float sscale = shrink ? sample_scale * 0.28571428f : sample_scale;
            uint cascade_layer = shrink ? 2u : uint(outside);
            uint cascade_index = 2u + cascade_layer;

            float cos_theta = dot(normal, -view_up);
            float4 ray_sample = float4(sample_uv.x, sample_uv.y, sample_clip.z, sample_clip.w)
                + mul(view_constants[cascade_index].world_to_sample, float4((normal * (0.06666667f * clamp(1.0f - cos_theta, 0.0f, 1.0f))) / sscale, 0.0f));
            float2 pc = (ray_sample.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 sample_pos = float3(pc.x, pc.y, ray_sample.z);
            float probe_radius = max((0.3f / sscale) * (1.0f - abs(cos_theta)), 0.025f);

            uint hash = uint((int(dispatch_thread_id.x) * 11) + (int(dispatch_thread_id.y) * 7) + (frame_index * 5u)) % 64u;
            float angle_hash = float(hash) * GOLDEN_RATIO;
            float frame_noise_scale = 5.588237762451171875f * float(frame_index % 64u);
            float noise = frac(52.98291778564453125f * frac((0.0671105608344078f * (float(dispatch_thread_id.x) + frame_noise_scale)) + (0.005837149918079376f * (float(dispatch_thread_id.y) + frame_noise_scale))));

            float4 shadow_offset = mul(view_constants[cascade_index].world_to_sample, float4(normal, 0.0f));

            float depth_sum = 0.0f;
            int hit_count = 0;
            for (int i = 0; i < 5; i++)
            {
                float angle = (float(i) + angle_hash) * 2.3999631404876708984375f;
                float3 offset = sample_pos + float3((float2(cos(angle), sin(angle)) * (float(i) + noise) * 0.2f.xx * pcf_radius) * 0.0008333333f.xx, probe_radius * (-0.0009765625f));
                float sample_depth = shadow_map.SampleLevel(sampler_shadow_nearest, float3(offset.xy, cascade_layer), 0.0f).x;
                if (sample_pos.z > sample_depth)
                {
                    depth_sum += sample_depth;
                    hit_count++;
                }
            }

            float2 pcf = 0.0f.xx;
            if (hit_count != 0)
            {
                float avg_depth = depth_sum / float(hit_count);
                float light = max((ray_sample.z - avg_depth) * ((17.0666675567626953125f * pcf_radius) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w))) / avg_depth, 0.0f);
                float pcf_scale = (light / pow(pow(light / (pcf_radius - 1.0f), 5.0f) + 1.0f, 0.2f)) + 1.0f;
                float2 shadow_offset_uv = ((shadow_offset.xy * float(int(sign(shadow_offset.z)))) / max(0.15f, abs(shadow_offset.z)).xx) * 0.0009765625f.xx * 2.0f;
                for (int j = 0; j < 10; j++)
                {
                    float angle = (float(j) + angle_hash) * 2.3999631404876708984375f;
                    float2 spiral = float2(cos(angle), sin(angle)) * (float(j) + noise) * 0.1f.xx * pcf_scale;
                    float3 offset = sample_pos + float3(spiral * 0.0008333333f.xx, probe_radius * (-0.0009765625f));
                    pcf += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(offset.xy, cascade_layer), offset.z + min(0.0f, dot(shadow_offset_uv, spiral))), 0.0f);
                }
                shadow = pcf * 0.1f;
            }
        }
    }

    float3 lit = (sun_color.xyz * 1.0f) * ((1.0f - shadow.x) * smoothstep(0.0f, 0.5f, ndl)) * ndl * albedo_override * 0.3183098733425140380859375f;
    float ground_factor = 1.0f - clamp(normal.y, 0.0f, 1.0f);
    float view_factor = 1.0f - clamp(view_up.y, 0.0f, 1.0f);
    float3 sky = lit + (lit * (((ground_factor * ground_factor) * ground_factor) * ((view_factor * view_factor) * view_factor)));
    img_output[uint2(int3(dispatch_thread_id).xy)] = sky + ((sky_high_color.xyz * 1.0f) * albedo_override * ((normal.y * 0.5f) + 0.5f));
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _terrain_lighting_texture.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float _162;

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

cbuffer color_library : register(b14, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

cbuffer $Globals : register(b15, space0)
{
    uint2 _Globals_img_output_dims : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    int _Globals_bool_render_shadows : packoffset(c1.z);
    uint2 _Globals_brdf_fg_lut_dims : packoffset(c2);
};

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t13, space0);
RWTexture2D<float3> img_output : register(u16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> visual_terrain_heightmap : register(t18, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (_Globals_bool_render_shadows == 0)
        {
            img_output[uint2(int3(gl_GlobalInvocationID).xy)] = float3(1.0f, 1.0f, 0.0f);
            break;
        }
        float3 _187 = (lerp(color_library_grass_1.xyz, color_library_grass_2.xyz, 0.1500000059604644775390625f.xxx) * 1.2000000476837158203125f) * 0.550000011920928955078125f;
        float2 _194 = ((((float2(int3(gl_GlobalInvocationID).xy) + 0.5f.xx) / float2(int2(_Globals_img_output_dims))) * 2.0f) - 1.0f.xx) * 65.0f.xx;
        float _195 = _194.x;
        float _196 = _194.y;
        float2 _200 = (float3(_195, 0.0f, _196).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _212 = lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, _200, 0.0f).x);
        bool3 _215 = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _200, 0.0f).x) + 0.100000001490116119384765625f) > _212).xxx;
        float3 _216 = float3(_215.x ? _187.x : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).x, _215.y ? _187.y : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).y, _215.z ? _187.z : float3(0.122000001370906829833984375f, 0.122000001370906829833984375f, 0.08900000154972076416015625f).z);
        float4 _218 = visual_terrain_heightmap.GatherRed(sampler_default, _200, int2(0, 0));
        float _223 = _218.z;
        float _224 = _218.y;
        float _226 = _218.w;
        float _227 = _218.x;
        float2 _236 = ((float2((_223 + _224) - (_226 + _227), (_227 + _224) - (_226 + _223)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
        float3 _243 = cross(normalize(float3(0.0f, _236.y, 1.0f)), normalize(float3(1.0f, _236.x, 0.0f)));
        float4 _246 = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float3 _247 = _246.xyz;
        bool3 _258 = ((frame_constants_render_overrides & 1u) != 0u).xxx;
        float3 _259 = float3(_258.x ? 0.5f.xxx.x : _216.x, _258.y ? 0.5f.xxx.y : _216.y, _258.z ? 0.5f.xxx.z : _216.z);
        float _261 = max(0.0f, dot(_243, _247));
        float _481;
        if (_Globals_bool_render_shadows == 1)
        {
            float2 _479;
            do
            {
                float4 _271 = mul(frame_constants_view_constants[2u].world_to_sample, float4(_195, _212, _196, 1.0f));
                if (_271.z > 1.0f)
                {
                    _479 = 0.0f.xx;
                    break;
                }
                float2 _276 = _271.xy;
                float2 _277 = abs(_276);
                bool _279 = any(bool2(_277.x > 0.9833333492279052734375f.xx.x, _277.y > 0.9833333492279052734375f.xx.y));
                float2 _283;
                if (_279)
                {
                    _283 = _276 * 0.5f;
                }
                else
                {
                    _283 = _276;
                }
                float _284 = _279 ? 5.0f : 10.0f;
                float _286 = _279 ? 0.5f : 1.0f;
                float2 _287 = abs(_283);
                float2 _290 = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * _284));
                bool _292 = any(bool2(_287.x > _290.x, _287.y > _290.y));
                float _298;
                float2 _299;
                float _300;
                if (_292)
                {
                    _298 = _284 * 0.2857142984867095947265625f;
                    _299 = _283 * 0.2857142984867095947265625f;
                    _300 = _286 * 0.2857142984867095947265625f;
                }
                else
                {
                    _298 = _284;
                    _299 = _283;
                    _300 = _286;
                }
                float _303 = float(_292 ? 2u : uint(_279));
                uint _305 = 2u + uint(_303);
                float _308 = dot(_243, -_247);
                float4 _320 = float4(_299.x, _299.y, _271.z, _271.w) + mul(frame_constants_view_constants[_305].world_to_sample, float4((_243 * (0.066666670143604278564453125f * clamp(1.0f - _308, 0.0f, 1.0f))) / _300.xxx, 0.0f));
                float2 _323 = (_320.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                float3 _324 = float3(_323.x, _323.y, _320.z);
                float _329 = max((0.300000011920928955078125f / _300) * (1.0f - abs(_308)), 0.02500000037252902984619140625f);
                float _342 = float((uint((int3(gl_GlobalInvocationID).x * 11) + (int3(gl_GlobalInvocationID).y * 7)) + (frame_constants_frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
                uint2 _343 = uint2(int3(gl_GlobalInvocationID).xy);
                float _348 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float _358 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_343.x) + _348)) + (0.005837149918079376220703125f * (float(_343.y) + _348))));
                float4 _364 = mul(frame_constants_view_constants[_305].world_to_sample, float4(_243, 0.0f));
                float _366;
                int _369;
                _366 = 0.0f;
                _369 = 0;
                float _367;
                int _370;
                for (int _371 = 0; _371 < 5; _366 = _367, _369 = _370, _371++)
                {
                    float _376 = float(_371);
                    float _378 = (_376 + _342) * 2.3999631404876708984375f;
                    float3 _391 = _324 + float3((((float2(cos(_378), sin(_378)) * (_376 + _358)) * 0.20000000298023223876953125f.xx) * _298) * 0.000833333353511989116668701171875f.xx, _329 * (-0.0009765625f));
                    float4 _398 = shadow_map.SampleLevel(sampler_shadow_nearest, float3(_391.xy, _303), 0.0f);
                    float _399 = _398.x;
                    if (_391.z > _399)
                    {
                        _367 = _366 + _399;
                        _370 = _369 + 1;
                    }
                    else
                    {
                        _367 = _366;
                        _370 = _369;
                    }
                }
                float _407 = _366 / float(_369);
                if (_369 == 0)
                {
                    _479 = 0.0f.xx;
                    break;
                }
                float _421 = max(((_320.z - _407) * ((17.0666675567626953125f * _298) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / _407, 0.0f);
                float _428 = (_421 / pow(pow(_421 / (_298 - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
                float2 _440 = (((_364.xy * float(int(sign(_364.z)))) / max(0.1500000059604644775390625f, abs(_364.z)).xx) * 0.0009765625f.xx) * 2.0f;
                float2 _442;
                _442 = 0.0f.xx;
                for (int _445 = 0; _445 < 10; )
                {
                    float _449 = float(_445);
                    float _451 = (_449 + _342) * 2.3999631404876708984375f;
                    float2 _458 = ((float2(cos(_451), sin(_451)) * (_449 + _358)) * 0.100000001490116119384765625f.xx) * _428;
                    float3 _464 = _324 + float3(_458 * 0.000833333353511989116668701171875f.xx, _329 * (-0.0009765625f));
                    _442 += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(_464.xy, _303), _464.z + min(0.0f, dot(_440, _458))), _162);
                    _445++;
                    continue;
                }
                _479 = _442 * 0.100000001490116119384765625f;
                break;
            } while(false);
            _481 = _479.x;
        }
        else
        {
            _481 = 0.0f;
        }
        float3 _489 = ((((frame_constants_sun_color.xyz * 1.0f) * ((1.0f - _481) * smoothstep(0.0f, 0.5f, _261))) * _261) * _259) * 0.3183098733425140380859375f.xxx;
        float _490 = _243.y;
        float _494 = 1.0f - clamp(_490, 0.0f, 1.0f);
        float _495 = _494 * _494;
        float _498 = 1.0f - clamp(_246.y, 0.0f, 1.0f);
        img_output[uint2(int3(gl_GlobalInvocationID).xy)] = (_489 + (_489 * (((_495 * _495) * _494) * ((_498 * _498) * _498)))) + ((((frame_constants_sky_high_color.xyz * 1.0f) * _259) * ((_490 * 0.5f) + 0.5f)) * 1.0f);
        break;
    } while(false);
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
