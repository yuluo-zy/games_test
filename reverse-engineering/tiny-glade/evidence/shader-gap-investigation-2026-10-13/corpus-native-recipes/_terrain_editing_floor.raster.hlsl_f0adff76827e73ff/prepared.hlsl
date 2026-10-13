#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/terrain_editing/floor.raster.hlsl"




#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

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
    uint2 _Globals_buildings_mask_dims : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c2);
    uint2 _Globals_paths_visual_dims : packoffset(c2.z);
    uint2 _Globals_water_mask_dims : packoffset(c3);
    uint2 _Globals_highlight_active_hill_dims : packoffset(c3.z);
    uint2 _Globals_mud_dims : packoffset(c4);
    uint2 _Globals_noise_debug_dims : packoffset(c4.z);
    uint2 _Globals_floor_heightmap_dims : packoffset(c5);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 _Globals_dirt_1_dims : packoffset(c6);
    uint2 _Globals_dirt_2_dims : packoffset(c6.z);
    uint2 _Globals_grass_dims : packoffset(c7);
    uint2 _Globals_dirt_mask_dims : packoffset(c7.z);
    uint2 _Globals_stone_floor_normal_dims : packoffset(c8);
    uint2 _Globals_stone_floor_seed_dims : packoffset(c8.z);
    uint2 _Globals_stone_floor_height_dims : packoffset(c9);
    uint2 _Globals_stone_floor_roughness_dims : packoffset(c9.z);
    uint2 _Globals_stone_floor_seed_2_dims : packoffset(c10);
    uint2 _Globals_stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 _Globals_stone_floor_height_2_dims : packoffset(c11);
    uint2 _Globals_stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 _Globals_stone_floor_seed_3_dims : packoffset(c12);
    uint2 _Globals_stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 _Globals_stone_floor_height_3_dims : packoffset(c13);
    uint2 _Globals_stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 _Globals_garden_mask_dims : packoffset(c14);
    uint _Globals_force_top_down_render : packoffset(c14.z);
};

SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 _147;
    float _148;
    bool _150;
    float2 _129 = ((((float2(int2(float2(float(uint(gl_VertexIndex) % 91u), float(uint(gl_VertexIndex) / 91u)))) + 0.5f.xx) * 0.010989011265337467193603515625f.xx) * 2.0f) - 1.0f.xx) * 65.72222137451171875f.xx;
    float3 _132 = float3(_129.x, 0.0f, _129.y);
    _132.y = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_132.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
    float _174;
    do
    {
        _147 = (_132.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        _148 = _147.x;
        _150 = !(_148 < 0.0f);
        bool _154;
        if (_150)
        {
            _154 = _148 > 1.0f;
        }
        else
        {
            _154 = true;
        }
        bool _160;
        if (!_154)
        {
            _160 = _147.y < 0.0f;
        }
        else
        {
            _160 = true;
        }
        bool _166;
        if (!_160)
        {
            _166 = _147.y > 1.0f;
        }
        else
        {
            _166 = true;
        }
        if (_166)
        {
            _174 = 0.0f;
            break;
        }
        else
        {
            _174 = rocky_terrain.SampleLevel(sampler_default, _147, 0.0f).x;
            break;
        }
        break; 
    } while(false);
    float _200;
    do
    {
        bool _180;
        if (_150)
        {
            _180 = _148 > 1.0f;
        }
        else
        {
            _180 = true;
        }
        bool _186;
        if (!_180)
        {
            _186 = _147.y < 0.0f;
        }
        else
        {
            _186 = true;
        }
        bool _192;
        if (!_186)
        {
            _192 = _147.y > 1.0f;
        }
        else
        {
            _192 = true;
        }
        if (_192)
        {
            _200 = 0.0f;
            break;
        }
        else
        {
            _200 = rocky_terrain.SampleLevel(sampler_default, _147, 0.0f).z;
            break;
        }
        break; 
    } while(false);
    float4 _202 = terrain_heightmap.GatherRed(sampler_default, _147, int2(0, 0));
    float2 _206 = 130.0f.xx / float2(_Globals_terrain_heightmap_dims);
    float _207 = _202.z;
    float _208 = _202.y;
    float _210 = _202.w;
    float _211 = _202.x;
    float2 _220 = ((float2((_207 + _208) - (_210 + _211), (_211 + _208) - (_210 + _207)) * 0.5f.xx) * 26.5f) / _206;
    float3 _227 = cross(normalize(float3(0.0f, _220.y, 1.0f)), normalize(float3(1.0f, _220.x, 0.0f)));
    float4 _232 = terrain_heightmap.GatherRed(sampler_default, _147, int2(0, 0));
    float _233 = _232.z;
    float _234 = _232.y;
    float _236 = _232.w;
    float _237 = _232.x;
    float _247 = length(((float2((_233 + _234) - (_236 + _237), (_237 + _234) - (_236 + _233)) * 0.5f.xx) * 26.5f) / _206);
    float3 _253;
    if (_247 < 2.0f)
    {
        _253 = _132 - (_227 * lerp(0.0f, 0.5f, clamp(_174 + _200, 0.0f, 1.0f)));
    }
    else
    {
        _253 = _132;
    }
    float _266 = lerp(_253.y, -0.800000011920928955078125f, smoothstep(0.20000000298023223876953125f, 0.60000002384185791015625f, water_mask.SampleLevel(sampler_llc, (_253.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, _247));
    float3 _267 = _253;
    _267.y = _266;
    float4 _270 = float4(_253.x, _266, _253.z, 1.0f);
    float4 _290;
    if (_Globals_force_top_down_render != 0u)
    {
        _290 = float4(_270.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), clamp((_266 - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        _290 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _270);
    }
    gl_Position = _290;
    out_var_C0 = _227;
    out_var_C1 = _267;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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

static uint _207;
static float3 _233;

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
    uint2 _Globals_buildings_mask_dims : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c2);
    uint2 _Globals_paths_visual_dims : packoffset(c2.z);
    uint2 _Globals_water_mask_dims : packoffset(c3);
    uint2 _Globals_highlight_active_hill_dims : packoffset(c3.z);
    uint2 _Globals_mud_dims : packoffset(c4);
    uint2 _Globals_noise_debug_dims : packoffset(c4.z);
    uint2 _Globals_floor_heightmap_dims : packoffset(c5);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 _Globals_dirt_1_dims : packoffset(c6);
    uint2 _Globals_dirt_2_dims : packoffset(c6.z);
    uint2 _Globals_grass_dims : packoffset(c7);
    uint2 _Globals_dirt_mask_dims : packoffset(c7.z);
    uint2 _Globals_stone_floor_normal_dims : packoffset(c8);
    uint2 _Globals_stone_floor_seed_dims : packoffset(c8.z);
    uint2 _Globals_stone_floor_height_dims : packoffset(c9);
    uint2 _Globals_stone_floor_roughness_dims : packoffset(c9.z);
    uint2 _Globals_stone_floor_seed_2_dims : packoffset(c10);
    uint2 _Globals_stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 _Globals_stone_floor_height_2_dims : packoffset(c11);
    uint2 _Globals_stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 _Globals_stone_floor_seed_3_dims : packoffset(c12);
    uint2 _Globals_stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 _Globals_stone_floor_height_3_dims : packoffset(c13);
    uint2 _Globals_stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 _Globals_garden_mask_dims : packoffset(c14);
    uint _Globals_force_top_down_render : packoffset(c14.z);
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

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> buildings_mask : register(t15, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> paths_visual : register(t19, space0);
Texture2D<float4> water_mask : register(t20, space0);
Texture2D<float4> mud : register(t22, space0);
Texture2D<float4> dirthpath_heightmap : register(t25, space0);
Texture2D<float4> dirt_1 : register(t26, space0);
Texture2D<float4> dirt_2 : register(t27, space0);
Texture2D<float4> grass : register(t28, space0);
Texture2D<float4> dirt_mask : register(t29, space0);
Texture2DArray<float4> brick_colors : register(t31, space0);
Texture2D<float4> stone_floor_normal : register(t33, space0);
Texture2D<float4> stone_floor_seed : register(t34, space0);
Texture2D<float4> stone_floor_height : register(t35, space0);
Texture2D<float4> stone_floor_roughness : register(t36, space0);
Texture2D<float4> stone_floor_seed_2 : register(t37, space0);
Texture2D<float4> stone_floor_normal_2 : register(t38, space0);
Texture2D<float4> stone_floor_height_2 : register(t39, space0);
Texture2D<float4> stone_floor_roughness_2 : register(t40, space0);
Texture2D<float4> stone_floor_seed_3 : register(t41, space0);
Texture2D<float4> stone_floor_normal_3 : register(t42, space0);
Texture2D<float4> stone_floor_height_3 : register(t43, space0);
Texture2D<float4> stone_floor_roughness_3 : register(t44, space0);
Texture2D<float4> garden_mask : register(t45, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static float3 in_var_C0;
static float3 in_var_C1;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    float2 _243 = (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    float4 _245 = paths_visual.SampleLevel(sampler_lnc, _243, 0.0f);
    float3 _247 = _245.yzw;
    bool _256 = frame_constants_glade_theme_id == 4u;
    float2 _259 = (in_var_C1.xz * (_256 ? 0.199999988079071044921875f : 0.04999999701976776123046875f)) + 7.0f.xx;
    float2 _263 = floor(_259 + dot(_259, 0.3660254180431365966796875f.xx).xx);
    float2 _267 = (_259 - _263) + dot(_263, 0.211324870586395263671875f.xx).xx;
    float _268 = _267.x;
    float _269 = _267.y;
    bool2 _271 = (_268 > _269).xx;
    float2 _272 = float2(_271.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _271.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 _274 = _267.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 _276 = _274.xy - _272;
    float4 _277 = float4(_276.x, _276.y, _274.z, _274.w);
    float2 _278 = fmod(_263, 289.0f.xx);
    float3 _283 = _278.y.xxx + float3(0.0f, _272.y, 1.0f);
    float3 _293 = (fmod(((_283 * 34.0f) + 1.0f.xxx) * _283, 289.0f.xxx) + _278.x.xxx) + float3(0.0f, _272.x, 1.0f);
    float2 _299 = _276.xy;
    float2 _301 = _274.zw;
    float3 _305 = max(0.5f.xxx - float3(dot(_267, _267), dot(_299, _299), dot(_301, _301)), 0.0f.xxx);
    float3 _306 = _305 * _305;
    float3 _310 = frac(fmod(((_293 * 34.0f) + 1.0f.xxx) * _293, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 _311 = _310 - 1.0f.xxx;
    float3 _313 = abs(_311) - 0.5f.xxx;
    float3 _316 = _311 - floor(_310 + (-0.5f).xxx);
    float3 _328;
    _328.x = (_316.x * _268) + (_313.x * _269);
    float2 _335 = (_316.yz * _277.xz) + (_313.yz * _277.yw);
    float _338 = 130.0f * dot((_306 * _306) * (1.792842864990234375f.xxx - (((_316 * _316) + (_313 * _313)) * 0.8537347316741943359375f)), float3(_328.x, _335.x, _335.y));
    float3 _342 = lerp(color_library_grass_1.xyz, color_library_grass_2.xyz, clamp(_338 - 0.4000000059604644775390625f, 0.0f, 1.0f).xxx);
    float3 _349;
    if (_256)
    {
        _349 = lerp(0.699999988079071044921875f.xxx, _342, clamp(_338 - 0.20000000298023223876953125f, 0.0f, 1.0f).xxx);
    }
    else
    {
        _349 = _342;
    }
    float3 _453;
    float3 _350 = _349 * 0.75f;
    float2 _351 = in_var_C1.xz * 0.20000000298023223876953125f;
    float _355_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, _351);
    float2 _355 = _355_tmp.xx;
    float _359 = _355.x + frame_constants_global_mip_bias;
    uint2 _361 = uint2(gl_FragCoord.yx);
    float _368 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float _378 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(_361.x) + _368)) + (0.005837149918079376220703125f * (float(_361.y) + _368))));
    float _380 = 1.0f - _245.x;
    float _384 = clamp(1.0f - dot(1.0f.xxx, (_247 * 2.0f) * _380), 0.0f, 1.0f);
    float3 _385 = _247 * float3(1.0f, 2.0f, 1.0f);
    float3 _393 = _385 / max(0.00999999977648258209228515625f, max(_385.x, max(_385.y, _385.z))).xxx;
    float3 _394 = ddx(in_var_C1);
    float3 _395 = ddy(in_var_C1);
    float2 _397 = ddx(_351);
    float2 _398 = ddy(_351);
    float2 _399 = -_398;
    float3 _400 = cross(in_var_C0, -_395);
    float3 _401 = cross(_394, in_var_C0);
    float _402 = _397.x;
    float _403 = _399.y;
    float _405 = _397.y;
    float _406 = _399.x;
    float _409 = 1.0f / ((_402 * _403) - (_405 * _406));
    float3x3 _419 = transpose(float3x3(((_400 * _402) + (_401 * _406)) * _409, ((_400 * _405) + (_401 * _403)) * _409, in_var_C0));
    float4 _424 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _430 = mul(_419, float3(1.0f, 0.0f, 0.0f));
    float3 _431 = mul(_419, float3(0.0f, 1.0f, 0.0f));
    float3 _432 = mul(_419, float3(0.0f, 0.0f, 1.0f));
    float3 _435 = cross(_430, _431);
    float3 _438 = (1.0f / dot(_432, _435)).xxx;
    float3x3 _442 = float3x3(cross(_431, _432) * _438, cross(_432, _430) * _438, _435 * _438);
    float3 _443 = mul(_442, in_var_C1 - (_424.xyz / _424.w.xxx));
    float3 _449 = float3(_351, 0.0f);
    float _624;
    do
    {
        _453 = (_449 + (_443 * ((-0.100000001490116119384765625f) / _443.z))) - _449;
        float _455 = 1.0f / max(1.0f, 2.0f);
        float _458 = lerp(0.0f, 1.0f, pow(_378 * _455, 2.0f));
        float3 _460 = _449 + (_453 * _458);
        float2 _461 = _460.xy;
        float3 _475 = (float3(stone_floor_height.SampleLevel(sampler_llr, _461, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _461, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _461, _359).x) - 1.0f.xxx) + _393;
        float _485 = _460.z - ((max(max(_384, _475.x), max(_475.y, _475.z)) - 1.0f) * 0.100000001490116119384765625f);
        bool _487 = (_485 < 0.0f) ? true : false;
        float _539;
        float _540;
        float _541;
        bool _542;
        float _543;
        if (_487)
        {
            _539 = _485;
            _540 = 0.0f;
            _541 = _458;
            _542 = _487;
            _543 = 0.0f;
        }
        else
        {
            float _495;
            float _499;
            _495 = _485;
            _499 = _458;
            float _496;
            bool _498;
            float _500;
            float _536;
            float _537;
            bool _538;
            uint _492 = 1u;
            bool _497 = _487;
            for (;;)
            {
                if (_492 < 3u)
                {
                    _500 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(_492) + _378) * _455, 2.0f)));
                    float3 _510 = _449 + (_453 * _500);
                    float2 _511 = _510.xy;
                    float3 _523 = (float3(stone_floor_height.SampleLevel(sampler_llr, _511, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _511, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _511, _359).x) - 1.0f.xxx) + _393;
                    _496 = _510.z - ((max(max(_384, _523.x), max(_523.y, _523.z)) - 1.0f) * 0.100000001490116119384765625f);
                    _498 = (_496 < 0.0f) ? true : false;
                    if (_498)
                    {
                        _536 = _496;
                        _537 = _500;
                        _538 = _498;
                        break;
                    }
                    _492++;
                    _495 = _496;
                    _497 = _498;
                    _499 = _500;
                    continue;
                }
                else
                {
                    _536 = 0.0f;
                    _537 = 1.0f;
                    _538 = _497;
                    break;
                }
            }
            _539 = _536;
            _540 = _495;
            _541 = _537;
            _542 = _538;
            _543 = _499;
        }
        if (_542)
        {
            float _548;
            float _551;
            float _553;
            float _555;
            _548 = _543;
            _551 = _539;
            _553 = _540;
            _555 = _541;
            for (uint _557 = 0u; _557 < 4u; )
            {
                float _561 = _548 + _555;
                float _562 = _561 * 0.5f;
                float3 _564 = _449 + (_453 * _562);
                float2 _565 = _564.xy;
                float3 _577 = (float3(stone_floor_height.SampleLevel(sampler_llr, _565, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _565, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _565, _359).x) - 1.0f.xxx) + _393;
                float _587 = _564.z - ((max(max(_384, _577.x), max(_577.y, _577.z)) - 1.0f) * 0.100000001490116119384765625f);
                bool _589 = (_587 < 0.0f) ? true : false;
                _548 = _589 ? _548 : _562;
                _551 = _589 ? _587 : _551;
                _553 = _589 ? _553 : _587;
                _555 = _589 ? _562 : _555;
                _557++;
                continue;
            }
            float _593 = lerp(_548, _555, _553 / (_553 + (-_551)));
            float3 _595 = _449 + (_453 * _593);
            float2 _596 = _595.xy;
            float3 _608 = (float3(stone_floor_height.SampleLevel(sampler_llr, _596, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _596, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _596, _359).x) - 1.0f.xxx) + _393;
            _624 = ((abs(_595.z - ((max(max(_384, _608.x), max(_608.y, _608.z)) - 1.0f) * 0.100000001490116119384765625f)) < (_553 * 0.89999997615814208984375f)) ? true : false) ? _593 : _555;
            break;
        }
        else
        {
            _624 = _543;
            break;
        }
        break; 
    } while(false);
    float3 _625 = _453 * _624;
    float2 _627 = _351 + _625.xy;
    float3 _629 = in_var_C1 + mul(_419, _625);
    float4 _639 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_clip, mul(frame_constants_view_constants[_Globals_current_view_index].world_to_view, float4(_629, 1.0f)));
    float3 _701;
    do
    {
        float3 _657 = (float3(stone_floor_height.SampleLevel(sampler_llr, _627, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _627, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _627, _359).x) - 1.0f.xxx) + _393;
        float _658 = _657.x;
        float4 _661 = float4(_384, _658, _657.yz);
        float4 _665 = max(max(_384, _658), max(_657.y, _657.z)).xxxx;
        float4 _667 = select(bool4(_661.x == _665.x, _661.y == _665.y, _661.z == _665.z, _661.w == _665.w), float4(1.0f, 1.0f, 1.0f, 1.0f), float4(0.0f, 0.0f, 0.0f, 0.0f));
        float3 _695;
        if (_667.y > 0.0f)
        {
            _695 = stone_floor_normal.SampleLevel(sampler_llr, _627, _359).xyz;
        }
        else
        {
            float3 _690;
            if (_667.z > 0.0f)
            {
                _690 = stone_floor_normal_2.SampleLevel(sampler_llr, _627, _359).xyz;
            }
            else
            {
                if (!(_667.w > 0.0f))
                {
                    _701 = float3(0.0f, 0.0f, 1.0f);
                    break;
                }
                _690 = stone_floor_normal_3.SampleLevel(sampler_llr, _627, _359).xyz;
            }
            _695 = _690;
        }
        float3 _697 = (_695 * 2.0f) - 1.0f.xxx;
        _697.z = _697.z * 0.0999999940395355224609375f;
        _701 = _697;
        break;
    } while(false);
    float3 _703 = normalize(mul(_701, _442));
    float2 _715 = _629.xz;
    float2 _717 = (_715 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    float3 _720 = paths_visual.SampleLevel(sampler_lnc, _717, 0.0f).yzw;
    float _725 = clamp(1.0f - dot(1.0f.xxx, (_720 * 2.0f) * _380), 0.0f, 1.0f);
    float3 _726 = _720 * float3(1.0f, 2.0f, 1.0f);
    float3 _736 = (float3(stone_floor_height.SampleLevel(sampler_llr, _627, _359).x, stone_floor_height_2.SampleLevel(sampler_llr, _627, _359).x, stone_floor_height_3.SampleLevel(sampler_llr, _627, _359).x) - 1.0f.xxx) + (_726 / max(0.00999999977648258209228515625f, max(_726.x, max(_726.y, _726.z))).xxx);
    float _737 = _736.x;
    float4 _740 = float4(_725, _737, _736.yz);
    float4 _744 = max(max(_725, _737), max(_736.y, _736.z)).xxxx;
    float4 _746 = select(bool4(_740.x == _744.x, _740.y == _744.y, _740.z == _744.z, _740.w == _744.w), float4(1.0f, 1.0f, 1.0f, 1.0f), float4(0.0f, 0.0f, 0.0f, 0.0f));
    float4 _748 = paths_visual.SampleLevel(sampler_lnc, _717, 0.0f);
    float _749 = _748.x;
    float _754 = pow(_749, 0.5f);
    float2 _756 = _715 * 0.20000000298023223876953125f;
    float4 _760 = dirthpath_heightmap.SampleBias(sampler_llr, _756 * 0.4000000059604644775390625f, frame_constants_global_mip_bias);
    float _761 = _760.x;
    float4 _765 = dirt_1.SampleBias(sampler_llr, _756 * 0.5f, frame_constants_global_mip_bias);
    float4 _771 = dirt_2.SampleBias(sampler_llr, _756 * 1.2000000476837158203125f, frame_constants_global_mip_bias);
    float4 _777 = grass.SampleBias(sampler_llr, _756 * 0.800000011920928955078125f, frame_constants_global_mip_bias);
    float3 _780 = _350 * (_777.y * 5.5f);
    float3 _789;
    if (_256)
    {
        _789 = float3(0.374780178070068359375f, 0.3005877435207366943359375f, 0.2449434697628021240234375f);
    }
    else
    {
        float3 _788;
        if (frame_constants_glade_theme_id == 3u)
        {
            _788 = _780 * 0.75f;
        }
        else
        {
            _788 = _780;
        }
        _789 = _788;
    }
    float _791 = clamp(_754 * 0.60000002384185791015625f, 0.0f, 1.0f);
    float _793 = 0.1500000059604644775390625f * (1.0f - _791);
    float _794 = _761 * _791;
    float _796 = max(_793, _794) - 0.100000001490116119384765625f;
    float _800 = max(_794 - _796, 0.0f);
    float3 _804 = (1.0f.xxx * _800) / (max(_793 - _796, 0.0f) + _800).xxx;
    float _805 = _804.x;
    float _807 = clamp(pow(dirt_mask.SampleLevel(sampler_lnc, _717, 0.0f).x, 0.5f) * 0.60000002384185791015625f, 0.0f, 1.0f);
    float _809 = 0.1500000059604644775390625f * (1.0f - _807);
    float _810 = _761 * _807;
    float _812 = max(_809, _810) - 0.100000001490116119384765625f;
    float _816 = max(_810 - _812, 0.0f);
    float3 _820 = (1.0f.xxx * _816) / (max(_809 - _812, 0.0f) + _816).xxx;
    float _821 = _820.x;
    float _975;
    float _976;
    if (_256)
    {
        int2 _825 = int2(floor(_715));
        float2 _826 = frac(_715);
        uint2 _827 = uint2(_825);
        uint _829 = _827.y;
        uint _831 = _829 + (_829 << 10u);
        uint _833 = _831 ^ (_831 >> 6u);
        uint _835 = _833 + (_833 << 3u);
        uint _837 = _835 ^ (_835 >> 11u);
        uint _843 = (((_827.x * 1664525u) + (_837 + (_837 << 15u))) + 1013904223u) * 1664525u;
        uint _845 = _843 ^ (_843 >> 11u);
        uint _848 = _845 ^ ((_845 << 7u) & 2636928640u);
        uint _851 = _848 ^ ((_848 << 15u) & 4022730752u);
        uint2 _859 = uint2(_825 + int2(1, 0));
        uint _861 = _859.y;
        uint _863 = _861 + (_861 << 10u);
        uint _865 = _863 ^ (_863 >> 6u);
        uint _867 = _865 + (_865 << 3u);
        uint _869 = _867 ^ (_867 >> 11u);
        uint _875 = (((_859.x * 1664525u) + (_869 + (_869 << 15u))) + 1013904223u) * 1664525u;
        uint _877 = _875 ^ (_875 >> 11u);
        uint _880 = _877 ^ ((_877 << 7u) & 2636928640u);
        uint _883 = _880 ^ ((_880 << 15u) & 4022730752u);
        uint2 _891 = uint2(_825 + int2(0, 1));
        uint _893 = _891.y;
        uint _895 = _893 + (_893 << 10u);
        uint _897 = _895 ^ (_895 >> 6u);
        uint _899 = _897 + (_897 << 3u);
        uint _901 = _899 ^ (_899 >> 11u);
        uint _907 = (((_891.x * 1664525u) + (_901 + (_901 << 15u))) + 1013904223u) * 1664525u;
        uint _909 = _907 ^ (_907 >> 11u);
        uint _912 = _909 ^ ((_909 << 7u) & 2636928640u);
        uint _915 = _912 ^ ((_912 << 15u) & 4022730752u);
        uint2 _923 = uint2(_825 + int2(1, 1));
        uint _925 = _923.y;
        uint _927 = _925 + (_925 << 10u);
        uint _929 = _927 ^ (_927 >> 6u);
        uint _931 = _929 + (_929 << 3u);
        uint _933 = _931 ^ (_931 >> 11u);
        uint _939 = (((_923.x * 1664525u) + (_933 + (_933 << 15u))) + 1013904223u) * 1664525u;
        uint _941 = _939 ^ (_939 >> 11u);
        uint _944 = _941 ^ ((_941 << 7u) & 2636928640u);
        uint _947 = _944 ^ ((_944 << 15u) & 4022730752u);
        float2 _957 = (_826 * _826) * (3.0f.xx - (_826 * 2.0f));
        float _958 = _957.x;
        float _959 = 1.0f - _958;
        float _961 = _957.y;
        float _972 = ((((1.0f - _961) * (((asfloat(((_851 ^ (_851 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _959) + ((asfloat(((_883 ^ (_883 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _958))) + (((asfloat(((_915 ^ (_915 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _959) * _961)) + (((asfloat(((_947 ^ (_947 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _958) * _961)) * 0.4000000059604644775390625f;
        _975 = _805 - _972;
        _976 = _821 - _972;
    }
    else
    {
        _975 = _805;
        _976 = _821;
    }
    bool _979 = _256 ? true : false;
    uint _980 = _256 ? 1065353216u : _207;
    uint _981 = _256 ? 8388607u : _207;
    float3 _987 = lerp(lerp(_789, _771.xyz * 1.10000002384185791015625f, smoothstep(0.20000000298023223876953125f, 0.64999997615814208984375f, _976).xxx), _765.xyz * 1.10000002384185791015625f, smoothstep(0.60000002384185791015625f, 1.0f, _975).xxx);
    float3 _994;
    if (_256)
    {
        _994 = lerp(_350, _987, smoothstep(0.0f, 0.25f, _975).xxx);
    }
    else
    {
        _994 = _987;
    }
    float4 _998 = water_mask.SampleLevel(sampler_llc, _717, 0.0f);
    float _999 = _998.y;
    float4 _1003 = mud.SampleLevel(sampler_llr, _717 * 10.0f, 0.0f);
    float3 _1004 = _1003.xyz;
    float _1011 = lerp(0.100000001490116119384765625f, 0.20000000298023223876953125f, smoothstep(0.0500000007450580596923828125f, 0.20000000298023223876953125f, _1003.x));
    float _1025;
    do
    {
        float _1015 = abs(_999 - 0.4000000059604644775390625f);
        if (_1015 > _1011)
        {
            _1025 = 0.0f;
            break;
        }
        float _1019 = _1015 / _1011;
        _1025 = 1.0f - ((_1019 * _1019) * (3.0f - (2.0f * _1019)));
        break;
    } while(false);
    float _1063;
    do
    {
        float _1036 = _717.x;
        bool _1042;
        if (!(_1036 < 0.0f))
        {
            _1042 = _1036 > 1.0f;
        }
        else
        {
            _1042 = true;
        }
        bool _1048;
        if (!_1042)
        {
            _1048 = _717.y < 0.0f;
        }
        else
        {
            _1048 = true;
        }
        bool _1054;
        if (!_1048)
        {
            _1054 = _717.y > 1.0f;
        }
        else
        {
            _1054 = true;
        }
        if (_1054)
        {
            _1063 = 0.0f;
            break;
        }
        else
        {
            _1063 = rocky_terrain.SampleLevel(sampler_default, _717, 0.0f).z;
            break;
        }
        break; 
    } while(false);
    uint _1392;
    uint _1393;
    bool _1394;
    float _1395;
    float3 _1396;
    float _1397;
    if (_746.y > 0.0f)
    {
        float4 _1296 = stone_floor_seed.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias);
        float _1297 = _1296.z;
        int _1307 = int(trunc((_1296.x * 32.0f) - 0.5f) + trunc((_1296.y * 32.0f) - 0.5f));
        uint _1309 = uint(_1307 ^ 7993234);
        uint _1311 = _1309 + (_1309 << 10u);
        uint _1313 = _1311 ^ (_1311 >> 6u);
        uint _1315 = _1313 + (_1313 << 3u);
        uint _1317 = _1315 ^ (_1315 >> 11u);
        bool _1320 = _979 ? _979 : true;
        uint _1321 = _979 ? _981 : 8388607u;
        uint _1322 = _979 ? _980 : 1065353216u;
        uint _1329 = uint(_1307 ^ 9730541);
        uint _1331 = _1329 + (_1329 << 10u);
        uint _1333 = _1331 ^ (_1331 >> 6u);
        uint _1335 = _1333 + (_1333 << 3u);
        uint _1337 = _1335 ^ (_1335 >> 11u);
        uint _1339 = _1337 + (_1337 << 15u);
        bool _1340 = _1320 ? _1320 : true;
        uint _1341 = _1320 ? _1321 : 8388607u;
        uint _1342 = _1320 ? _1322 : 1065353216u;
        float4 _1358 = stone_floor_roughness.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias);
        float _1359 = _1358.x;
        uint _1381 = _1340 ? _1341 : 8388607u;
        uint _1382 = _1340 ? _1342 : 1065353216u;
        _1392 = _1382;
        _1393 = _1381;
        _1394 = _1340 ? _1340 : true;
        _1395 = lerp(0.75f, 1.0f, _1359) * lerp(0.800000011920928955078125f, 1.0f, asfloat((_1339 & _1381) | _1382) - 1.0f);
        _1396 = ((((brick_colors.Sample(sampler_llr, float3((((_715 * 0.100000001490116119384765625f) + float2((asfloat(((_1317 + (_1317 << 15u)) & _1321) | _1322) - 1.0f) * 0.300000011920928955078125f, (asfloat((_1339 & _1341) | _1342) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * _1297).xx) + (_1359 * 0.100000001490116119384765625f).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _1297)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
        _1397 = smoothstep(0.20000000298023223876953125f, 0.75f, 1.0f - stone_floor_height.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x);
    }
    else
    {
        uint _1287;
        uint _1288;
        bool _1289;
        float _1290;
        float3 _1291;
        float _1292;
        if (_746.z > 0.0f)
        {
            float4 _1191 = stone_floor_seed_2.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias);
            float _1192 = _1191.z;
            int _1202 = int(trunc((_1191.x * 32.0f) - 0.5f) + trunc((_1191.y * 32.0f) - 0.5f));
            uint _1204 = uint(_1202 ^ 7993234);
            uint _1206 = _1204 + (_1204 << 10u);
            uint _1208 = _1206 ^ (_1206 >> 6u);
            uint _1210 = _1208 + (_1208 << 3u);
            uint _1212 = _1210 ^ (_1210 >> 11u);
            bool _1215 = _979 ? _979 : true;
            uint _1216 = _979 ? _981 : 8388607u;
            uint _1217 = _979 ? _980 : 1065353216u;
            uint _1224 = uint(_1202 ^ 9730541);
            uint _1226 = _1224 + (_1224 << 10u);
            uint _1228 = _1226 ^ (_1226 >> 6u);
            uint _1230 = _1228 + (_1228 << 3u);
            uint _1232 = _1230 ^ (_1230 >> 11u);
            uint _1234 = _1232 + (_1232 << 15u);
            bool _1235 = _1215 ? _1215 : true;
            uint _1236 = _1215 ? _1216 : 8388607u;
            uint _1237 = _1215 ? _1217 : 1065353216u;
            float4 _1253 = stone_floor_roughness_2.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias);
            float _1254 = _1253.x;
            uint _1276 = _1235 ? _1236 : 8388607u;
            uint _1277 = _1235 ? _1237 : 1065353216u;
            _1287 = _1277;
            _1288 = _1276;
            _1289 = _1235 ? _1235 : true;
            _1290 = lerp(0.75f, 1.0f, _1254) * lerp(0.800000011920928955078125f, 1.0f, asfloat((_1234 & _1276) | _1277) - 1.0f);
            _1291 = ((((brick_colors.Sample(sampler_llr, float3((((_715 * 0.100000001490116119384765625f) + float2((asfloat(((_1212 + (_1212 << 15u)) & _1216) | _1217) - 1.0f) * 0.300000011920928955078125f, (asfloat((_1234 & _1236) | _1237) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * _1192).xx) + (_1254 * 0.100000001490116119384765625f).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _1192)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_2.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
            _1292 = smoothstep(0.300000011920928955078125f, 0.5f, 1.0f - stone_floor_height_2.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x);
        }
        else
        {
            uint _1182;
            uint _1183;
            bool _1184;
            float _1185;
            float3 _1186;
            float _1187;
            if (_746.w > 0.0f)
            {
                float4 _1089 = stone_floor_seed_3.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias);
                float _1090 = _1089.z;
                int _1100 = int(trunc((_1089.x * 32.0f) - 0.5f) + trunc((_1089.y * 32.0f) - 0.5f));
                uint _1102 = uint(_1100 ^ 7993234);
                uint _1104 = _1102 + (_1102 << 10u);
                uint _1106 = _1104 ^ (_1104 >> 6u);
                uint _1108 = _1106 + (_1106 << 3u);
                uint _1110 = _1108 ^ (_1108 >> 11u);
                bool _1113 = _979 ? _979 : true;
                uint _1114 = _979 ? _981 : 8388607u;
                uint _1115 = _979 ? _980 : 1065353216u;
                uint _1122 = uint(_1100 ^ 9730541);
                uint _1124 = _1122 + (_1122 << 10u);
                uint _1126 = _1124 ^ (_1124 >> 6u);
                uint _1128 = _1126 + (_1126 << 3u);
                uint _1130 = _1128 ^ (_1128 >> 11u);
                uint _1132 = _1130 + (_1130 << 15u);
                bool _1133 = _1113 ? _1113 : true;
                uint _1134 = _1113 ? _1114 : 8388607u;
                uint _1135 = _1113 ? _1115 : 1065353216u;
                uint _1171 = _1133 ? _1134 : 8388607u;
                uint _1172 = _1133 ? _1135 : 1065353216u;
                _1182 = _1172;
                _1183 = _1171;
                _1184 = _1133 ? _1133 : true;
                _1185 = lerp(0.75f, 1.0f, stone_floor_roughness_3.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat((_1132 & _1171) | _1172) - 1.0f);
                _1186 = ((((brick_colors.Sample(sampler_llr, float3(((_715 * 0.100000001490116119384765625f) + float2((asfloat(((_1110 + (_1110 << 15u)) & _1114) | _1115) - 1.0f) * 0.300000011920928955078125f, (asfloat((_1132 & _1134) | _1135) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * _1090).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, _1090)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_3.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
                _1187 = smoothstep(0.300000011920928955078125f, 1.0f, 1.0f - stone_floor_height_3.SampleBias(sampler_llr, _627, frame_constants_global_mip_bias).x);
            }
            else
            {
                _1182 = _980;
                _1183 = _981;
                _1184 = _979;
                _1185 = lerp(1.0f, 0.89999997615814208984375f, _975);
                _1186 = lerp(lerp(lerp(_994, _1004, smoothstep(0.4000000059604644775390625f, 0.5f, _999).xxx), _1004 * 0.75f, _1025.xxx), _1004, (smoothstep(0.3499999940395355224609375f, 0.449999988079071044921875f, (garden_mask.SampleLevel(sampler_llc, _717, 0.0f).y - smoothstep(0.300000011920928955078125f, 0.5f, _754)) - smoothstep(0.300000011920928955078125f, 0.5f, _1063)) * (_256 ? 0.60000002384185791015625f : 1.0f)).xxx);
                _1187 = 0.0f;
            }
            _1287 = _1182;
            _1288 = _1183;
            _1289 = _1184;
            _1290 = _1185;
            _1291 = _1186;
            _1292 = _1187;
        }
        _1392 = _1287;
        _1393 = _1288;
        _1394 = _1289;
        _1395 = _1290;
        _1396 = _1291;
        _1397 = _1292;
    }
    bool _1402;
    if (_256)
    {
        _1402 = in_var_C1.y > 0.0f;
    }
    else
    {
        _1402 = false;
    }
    float3 _1608;
    float3 _1609;
    if (_1402)
    {
        float2 _1427 = in_var_C1.xz * 5.0f;
        int2 _1429 = int2(floor(_1427));
        float2 _1430 = frac(_1427);
        uint2 _1431 = uint2(_1429);
        uint _1433 = _1431.y;
        uint _1435 = _1433 + (_1433 << 10u);
        uint _1437 = _1435 ^ (_1435 >> 6u);
        uint _1439 = _1437 + (_1437 << 3u);
        uint _1441 = _1439 ^ (_1439 >> 11u);
        bool _1444 = _979 ? _979 : true;
        uint _1445 = _979 ? (_256 ? 1664525u : _207) : 1664525u;
        uint _1446 = _979 ? (_256 ? 1013904223u : _207) : 1013904223u;
        uint _1450 = (((_1431.x * _1445) + (_1441 + (_1441 << 15u))) + _1446) * _1445;
        uint _1452 = _1450 ^ (_1450 >> 11u);
        uint _1455 = _1452 ^ ((_1452 << 7u) & 2636928640u);
        uint _1458 = _1455 ^ ((_1455 << 15u) & 4022730752u);
        bool _1461 = _1394 ? _1394 : true;
        uint _1462 = _1394 ? _1393 : 8388607u;
        uint _1463 = _1394 ? _1392 : 1065353216u;
        uint2 _1469 = uint2(_1429 + int2(1, 0));
        uint _1471 = _1469.y;
        uint _1473 = _1471 + (_1471 << 10u);
        uint _1475 = _1473 ^ (_1473 >> 6u);
        uint _1477 = _1475 + (_1475 << 3u);
        uint _1479 = _1477 ^ (_1477 >> 11u);
        bool _1482 = _1444 ? _1444 : true;
        uint _1483 = _1444 ? _1445 : 1664525u;
        uint _1484 = _1444 ? _1446 : 1013904223u;
        uint _1488 = (((_1469.x * _1483) + (_1479 + (_1479 << 15u))) + _1484) * _1483;
        uint _1490 = _1488 ^ (_1488 >> 11u);
        uint _1493 = _1490 ^ ((_1490 << 7u) & 2636928640u);
        uint _1496 = _1493 ^ ((_1493 << 15u) & 4022730752u);
        bool _1499 = _1461 ? _1461 : true;
        uint _1500 = _1461 ? _1462 : 8388607u;
        uint _1501 = _1461 ? _1463 : 1065353216u;
        uint2 _1507 = uint2(_1429 + int2(0, 1));
        uint _1509 = _1507.y;
        uint _1511 = _1509 + (_1509 << 10u);
        uint _1513 = _1511 ^ (_1511 >> 6u);
        uint _1515 = _1513 + (_1513 << 3u);
        uint _1517 = _1515 ^ (_1515 >> 11u);
        bool _1520 = _1482 ? _1482 : true;
        uint _1521 = _1482 ? _1483 : 1664525u;
        uint _1522 = _1482 ? _1484 : 1013904223u;
        uint _1526 = (((_1507.x * _1521) + (_1517 + (_1517 << 15u))) + _1522) * _1521;
        uint _1528 = _1526 ^ (_1526 >> 11u);
        uint _1531 = _1528 ^ ((_1528 << 7u) & 2636928640u);
        uint _1534 = _1531 ^ ((_1531 << 15u) & 4022730752u);
        bool _1537 = _1499 ? _1499 : true;
        uint _1538 = _1499 ? _1500 : 8388607u;
        uint _1539 = _1499 ? _1501 : 1065353216u;
        uint2 _1545 = uint2(_1429 + int2(1, 1));
        uint _1547 = _1545.y;
        uint _1549 = _1547 + (_1547 << 10u);
        uint _1551 = _1549 ^ (_1549 >> 6u);
        uint _1553 = _1551 + (_1551 << 3u);
        uint _1555 = _1553 ^ (_1553 >> 11u);
        uint _1558 = _1520 ? _1521 : 1664525u;
        uint _1563 = (((_1545.x * _1558) + (_1555 + (_1555 << 15u))) + (_1520 ? _1522 : 1013904223u)) * _1558;
        uint _1565 = _1563 ^ (_1563 >> 11u);
        uint _1568 = _1565 ^ ((_1565 << 7u) & 2636928640u);
        uint _1571 = _1568 ^ ((_1568 << 15u) & 4022730752u);
        float2 _1583 = (_1430 * _1430) * (3.0f.xx - (_1430 * 2.0f));
        float _1584 = _1583.x;
        float _1585 = 1.0f - _1584;
        float _1587 = _1583.y;
        float3 _1604 = ((((clamp(smoothstep(0.20000000298023223876953125f, 1.0f, dirthpath_heightmap.SampleBias(sampler_llr, _756 * 0.1500000059604644775390625f, frame_constants_global_mip_bias).x * lerp(0.64999997615814208984375f, 1.0f, _1397)), 0.100000001490116119384765625f, lerp(0.5f, 0.20000000298023223876953125f, _749)) * clamp(_703.y, 0.0f, 1.0f)) * (1.0f - (_749 * 0.5f))) + lerp(0.0f, 0.20000000298023223876953125f, smoothstep(0.0f, 0.4000000059604644775390625f, _1397))) * (1.0f - smoothstep(0.0f, 0.300000011920928955078125f, buildings_mask.SampleLevel(sampler_llc, _243, 0.0f).y - ((((((1.0f - _1587) * (((asfloat(((_1458 ^ (_1458 >> 18u)) & _1462) | _1463) - 1.0f) * _1585) + ((asfloat(((_1496 ^ (_1496 >> 18u)) & _1500) | _1501) - 1.0f) * _1584))) + (((asfloat(((_1534 ^ (_1534 >> 18u)) & _1538) | _1539) - 1.0f) * _1585) * _1587)) + (((asfloat(((_1571 ^ (_1571 >> 18u)) & (_1537 ? _1538 : 8388607u)) | (_1537 ? _1539 : 1065353216u)) - 1.0f) * _1584) * _1587)) * 0.100000001490116119384765625f) * _1397)))).xxx;
        _1608 = normalize(lerp(_703, in_var_C0, _1604));
        _1609 = lerp(_1396, 0.699999988079071044921875f.xxx, _1604);
    }
    else
    {
        _1608 = _703;
        _1609 = _1396;
    }
    float3 _1611 = (_1608 * 0.5f) + 0.5f.xxx;
    float3 _1638 = sqrt(clamp(_1609, 0.0f.xxx, 1.0f.xxx));
    uint _1657 = (uint((clamp(_1638.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_1638.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_1638.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _1658 = uint2(((uint((clamp(_1611.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_1611.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_1611.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_1395, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _1657);
    _1658.y = _1657 | 905969664u;
    out_var_SV_TARGET0 = _1658;
    gl_FragDepth = (_639.xyz / _639.w.xxx).z;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)

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
    uint2 _Globals_buildings_mask_dims : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_rocky_terrain_dims : packoffset(c2);
    uint2 _Globals_paths_visual_dims : packoffset(c2.z);
    uint2 _Globals_water_mask_dims : packoffset(c3);
    uint2 _Globals_highlight_active_hill_dims : packoffset(c3.z);
    uint2 _Globals_mud_dims : packoffset(c4);
    uint2 _Globals_noise_debug_dims : packoffset(c4.z);
    uint2 _Globals_floor_heightmap_dims : packoffset(c5);
    uint2 _Globals_dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 _Globals_dirt_1_dims : packoffset(c6);
    uint2 _Globals_dirt_2_dims : packoffset(c6.z);
    uint2 _Globals_grass_dims : packoffset(c7);
    uint2 _Globals_dirt_mask_dims : packoffset(c7.z);
    uint2 _Globals_stone_floor_normal_dims : packoffset(c8);
    uint2 _Globals_stone_floor_seed_dims : packoffset(c8.z);
    uint2 _Globals_stone_floor_height_dims : packoffset(c9);
    uint2 _Globals_stone_floor_roughness_dims : packoffset(c9.z);
    uint2 _Globals_stone_floor_seed_2_dims : packoffset(c10);
    uint2 _Globals_stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 _Globals_stone_floor_height_2_dims : packoffset(c11);
    uint2 _Globals_stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 _Globals_stone_floor_seed_3_dims : packoffset(c12);
    uint2 _Globals_stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 _Globals_stone_floor_height_3_dims : packoffset(c13);
    uint2 _Globals_stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 _Globals_garden_mask_dims : packoffset(c14);
    uint _Globals_force_top_down_render : packoffset(c14.z);
};

SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t16, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t20, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 _147;
    float _148;
    bool _150;
    float2 _129 = ((((float2(int2(float2(float(uint(gl_VertexIndex) % 91u), float(uint(gl_VertexIndex) / 91u)))) + 0.5f.xx) * 0.010989011265337467193603515625f.xx) * 2.0f) - 1.0f.xx) * 65.72222137451171875f.xx;
    float3 _132 = float3(_129.x, 0.0f, _129.y);
    _132.y = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (_132.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
    float _174;
    do
    {
        _147 = (_132.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        _148 = _147.x;
        _150 = !(_148 < 0.0f);
        bool _154;
        if (_150)
        {
            _154 = _148 > 1.0f;
        }
        else
        {
            _154 = true;
        }
        bool _160;
        if (!_154)
        {
            _160 = _147.y < 0.0f;
        }
        else
        {
            _160 = true;
        }
        bool _166;
        if (!_160)
        {
            _166 = _147.y > 1.0f;
        }
        else
        {
            _166 = true;
        }
        if (_166)
        {
            _174 = 0.0f;
            break;
        }
        else
        {
            _174 = rocky_terrain.SampleLevel(sampler_default, _147, 0.0f).x;
            break;
        }
        break; 
    } while(false);
    float _200;
    do
    {
        bool _180;
        if (_150)
        {
            _180 = _148 > 1.0f;
        }
        else
        {
            _180 = true;
        }
        bool _186;
        if (!_180)
        {
            _186 = _147.y < 0.0f;
        }
        else
        {
            _186 = true;
        }
        bool _192;
        if (!_186)
        {
            _192 = _147.y > 1.0f;
        }
        else
        {
            _192 = true;
        }
        if (_192)
        {
            _200 = 0.0f;
            break;
        }
        else
        {
            _200 = rocky_terrain.SampleLevel(sampler_default, _147, 0.0f).z;
            break;
        }
        break; 
    } while(false);
    float4 _202 = terrain_heightmap.GatherRed(sampler_default, _147, int2(0, 0));
    float2 _206 = 130.0f.xx / float2(_Globals_terrain_heightmap_dims);
    float _207 = _202.z;
    float _208 = _202.y;
    float _210 = _202.w;
    float _211 = _202.x;
    float2 _220 = ((float2((_207 + _208) - (_210 + _211), (_211 + _208) - (_210 + _207)) * 0.5f.xx) * 26.5f) / _206;
    float3 _227 = cross(normalize(float3(0.0f, _220.y, 1.0f)), normalize(float3(1.0f, _220.x, 0.0f)));
    float4 _232 = terrain_heightmap.GatherRed(sampler_default, _147, int2(0, 0));
    float _233 = _232.z;
    float _234 = _232.y;
    float _236 = _232.w;
    float _237 = _232.x;
    float _247 = length(((float2((_233 + _234) - (_236 + _237), (_237 + _234) - (_236 + _233)) * 0.5f.xx) * 26.5f) / _206);
    float3 _253;
    if (_247 < 2.0f)
    {
        _253 = _132 - (_227 * lerp(0.0f, 0.5f, clamp(_174 + _200, 0.0f, 1.0f)));
    }
    else
    {
        _253 = _132;
    }
    float _266 = lerp(_253.y, -0.800000011920928955078125f, smoothstep(0.20000000298023223876953125f, 0.60000002384185791015625f, water_mask.SampleLevel(sampler_llc, (_253.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, _247));
    float3 _267 = _253;
    _267.y = _266;
    float4 _270 = float4(_253.x, _266, _253.z, 1.0f);
    float4 _290;
    if (_Globals_force_top_down_render != 0u)
    {
        _290 = float4(_270.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), clamp((_266 - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        _290 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _270);
    }
    gl_Position = _290;
    out_var_C0 = _227;
    out_var_C1 = _267;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

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
