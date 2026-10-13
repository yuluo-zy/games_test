// ============================================================
// terrain_editing/floor.raster.hlsl
// 功能:地面(floor)渲染。vs_main 由 SV_VertexID 生成 91x91 地形网格顶点,采样地形高度
//       + 岩质地表高度,计算地形法线并做水面下陷;ps_main 是大型视差遮蔽(POM)着色器:
//       草 Simplex 噪声 + 路径视觉贴图 + 石板 POM 步进/二分细化 + 石板砖块随机色 +
//       泥土/路径/水/泥混合 + 雪景分支,打包法线/反照率到 uint2 SV_Target0 并写深度。
// 入口:vs_main(SV_VertexID)、ps_main(SV_Target0 + SV_DepthLessEqual)。
// 变体:两个 TGV 变体(b903e43ffb3da915 / f0adff76827e73ff)顶点着色器相同;像素着色器
//       b903 为完整 POM 版本,f0ad 为空(仅入口)。
// 绑定:t15 buildings_mask、t16 terrain_heightmap、t18 rocky_terrain、t19 paths_visual、
//       t20 water_mask、t22 mud、t25 dirthpath_heightmap、t26 dirt_1、t27 dirt_2、t28 grass、
//       t29 dirt_mask、t31 brick_colors(Texture2DArray)、t33..t44 三组石板纹理、t45 garden_mask、
//       b13 $Globals、b14 color_library、b0 space1 frame_constants。
// ============================================================

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提升段:TGV_b903e43ffb3da915 变体,顶点着色器(行为等价,语义重命名)。
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
    uint current_view_index : packoffset(c0);
    uint2 buildings_mask_dims : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 paths_visual_dims : packoffset(c2.z);
    uint2 water_mask_dims : packoffset(c3);
    uint2 highlight_active_hill_dims : packoffset(c3.z);
    uint2 mud_dims : packoffset(c4);
    uint2 noise_debug_dims : packoffset(c4.z);
    uint2 floor_heightmap_dims : packoffset(c5);
    uint2 dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 dirt_1_dims : packoffset(c6);
    uint2 dirt_2_dims : packoffset(c6.z);
    uint2 grass_dims : packoffset(c7);
    uint2 dirt_mask_dims : packoffset(c7.z);
    uint2 stone_floor_normal_dims : packoffset(c8);
    uint2 stone_floor_seed_dims : packoffset(c8.z);
    uint2 stone_floor_height_dims : packoffset(c9);
    uint2 stone_floor_roughness_dims : packoffset(c9.z);
    uint2 stone_floor_seed_2_dims : packoffset(c10);
    uint2 stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 stone_floor_height_2_dims : packoffset(c11);
    uint2 stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 stone_floor_seed_3_dims : packoffset(c12);
    uint2 stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 stone_floor_height_3_dims : packoffset(c13);
    uint2 stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 garden_mask_dims : packoffset(c14);
    uint force_top_down_render : packoffset(c14.z);
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
    // 91x91 地形网格顶点位置。
    float2 uv;
    float uv_x;
    bool uv_x_ok;
    float2 grid_pos = ((((float2(int2(float2(float(uint(gl_VertexIndex) % 91u), float(uint(gl_VertexIndex) / 91u)))) + 0.5f.xx) * 0.010989011265337467193603515625f.xx) * 2.0f) - 1.0f.xx) * 65.72222137451171875f.xx;
    // 地形高度采样,映射到 [-2.5, 24]。
    float3 terrain_pos = float3(grid_pos.x, 0.0f, grid_pos.y);
    terrain_pos.y = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (terrain_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
    float rocky_x;
    do
    {
        uv = (terrain_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        uv_x = uv.x;
        uv_x_ok = !(uv_x < 0.0f);
        bool uv_x_high;
        if (uv_x_ok)
        {
            uv_x_high = uv_x > 1.0f;
        }
        else
        {
            uv_x_high = true;
        }
        bool uv_y_low;
        if (!uv_x_high)
        {
            uv_y_low = uv.y < 0.0f;
        }
        else
        {
            uv_y_low = true;
        }
        bool uv_y_high;
        if (!uv_y_low)
        {
            uv_y_high = uv.y > 1.0f;
        }
        else
        {
            uv_y_high = true;
        }
        if (uv_y_high)
        {
            rocky_x = 0.0f;
            break;
        }
        else
        {
            rocky_x = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f).x;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float rocky_z;
    do
    {
        bool uv_x_high2;
        if (uv_x_ok)
        {
            uv_x_high2 = uv_x > 1.0f;
        }
        else
        {
            uv_x_high2 = true;
        }
        bool uv_y_low2;
        if (!uv_x_high2)
        {
            uv_y_low2 = uv.y < 0.0f;
        }
        else
        {
            uv_y_low2 = true;
        }
        bool uv_y_high2;
        if (!uv_y_low2)
        {
            uv_y_high2 = uv.y > 1.0f;
        }
        else
        {
            uv_y_high2 = true;
        }
        if (uv_y_high2)
        {
            rocky_z = 0.0f;
            break;
        }
        else
        {
            rocky_z = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f).z;
            break;
        }
        break; // unreachable workaround
    } while(false);
    // GatherRed:中心差分计算地形法线。
    float4 gather = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float2 texel_scale = 130.0f.xx / float2(terrain_heightmap_dims);
    float v207 = gather.z;
    float g_y = gather.y;
    float g_w = gather.w;
    float g_x = gather.x;
    float2 grad = ((float2((v207 + g_y) - (g_w + g_x), (g_x + g_y) - (g_w + v207)) * 0.5f.xx) * 26.5f) / texel_scale;
    // 地形法线(由梯度叉积)。
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    float4 gather2 = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float v233 = gather2.z;
    float g2_y = gather2.y;
    float g2_w = gather2.w;
    float g2_x = gather2.x;
    float v247 = length(((float2((v233 + g2_y) - (g2_w + g2_x), (g2_x + g2_y) - (g2_w + v233)) * 0.5f.xx) * 26.5f) / texel_scale);
    // 岩质地表高度混合(沿法线下压)。
    float3 displ_pos;
    if (v247 < 2.0f)
    {
        displ_pos = terrain_pos - (normal * lerp(0.0f, 0.5f, clamp(rocky_x + rocky_z, 0.0f, 1.0f)));
    }
    else
    {
        displ_pos = terrain_pos;
    }
    // 水面下陷:沿水遮罩把地面压到 -0.8。
    float water_y = lerp(displ_pos.y, -0.800000011920928955078125f, smoothstep(0.20000000298023223876953125f, 0.60000002384185791015625f, water_mask.SampleLevel(sampler_llc, (displ_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, v247));
    float3 v267 = displ_pos;
    v267.y = water_y;
    float4 pos4 = float4(displ_pos.x, water_y, displ_pos.z, 1.0f);
    float4 clip_pos;
    if (force_top_down_render != 0u)
    {
        clip_pos = float4(pos4.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), clamp((water_y - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, pos4);
    }
    // 输出:顶点位置 + 法线(C0)/世界坐标(C1)。
    gl_Position = clip_pos;
    out_var_C0 = normal;
    out_var_C1 = v267;
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
// 提升段:TGV_b903e43ffb3da915 变体,像素着色器(POM 石板地面,行为等价,语义重命名)。
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

static uint v207;
static float3 v233;

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
    uint2 buildings_mask_dims : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 paths_visual_dims : packoffset(c2.z);
    uint2 water_mask_dims : packoffset(c3);
    uint2 highlight_active_hill_dims : packoffset(c3.z);
    uint2 mud_dims : packoffset(c4);
    uint2 noise_debug_dims : packoffset(c4.z);
    uint2 floor_heightmap_dims : packoffset(c5);
    uint2 dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 dirt_1_dims : packoffset(c6);
    uint2 dirt_2_dims : packoffset(c6.z);
    uint2 grass_dims : packoffset(c7);
    uint2 dirt_mask_dims : packoffset(c7.z);
    uint2 stone_floor_normal_dims : packoffset(c8);
    uint2 stone_floor_seed_dims : packoffset(c8.z);
    uint2 stone_floor_height_dims : packoffset(c9);
    uint2 stone_floor_roughness_dims : packoffset(c9.z);
    uint2 stone_floor_seed_2_dims : packoffset(c10);
    uint2 stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 stone_floor_height_2_dims : packoffset(c11);
    uint2 stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 stone_floor_seed_3_dims : packoffset(c12);
    uint2 stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 stone_floor_height_3_dims : packoffset(c13);
    uint2 stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 garden_mask_dims : packoffset(c14);
    uint force_top_down_render : packoffset(c14.z);
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
    // 像素着色器:草地/泥土/石板 POM 混合地面。
    float2 uv0 = (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    // 路径视觉贴图:alpha 区分草地 vs 路径。
    float4 paths_sample = paths_visual.SampleLevel(sampler_lnc, uv0, 0.0f);
    float3 v247 = paths_sample.yzw;
    bool is_theme4 = frame_constants_glade_theme_id == 4u;
    // 草 Simplex 值噪声(主题 4 用更高频率)。
    float2 noise_pos = (in_var_C1.xz * (is_theme4 ? 0.199999988079071044921875f : 0.04999999701976776123046875f)) + 7.0f.xx;
    float2 sim_cell = floor(noise_pos + dot(noise_pos, 0.3660254180431365966796875f.xx).xx);
    float2 v267 = (noise_pos - sim_cell) + dot(sim_cell, 0.211324870586395263671875f.xx).xx;
    float sim_fx = v267.x;
    float sim_fy = v267.y;
    bool2 v271 = (sim_fx > sim_fy).xx;
    float2 sim_uv = float2(v271.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, v271.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 sim_off = v267.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 sim_off_xy = sim_off.xy - sim_uv;
    float4 sim_off4 = float4(sim_off_xy.x, sim_off_xy.y, sim_off.z, sim_off.w);
    float2 sim_cell_f = fmod(sim_cell, 289.0f.xx);
    float3 sim_ax = sim_cell_f.y.xxx + float3(0.0f, sim_uv.y, 1.0f);
    float3 sim_ay = (fmod(((sim_ax * 34.0f) + 1.0f.xxx) * sim_ax, 289.0f.xxx) + sim_cell_f.x.xxx) + float3(0.0f, sim_uv.x, 1.0f);
    float2 v299 = sim_off_xy.xy;
    float2 v301 = sim_off.zw;
    float3 sim_dist = max(0.5f.xxx - float3(dot(v267, v267), dot(v299, v299), dot(v301, v301)), 0.0f.xxx);
    float3 sim_dist2 = sim_dist * sim_dist;
    float3 sim_h = frac(fmod(((sim_ay * 34.0f) + 1.0f.xxx) * sim_ay, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 sim_g = sim_h - 1.0f.xxx;
    float3 sim_gabs = abs(sim_g) - 0.5f.xxx;
    float3 sim_f2 = sim_g - floor(sim_h + (-0.5f).xxx);
    float3 sim_dot1;
    sim_dot1.x = (sim_f2.x * sim_fx) + (sim_gabs.x * sim_fy);
    float2 sim_dot23 = (sim_f2.yz * sim_off4.xz) + (sim_gabs.yz * sim_off4.yw);
    float grass_noise = 130.0f * dot((sim_dist2 * sim_dist2) * (1.792842864990234375f.xxx - (((sim_f2 * sim_f2) + (sim_gabs * sim_gabs)) * 0.8537347316741943359375f)), float3(sim_dot1.x, sim_dot23.x, sim_dot23.y));
    float3 grass_color = lerp(color_library_grass_1.xyz, color_library_grass_2.xyz, clamp(grass_noise - 0.4000000059604644775390625f, 0.0f, 1.0f).xxx);
    float3 grass_color2;
    if (is_theme4)
    {
        grass_color2 = lerp(0.699999988079071044921875f.xxx, grass_color, clamp(grass_noise - 0.20000000298023223876953125f, 0.0f, 1.0f).xxx);
    }
    else
    {
        grass_color2 = grass_color;
    }
    float3 local_d;
    // 石板 POM:计算 TBN 基与视差射线。
    float3 grass_col = grass_color2 * 0.75f;
    float2 stone_uv = in_var_C1.xz * 0.20000000298023223876953125f;
    float _355_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, stone_uv);
    float2 stone_lod = _355_tmp.xx;
    float stone_lod_bias = stone_lod.x + frame_constants_global_mip_bias;
    uint2 px_yx = uint2(gl_FragCoord.yx);
    float frame_rot = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
    float frame_rand = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_yx.x) + frame_rot)) + (0.005837149918079376220703125f * (float(px_yx.y) + frame_rot))));
    float path_alpha_inv = 1.0f - paths_sample.x;
    float path_h = clamp(1.0f - dot(1.0f.xxx, (v247 * 2.0f) * path_alpha_inv), 0.0f, 1.0f);
    float3 path_d = v247 * float3(1.0f, 2.0f, 1.0f);
    float3 path_dn = path_d / max(0.00999999977648258209228515625f, max(path_d.x, max(path_d.y, path_d.z))).xxx;
    float3 ddx_c1 = ddx(in_var_C1);
    float3 ddy_c1 = ddy(in_var_C1);
    float2 ddx_suv = ddx(stone_uv);
    float2 ddy_suv = ddy(stone_uv);
    float2 neg_ddy = -ddy_suv;
    float3 c1 = cross(in_var_C0, -ddy_c1);
    float3 c2 = cross(ddx_c1, in_var_C0);
    float ddx_x = ddx_suv.x;
    float ddy_y = neg_ddy.y;
    float ddx_y = ddx_suv.y;
    float ddy_x = neg_ddy.x;
    float det_inv = 1.0f / ((ddx_x * ddy_y) - (ddx_y * ddy_x));
    float3x3 uv_basis = transpose(float3x3(((c1 * ddx_x) + (c2 * ddy_x)) * det_inv, ((c1 * ddx_y) + (c2 * ddy_y)) * det_inv, in_var_C0));
    // 相机位置(世界)。
    float4 cam_pos4 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 tbn_x = mul(uv_basis, float3(1.0f, 0.0f, 0.0f));
    float3 tbn_y = mul(uv_basis, float3(0.0f, 1.0f, 0.0f));
    float3 tbn_z = mul(uv_basis, float3(0.0f, 0.0f, 1.0f));
    float3 tbn_cross = cross(tbn_x, tbn_y);
    float3 tbn_scale = (1.0f / dot(tbn_z, tbn_cross)).xxx;
    float3x3 tbn_inv = float3x3(cross(tbn_y, tbn_z) * tbn_scale, cross(tbn_z, tbn_x) * tbn_scale, tbn_cross * tbn_scale);
    float3 local_pos = mul(tbn_inv, in_var_C1 - (cam_pos4.xyz / cam_pos4.w.xxx));
    float3 local_o = float3(stone_uv, 0.0f);
    // 视差遮蔽步进 + 二分细化。
    float t_final;
    do
    {
        local_d = (local_o + (local_pos * ((-0.100000001490116119384765625f) / local_pos.z))) - local_o;
        float inv_steps = 1.0f / max(1.0f, 2.0f);
        float t0 = lerp(0.0f, 1.0f, pow(frame_rand * inv_steps, 2.0f));
        float3 suv4 = local_o + (local_d * t0);
        float2 suv = suv4.xy;
        float3 h012 = (float3(stone_floor_height.SampleLevel(sampler_llr, suv, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, suv, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, suv, stone_lod_bias).x) - 1.0f.xxx) + path_dn;
        float hdiff = suv4.z - ((max(max(path_h, h012.x), max(h012.y, h012.z)) - 1.0f) * 0.100000001490116119384765625f);
        bool hit0 = (hdiff < 0.0f) ? true : false;
        float h_lo;
        float h_hi;
        float t_hi;
        bool hit_flag;
        float t_lo;
        if (hit0)
        {
            h_lo = hdiff;
            h_hi = 0.0f;
            t_hi = t0;
            hit_flag = hit0;
            t_lo = 0.0f;
        }
        else
        {
            float h_prev;
            float t_prev;
            h_prev = hdiff;
            t_prev = t0;
            float h_cur;
            bool hit_cur;
            float t_cur;
            float h_hit;
            float t_hit;
            bool hit_fin;
            uint step_i = 1u;
            bool hit_prev = hit0;
            for (;;)
            {
                if (step_i < 3u)
                {
                    t_cur = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + frame_rand) * inv_steps, 2.0f)));
                    float3 suv4b = local_o + (local_d * t_cur);
                    float2 suvb = suv4b.xy;
                    float3 h012b = (float3(stone_floor_height.SampleLevel(sampler_llr, suvb, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, suvb, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, suvb, stone_lod_bias).x) - 1.0f.xxx) + path_dn;
                    h_cur = suv4b.z - ((max(max(path_h, h012b.x), max(h012b.y, h012b.z)) - 1.0f) * 0.100000001490116119384765625f);
                    hit_cur = (h_cur < 0.0f) ? true : false;
                    if (hit_cur)
                    {
                        h_hit = h_cur;
                        t_hit = t_cur;
                        hit_fin = hit_cur;
                        break;
                    }
                    step_i++;
                    h_prev = h_cur;
                    hit_prev = hit_cur;
                    t_prev = t_cur;
                    continue;
                }
                else
                {
                    h_hit = 0.0f;
                    t_hit = 1.0f;
                    hit_fin = hit_prev;
                    break;
                }
            }
            h_lo = h_hit;
            h_hi = h_prev;
            t_hi = t_hit;
            hit_flag = hit_fin;
            t_lo = t_prev;
        }
        if (hit_flag)
        {
            float bs_lo;
            float bs_h;
            float bs_hh;
            float bs_t;
            bs_lo = t_lo;
            bs_h = h_lo;
            bs_hh = h_hi;
            bs_t = t_hi;
            for (uint bs_i = 0u; bs_i < 4u; )
            {
                float bs_mid_t = bs_lo + bs_t;
                float bs_mid = bs_mid_t * 0.5f;
                float3 bs_uv4 = local_o + (local_d * bs_mid);
                float2 bs_uv = bs_uv4.xy;
                float3 bs_h012 = (float3(stone_floor_height.SampleLevel(sampler_llr, bs_uv, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, bs_uv, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, bs_uv, stone_lod_bias).x) - 1.0f.xxx) + path_dn;
                float bs_hdiff = bs_uv4.z - ((max(max(path_h, bs_h012.x), max(bs_h012.y, bs_h012.z)) - 1.0f) * 0.100000001490116119384765625f);
                bool bs_hit = (bs_hdiff < 0.0f) ? true : false;
                bs_lo = bs_hit ? bs_lo : bs_mid;
                bs_h = bs_hit ? bs_hdiff : bs_h;
                bs_hh = bs_hit ? bs_hh : bs_hdiff;
                bs_t = bs_hit ? bs_mid : bs_t;
                bs_i++;
                continue;
            }
            float hit_t = lerp(bs_lo, bs_t, bs_hh / (bs_hh + (-bs_h)));
            float3 bs_uv4b = local_o + (local_d * hit_t);
            float2 bs_uvb = bs_uv4b.xy;
            float3 bs_h012b = (float3(stone_floor_height.SampleLevel(sampler_llr, bs_uvb, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, bs_uvb, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, bs_uvb, stone_lod_bias).x) - 1.0f.xxx) + path_dn;
            t_final = ((abs(bs_uv4b.z - ((max(max(path_h, bs_h012b.x), max(bs_h012b.y, bs_h012b.z)) - 1.0f) * 0.100000001490116119384765625f)) < (bs_hh * 0.89999997615814208984375f)) ? true : false) ? hit_t : bs_t;
            break;
        }
        else
        {
            t_final = t_lo;
            break;
        }
        break; // unreachable workaround
    } while(false);
    // 命中位移 → 最终石板采样 uv / 世界位置。
    float3 disp = local_d * t_final;
    float2 suv_final = stone_uv + disp.xy;
    float3 world_final = in_var_C1 + mul(uv_basis, disp);
    float4 clip_final = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(world_final, 1.0f)));
    // 选择石板法线层(高度最大的一层)。
    float3 tile_n;
    do
    {
        float3 h012c = (float3(stone_floor_height.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x) - 1.0f.xxx) + path_dn;
        float hc = h012c.x;
        float4 h4 = float4(path_h, hc, h012c.yz);
        float4 hmax4 = max(max(path_h, hc), max(h012c.y, h012c.z)).xxxx;
        float4 hsel = select(bool4(h4.x == hmax4.x, h4.y == hmax4.y, h4.z == hmax4.z, h4.w == hmax4.w), float4(1.0f, 1.0f, 1.0f, 1.0f), float4(0.0f, 0.0f, 0.0f, 0.0f));
        float3 ln2;
        if (hsel.y > 0.0f)
        {
            ln2 = stone_floor_normal.SampleLevel(sampler_llr, suv_final, stone_lod_bias).xyz;
        }
        else
        {
            float3 ln1;
            if (hsel.z > 0.0f)
            {
                ln1 = stone_floor_normal_2.SampleLevel(sampler_llr, suv_final, stone_lod_bias).xyz;
            }
            else
            {
                if (!(hsel.w > 0.0f))
                {
                    tile_n = float3(0.0f, 0.0f, 1.0f);
                    break;
                }
                ln1 = stone_floor_normal_3.SampleLevel(sampler_llr, suv_final, stone_lod_bias).xyz;
            }
            ln2 = ln1;
        }
        float3 ln_adj = (ln2 * 2.0f) - 1.0f.xxx;
        ln_adj.z = ln_adj.z * 0.0999999940395355224609375f;
        tile_n = ln_adj;
        break;
    } while(false);
    // 石板法线 → 世界空间。
    float3 world_n = normalize(mul(tile_n, tbn_inv));
    float2 world_xz = world_final.xz;
    float2 world_uv = (world_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    float3 path_yzw2 = paths_visual.SampleLevel(sampler_lnc, world_uv, 0.0f).yzw;
    float path_h2 = clamp(1.0f - dot(1.0f.xxx, (path_yzw2 * 2.0f) * path_alpha_inv), 0.0f, 1.0f);
    float3 path_d2 = path_yzw2 * float3(1.0f, 2.0f, 1.0f);
    float3 h012d = (float3(stone_floor_height.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x, stone_floor_height_2.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x, stone_floor_height_3.SampleLevel(sampler_llr, suv_final, stone_lod_bias).x) - 1.0f.xxx) + (path_d2 / max(0.00999999977648258209228515625f, max(path_d2.x, max(path_d2.y, path_d2.z))).xxx);
    float hd = h012d.x;
    float4 h4d = float4(path_h2, hd, h012d.yz);
    float4 hmax4d = max(max(path_h2, hd), max(h012d.y, h012d.z)).xxxx;
    float4 hseld = select(bool4(h4d.x == hmax4d.x, h4d.y == hmax4d.y, h4d.z == hmax4d.z, h4d.w == hmax4d.w), float4(1.0f, 1.0f, 1.0f, 1.0f), float4(0.0f, 0.0f, 0.0f, 0.0f));
    float4 path2 = paths_visual.SampleLevel(sampler_lnc, world_uv, 0.0f);
    float path2_x = path2.x;
    float path2_sqrt = pow(path2_x, 0.5f);
    float2 floor_uv = world_xz * 0.20000000298023223876953125f;
    float4 dirth_sample = dirthpath_heightmap.SampleBias(sampler_llr, floor_uv * 0.4000000059604644775390625f, frame_constants_global_mip_bias);
    float dirth_h = dirth_sample.x;
    float4 dirt1 = dirt_1.SampleBias(sampler_llr, floor_uv * 0.5f, frame_constants_global_mip_bias);
    float4 dirt2 = dirt_2.SampleBias(sampler_llr, floor_uv * 1.2000000476837158203125f, frame_constants_global_mip_bias);
    float4 grass_s = grass.SampleBias(sampler_llr, floor_uv * 0.800000011920928955078125f, frame_constants_global_mip_bias);
    float3 grass_s_col = grass_col * (grass_s.y * 5.5f);
    float3 base_col;
    if (is_theme4)
    {
        base_col = float3(0.374780178070068359375f, 0.3005877435207366943359375f, 0.2449434697628021240234375f);
    }
    else
    {
        float3 grass_theme3;
        if (frame_constants_glade_theme_id == 3u)
        {
            grass_theme3 = grass_s_col * 0.75f;
        }
        else
        {
            grass_theme3 = grass_s_col;
        }
        base_col = grass_theme3;
    }
    float path_amt = clamp(path2_sqrt * 0.60000002384185791015625f, 0.0f, 1.0f);
    float path_mix = 0.1500000059604644775390625f * (1.0f - path_amt);
    float dirth_amt = dirth_h * path_amt;
    float path_mk = max(path_mix, dirth_amt) - 0.100000001490116119384765625f;
    float path_mk2 = max(dirth_amt - path_mk, 0.0f);
    float3 path_w = (1.0f.xxx * path_mk2) / (max(path_mix - path_mk, 0.0f) + path_mk2).xxx;
    float path_w2 = path_w.x;
    float dirt_amt = clamp(pow(dirt_mask.SampleLevel(sampler_lnc, world_uv, 0.0f).x, 0.5f) * 0.60000002384185791015625f, 0.0f, 1.0f);
    float dirt_mix = 0.1500000059604644775390625f * (1.0f - dirt_amt);
    float dirt_amt2 = dirth_h * dirt_amt;
    float dirt_mk = max(dirt_mix, dirt_amt2) - 0.100000001490116119384765625f;
    float dirt_mk2 = max(dirt_amt2 - dirt_mk, 0.0f);
    float3 dirt_w = (1.0f.xxx * dirt_mk2) / (max(dirt_mix - dirt_mk, 0.0f) + dirt_mk2).xxx;
    float dirt_w2 = dirt_w.x;
    // 泥土/路径权重(主题 4 加噪声扰动)。
    float path_w3;
    float dirt_w3;
    if (is_theme4)
    {
        int2 th_cell = int2(floor(world_xz));
        float2 th_frac = frac(world_xz);
        uint2 v827 = uint2(th_cell);
        uint v829 = v827.y;
        uint v831 = v829 + (v829 << 10u);
        uint v833 = v831 ^ (v831 >> 6u);
        uint v835 = v833 + (v833 << 3u);
        uint v837 = v835 ^ (v835 >> 11u);
        uint v843 = (((v827.x * 1664525u) + (v837 + (v837 << 15u))) + 1013904223u) * 1664525u;
        uint v845 = v843 ^ (v843 >> 11u);
        uint v848 = v845 ^ ((v845 << 7u) & 2636928640u);
        uint th_h00 = v848 ^ ((v848 << 15u) & 4022730752u);
        uint2 v859 = uint2(th_cell + int2(1, 0));
        uint v861 = v859.y;
        uint v863 = v861 + (v861 << 10u);
        uint v865 = v863 ^ (v863 >> 6u);
        uint v867 = v865 + (v865 << 3u);
        uint v869 = v867 ^ (v867 >> 11u);
        uint v875 = (((v859.x * 1664525u) + (v869 + (v869 << 15u))) + 1013904223u) * 1664525u;
        uint v877 = v875 ^ (v875 >> 11u);
        uint v880 = v877 ^ ((v877 << 7u) & 2636928640u);
        uint th_h10 = v880 ^ ((v880 << 15u) & 4022730752u);
        uint2 v891 = uint2(th_cell + int2(0, 1));
        uint v893 = v891.y;
        uint v895 = v893 + (v893 << 10u);
        uint v897 = v895 ^ (v895 >> 6u);
        uint v899 = v897 + (v897 << 3u);
        uint v901 = v899 ^ (v899 >> 11u);
        uint v907 = (((v891.x * 1664525u) + (v901 + (v901 << 15u))) + 1013904223u) * 1664525u;
        uint v909 = v907 ^ (v907 >> 11u);
        uint v912 = v909 ^ ((v909 << 7u) & 2636928640u);
        uint th_h01 = v912 ^ ((v912 << 15u) & 4022730752u);
        uint2 v923 = uint2(th_cell + int2(1, 1));
        uint v925 = v923.y;
        uint v927 = v925 + (v925 << 10u);
        uint v929 = v927 ^ (v927 >> 6u);
        uint v931 = v929 + (v929 << 3u);
        uint v933 = v931 ^ (v931 >> 11u);
        uint v939 = (((v923.x * 1664525u) + (v933 + (v933 << 15u))) + 1013904223u) * 1664525u;
        uint v941 = v939 ^ (v939 >> 11u);
        uint v944 = v941 ^ ((v941 << 7u) & 2636928640u);
        uint th_h11 = v944 ^ ((v944 << 15u) & 4022730752u);
        float2 th_u = (th_frac * th_frac) * (3.0f.xx - (th_frac * 2.0f));
        float th_ux = th_u.x;
        float th_omux = 1.0f - th_ux;
        float th_uy = th_u.y;
        float th_noise = ((((1.0f - th_uy) * (((asfloat(((th_h00 ^ (th_h00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * th_omux) + ((asfloat(((th_h10 ^ (th_h10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * th_ux))) + (((asfloat(((th_h01 ^ (th_h01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * th_omux) * th_uy)) + (((asfloat(((th_h11 ^ (th_h11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * th_ux) * th_uy)) * 0.4000000059604644775390625f;
        path_w3 = path_w2 - th_noise;
        dirt_w3 = dirt_w2 - th_noise;
    }
    else
    {
        path_w3 = path_w2;
        dirt_w3 = dirt_w2;
    }
    bool theme4_carry = is_theme4 ? true : false;
    uint mask_hi = is_theme4 ? 1065353216u : v207;
    uint mask_lo = is_theme4 ? 8388607u : v207;
    float3 soil_col = lerp(lerp(base_col, dirt2.xyz * 1.10000002384185791015625f, smoothstep(0.20000000298023223876953125f, 0.64999997615814208984375f, dirt_w3).xxx), dirt1.xyz * 1.10000002384185791015625f, smoothstep(0.60000002384185791015625f, 1.0f, path_w3).xxx);
    float3 soil_col2;
    if (is_theme4)
    {
        soil_col2 = lerp(grass_col, soil_col, smoothstep(0.0f, 0.25f, path_w3).xxx);
    }
    else
    {
        soil_col2 = soil_col;
    }
    // 水/泥混合。
    float4 water_s = water_mask.SampleLevel(sampler_llc, world_uv, 0.0f);
    float wm_y = water_s.y;
    float4 mud_s = mud.SampleLevel(sampler_llr, world_uv * 10.0f, 0.0f);
    float3 mud_col = mud_s.xyz;
    float mud_edge = lerp(0.100000001490116119384765625f, 0.20000000298023223876953125f, smoothstep(0.0500000007450580596923828125f, 0.20000000298023223876953125f, mud_s.x));
    float mud_fall;
    do
    {
        float mud_diff = abs(wm_y - 0.4000000059604644775390625f);
        if (mud_diff > mud_edge)
        {
            mud_fall = 0.0f;
            break;
        }
        float mud_norm = mud_diff / mud_edge;
        mud_fall = 1.0f - ((mud_norm * mud_norm) * (3.0f - (2.0f * mud_norm)));
        break;
    } while(false);
    float rocky_z2;
    do
    {
        float rocky_x2 = world_uv.x;
        bool rocky_x_ok2;
        if (!(rocky_x2 < 0.0f))
        {
            rocky_x_ok2 = rocky_x2 > 1.0f;
        }
        else
        {
            rocky_x_ok2 = true;
        }
        bool rocky_y_low2;
        if (!rocky_x_ok2)
        {
            rocky_y_low2 = world_uv.y < 0.0f;
        }
        else
        {
            rocky_y_low2 = true;
        }
        bool rocky_y_high2;
        if (!rocky_y_low2)
        {
            rocky_y_high2 = world_uv.y > 1.0f;
        }
        else
        {
            rocky_y_high2 = true;
        }
        if (rocky_y_high2)
        {
            rocky_z2 = 0.0f;
            break;
        }
        else
        {
            rocky_z2 = rocky_terrain.SampleLevel(sampler_default, world_uv, 0.0f).z;
            break;
        }
        break; // unreachable workaround
    } while(false);
    // 石板砖块颜色:按 seed 贴图随机采样 brick_colors(三层,选高度最大层)。
    uint s_hi;
    uint s_lo;
    bool s_carry;
    float s_rough;
    float3 s_col;
    float s_h;
    if (hseld.y > 0.0f)
    {
        float4 seed1 = stone_floor_seed.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias);
        float seed1_z = seed1.z;
        int seed1_idx = int(trunc((seed1.x * 32.0f) - 0.5f) + trunc((seed1.y * 32.0f) - 0.5f));
        uint v1309 = uint(seed1_idx ^ 7993234);
        uint v1311 = v1309 + (v1309 << 10u);
        uint v1313 = v1311 ^ (v1311 >> 6u);
        uint v1315 = v1313 + (v1313 << 3u);
        uint v1317 = v1315 ^ (v1315 >> 11u);
        bool v1320 = theme4_carry ? theme4_carry : true;
        uint v1321 = theme4_carry ? mask_lo : 8388607u;
        uint v1322 = theme4_carry ? mask_hi : 1065353216u;
        uint v1329 = uint(seed1_idx ^ 9730541);
        uint v1331 = v1329 + (v1329 << 10u);
        uint v1333 = v1331 ^ (v1331 >> 6u);
        uint v1335 = v1333 + (v1333 << 3u);
        uint v1337 = v1335 ^ (v1335 >> 11u);
        uint v1339 = v1337 + (v1337 << 15u);
        bool v1340 = v1320 ? v1320 : true;
        uint v1341 = v1320 ? v1321 : 8388607u;
        uint v1342 = v1320 ? v1322 : 1065353216u;
        float4 rough1_s = stone_floor_roughness.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias);
        float rough1_x = rough1_s.x;
        uint v1381 = v1340 ? v1341 : 8388607u;
        uint v1382 = v1340 ? v1342 : 1065353216u;
        s_hi = v1382;
        s_lo = v1381;
        s_carry = v1340 ? v1340 : true;
        s_rough = lerp(0.75f, 1.0f, rough1_x) * lerp(0.800000011920928955078125f, 1.0f, asfloat((v1339 & v1381) | v1382) - 1.0f);
        s_col = ((((brick_colors.Sample(sampler_llr, float3((((world_xz * 0.100000001490116119384765625f) + float2((asfloat(((v1317 + (v1317 << 15u)) & v1321) | v1322) - 1.0f) * 0.300000011920928955078125f, (asfloat((v1339 & v1341) | v1342) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * seed1_z).xx) + (rough1_x * 0.100000001490116119384765625f).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, seed1_z)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
        s_h = smoothstep(0.20000000298023223876953125f, 0.75f, 1.0f - stone_floor_height.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x);
    }
    else
    {
        uint s2_hi;
        uint s2_lo;
        bool s2_carry;
        float s2_rough;
        float3 s2_col;
        float s2_h;
        if (hseld.z > 0.0f)
        {
            float4 seed2 = stone_floor_seed_2.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias);
            float seed2_z = seed2.z;
            int seed2_idx = int(trunc((seed2.x * 32.0f) - 0.5f) + trunc((seed2.y * 32.0f) - 0.5f));
            uint v1204 = uint(seed2_idx ^ 7993234);
            uint v1206 = v1204 + (v1204 << 10u);
            uint v1208 = v1206 ^ (v1206 >> 6u);
            uint v1210 = v1208 + (v1208 << 3u);
            uint v1212 = v1210 ^ (v1210 >> 11u);
            bool v1215 = theme4_carry ? theme4_carry : true;
            uint v1216 = theme4_carry ? mask_lo : 8388607u;
            uint v1217 = theme4_carry ? mask_hi : 1065353216u;
            uint v1224 = uint(seed2_idx ^ 9730541);
            uint v1226 = v1224 + (v1224 << 10u);
            uint v1228 = v1226 ^ (v1226 >> 6u);
            uint v1230 = v1228 + (v1228 << 3u);
            uint v1232 = v1230 ^ (v1230 >> 11u);
            uint v1234 = v1232 + (v1232 << 15u);
            bool v1235 = v1215 ? v1215 : true;
            uint v1236 = v1215 ? v1216 : 8388607u;
            uint v1237 = v1215 ? v1217 : 1065353216u;
            float4 rough2_s = stone_floor_roughness_2.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias);
            float rough2_x = rough2_s.x;
            uint v1276 = v1235 ? v1236 : 8388607u;
            uint v1277 = v1235 ? v1237 : 1065353216u;
            s2_hi = v1277;
            s2_lo = v1276;
            s2_carry = v1235 ? v1235 : true;
            s2_rough = lerp(0.75f, 1.0f, rough2_x) * lerp(0.800000011920928955078125f, 1.0f, asfloat((v1234 & v1276) | v1277) - 1.0f);
            s2_col = ((((brick_colors.Sample(sampler_llr, float3((((world_xz * 0.100000001490116119384765625f) + float2((asfloat(((v1212 + (v1212 << 15u)) & v1216) | v1217) - 1.0f) * 0.300000011920928955078125f, (asfloat((v1234 & v1236) | v1237) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * seed2_z).xx) + (rough2_x * 0.100000001490116119384765625f).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, seed2_z)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_2.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
            s2_h = smoothstep(0.300000011920928955078125f, 0.5f, 1.0f - stone_floor_height_2.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x);
        }
        else
        {
            uint s3_hi;
            uint s3_lo;
            bool s3_carry;
            float s3_rough;
            float3 s3_col;
            float s3_h;
            if (hseld.w > 0.0f)
            {
                float4 seed3 = stone_floor_seed_3.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias);
                float seed3_z = seed3.z;
                int seed3_idx = int(trunc((seed3.x * 32.0f) - 0.5f) + trunc((seed3.y * 32.0f) - 0.5f));
                uint v1102 = uint(seed3_idx ^ 7993234);
                uint v1104 = v1102 + (v1102 << 10u);
                uint v1106 = v1104 ^ (v1104 >> 6u);
                uint v1108 = v1106 + (v1106 << 3u);
                uint v1110 = v1108 ^ (v1108 >> 11u);
                bool v1113 = theme4_carry ? theme4_carry : true;
                uint v1114 = theme4_carry ? mask_lo : 8388607u;
                uint v1115 = theme4_carry ? mask_hi : 1065353216u;
                uint v1122 = uint(seed3_idx ^ 9730541);
                uint v1124 = v1122 + (v1122 << 10u);
                uint v1126 = v1124 ^ (v1124 >> 6u);
                uint v1128 = v1126 + (v1126 << 3u);
                uint v1130 = v1128 ^ (v1128 >> 11u);
                uint v1132 = v1130 + (v1130 << 15u);
                bool v1133 = v1113 ? v1113 : true;
                uint v1134 = v1113 ? v1114 : 8388607u;
                uint v1135 = v1113 ? v1115 : 1065353216u;
                uint v1171 = v1133 ? v1134 : 8388607u;
                uint v1172 = v1133 ? v1135 : 1065353216u;
                s3_hi = v1172;
                s3_lo = v1171;
                s3_carry = v1133 ? v1133 : true;
                s3_rough = lerp(0.75f, 1.0f, stone_floor_roughness_3.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat((v1132 & v1171) | v1172) - 1.0f);
                s3_col = ((((brick_colors.Sample(sampler_llr, float3(((world_xz * 0.100000001490116119384765625f) + float2((asfloat(((v1110 + (v1110 << 15u)) & v1114) | v1115) - 1.0f) * 0.300000011920928955078125f, (asfloat((v1132 & v1134) | v1135) - 1.0f) * 0.300000011920928955078125f)) + (0.5f * seed3_z).xx, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, seed3_z)) * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_3.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x)) * 0.85000002384185791015625f;
                s3_h = smoothstep(0.300000011920928955078125f, 1.0f, 1.0f - stone_floor_height_3.SampleBias(sampler_llr, suv_final, frame_constants_global_mip_bias).x);
            }
            else
            {
                s3_hi = mask_hi;
                s3_lo = mask_lo;
                s3_carry = theme4_carry;
                s3_rough = lerp(1.0f, 0.89999997615814208984375f, path_w3);
                s3_col = lerp(lerp(lerp(soil_col2, mud_col, smoothstep(0.4000000059604644775390625f, 0.5f, wm_y).xxx), mud_col * 0.75f, mud_fall.xxx), mud_col, (smoothstep(0.3499999940395355224609375f, 0.449999988079071044921875f, (garden_mask.SampleLevel(sampler_llc, world_uv, 0.0f).y - smoothstep(0.300000011920928955078125f, 0.5f, path2_sqrt)) - smoothstep(0.300000011920928955078125f, 0.5f, rocky_z2)) * (is_theme4 ? 0.60000002384185791015625f : 1.0f)).xxx);
                s3_h = 0.0f;
            }
            s2_hi = s3_hi;
            s2_lo = s3_lo;
            s2_carry = s3_carry;
            s2_rough = s3_rough;
            s2_col = s3_col;
            s2_h = s3_h;
        }
        s_hi = s2_hi;
        s_lo = s2_lo;
        s_carry = s2_carry;
        s_rough = s2_rough;
        s_col = s2_col;
        s_h = s2_h;
    }
    bool snow_flag;
    if (is_theme4)
    {
        snow_flag = in_var_C1.y > 0.0f;
    }
    else
    {
        snow_flag = false;
    }
    // 雪景分支:沿法线/建筑遮罩混雪。
    float3 final_n;
    float3 final_c;
    if (snow_flag)
    {
        float2 snow_uv = in_var_C1.xz * 5.0f;
        int2 snow_cell = int2(floor(snow_uv));
        float2 snow_frac = frac(snow_uv);
        uint2 v1431 = uint2(snow_cell);
        uint v1433 = v1431.y;
        uint v1435 = v1433 + (v1433 << 10u);
        uint v1437 = v1435 ^ (v1435 >> 6u);
        uint v1439 = v1437 + (v1437 << 3u);
        uint v1441 = v1439 ^ (v1439 >> 11u);
        bool v1444 = theme4_carry ? theme4_carry : true;
        uint v1445 = theme4_carry ? (is_theme4 ? 1664525u : v207) : 1664525u;
        uint v1446 = theme4_carry ? (is_theme4 ? 1013904223u : v207) : 1013904223u;
        uint v1450 = (((v1431.x * v1445) + (v1441 + (v1441 << 15u))) + v1446) * v1445;
        uint v1452 = v1450 ^ (v1450 >> 11u);
        uint v1455 = v1452 ^ ((v1452 << 7u) & 2636928640u);
        uint v1458 = v1455 ^ ((v1455 << 15u) & 4022730752u);
        bool v1461 = s_carry ? s_carry : true;
        uint v1462 = s_carry ? s_lo : 8388607u;
        uint v1463 = s_carry ? s_hi : 1065353216u;
        uint2 v1469 = uint2(snow_cell + int2(1, 0));
        uint v1471 = v1469.y;
        uint v1473 = v1471 + (v1471 << 10u);
        uint v1475 = v1473 ^ (v1473 >> 6u);
        uint v1477 = v1475 + (v1475 << 3u);
        uint v1479 = v1477 ^ (v1477 >> 11u);
        bool v1482 = v1444 ? v1444 : true;
        uint v1483 = v1444 ? v1445 : 1664525u;
        uint v1484 = v1444 ? v1446 : 1013904223u;
        uint v1488 = (((v1469.x * v1483) + (v1479 + (v1479 << 15u))) + v1484) * v1483;
        uint v1490 = v1488 ^ (v1488 >> 11u);
        uint v1493 = v1490 ^ ((v1490 << 7u) & 2636928640u);
        uint v1496 = v1493 ^ ((v1493 << 15u) & 4022730752u);
        bool v1499 = v1461 ? v1461 : true;
        uint v1500 = v1461 ? v1462 : 8388607u;
        uint v1501 = v1461 ? v1463 : 1065353216u;
        uint2 v1507 = uint2(snow_cell + int2(0, 1));
        uint v1509 = v1507.y;
        uint v1511 = v1509 + (v1509 << 10u);
        uint v1513 = v1511 ^ (v1511 >> 6u);
        uint v1515 = v1513 + (v1513 << 3u);
        uint v1517 = v1515 ^ (v1515 >> 11u);
        bool v1520 = v1482 ? v1482 : true;
        uint v1521 = v1482 ? v1483 : 1664525u;
        uint v1522 = v1482 ? v1484 : 1013904223u;
        uint v1526 = (((v1507.x * v1521) + (v1517 + (v1517 << 15u))) + v1522) * v1521;
        uint v1528 = v1526 ^ (v1526 >> 11u);
        uint v1531 = v1528 ^ ((v1528 << 7u) & 2636928640u);
        uint v1534 = v1531 ^ ((v1531 << 15u) & 4022730752u);
        bool v1537 = v1499 ? v1499 : true;
        uint v1538 = v1499 ? v1500 : 8388607u;
        uint v1539 = v1499 ? v1501 : 1065353216u;
        uint2 v1545 = uint2(snow_cell + int2(1, 1));
        uint v1547 = v1545.y;
        uint v1549 = v1547 + (v1547 << 10u);
        uint v1551 = v1549 ^ (v1549 >> 6u);
        uint v1553 = v1551 + (v1551 << 3u);
        uint v1555 = v1553 ^ (v1553 >> 11u);
        uint v1558 = v1520 ? v1521 : 1664525u;
        uint v1563 = (((v1545.x * v1558) + (v1555 + (v1555 << 15u))) + (v1520 ? v1522 : 1013904223u)) * v1558;
        uint v1565 = v1563 ^ (v1563 >> 11u);
        uint v1568 = v1565 ^ ((v1565 << 7u) & 2636928640u);
        uint v1571 = v1568 ^ ((v1568 << 15u) & 4022730752u);
        float2 snow_u = (snow_frac * snow_frac) * (3.0f.xx - (snow_frac * 2.0f));
        float snow_ux = snow_u.x;
        float snow_omux = 1.0f - snow_ux;
        float snow_uy = snow_u.y;
        float3 snow_amt = ((((clamp(smoothstep(0.20000000298023223876953125f, 1.0f, dirthpath_heightmap.SampleBias(sampler_llr, floor_uv * 0.1500000059604644775390625f, frame_constants_global_mip_bias).x * lerp(0.64999997615814208984375f, 1.0f, s_h)), 0.100000001490116119384765625f, lerp(0.5f, 0.20000000298023223876953125f, path2_x)) * clamp(world_n.y, 0.0f, 1.0f)) * (1.0f - (path2_x * 0.5f))) + lerp(0.0f, 0.20000000298023223876953125f, smoothstep(0.0f, 0.4000000059604644775390625f, s_h))) * (1.0f - smoothstep(0.0f, 0.300000011920928955078125f, buildings_mask.SampleLevel(sampler_llc, uv0, 0.0f).y - ((((((1.0f - snow_uy) * (((asfloat(((v1458 ^ (v1458 >> 18u)) & v1462) | v1463) - 1.0f) * snow_omux) + ((asfloat(((v1496 ^ (v1496 >> 18u)) & v1500) | v1501) - 1.0f) * snow_ux))) + (((asfloat(((v1534 ^ (v1534 >> 18u)) & v1538) | v1539) - 1.0f) * snow_omux) * snow_uy)) + (((asfloat(((v1571 ^ (v1571 >> 18u)) & (v1537 ? v1538 : 8388607u)) | (v1537 ? v1539 : 1065353216u)) - 1.0f) * snow_ux) * snow_uy)) * 0.100000001490116119384765625f) * s_h)))).xxx;
        final_n = normalize(lerp(world_n, in_var_C0, snow_amt));
        final_c = lerp(s_col, 0.699999988079071044921875f.xxx, snow_amt);
    }
    else
    {
        final_n = world_n;
        final_c = s_col;
    }
    // 输出:打包法线/反照率到 uint2 gbuffer,并写深度。
    float3 packed_n = (final_n * 0.5f) + 0.5f.xxx;
    float3 albedo_sqrt = sqrt(clamp(final_c, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer = uint2(((uint((clamp(packed_n.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_n.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_n.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(s_rough, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), albedo_word);
    gbuffer.y = albedo_word | 905969664u;
    out_var_SV_TARGET0 = gbuffer;
    gl_FragDepth = (clip_final.xyz / clip_final.w.xxx).z;
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
// 提升段:TGV_f0adff76827e73ff 变体,顶点着色器(与 b903 相同,行为等价,语义重命名)。
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
    uint current_view_index : packoffset(c0);
    uint2 buildings_mask_dims : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 rocky_terrain_dims : packoffset(c2);
    uint2 paths_visual_dims : packoffset(c2.z);
    uint2 water_mask_dims : packoffset(c3);
    uint2 highlight_active_hill_dims : packoffset(c3.z);
    uint2 mud_dims : packoffset(c4);
    uint2 noise_debug_dims : packoffset(c4.z);
    uint2 floor_heightmap_dims : packoffset(c5);
    uint2 dirthpath_heightmap_dims : packoffset(c5.z);
    uint2 dirt_1_dims : packoffset(c6);
    uint2 dirt_2_dims : packoffset(c6.z);
    uint2 grass_dims : packoffset(c7);
    uint2 dirt_mask_dims : packoffset(c7.z);
    uint2 stone_floor_normal_dims : packoffset(c8);
    uint2 stone_floor_seed_dims : packoffset(c8.z);
    uint2 stone_floor_height_dims : packoffset(c9);
    uint2 stone_floor_roughness_dims : packoffset(c9.z);
    uint2 stone_floor_seed_2_dims : packoffset(c10);
    uint2 stone_floor_normal_2_dims : packoffset(c10.z);
    uint2 stone_floor_height_2_dims : packoffset(c11);
    uint2 stone_floor_roughness_2_dims : packoffset(c11.z);
    uint2 stone_floor_seed_3_dims : packoffset(c12);
    uint2 stone_floor_normal_3_dims : packoffset(c12.z);
    uint2 stone_floor_height_3_dims : packoffset(c13);
    uint2 stone_floor_roughness_3_dims : packoffset(c13.z);
    uint2 garden_mask_dims : packoffset(c14);
    uint force_top_down_render : packoffset(c14.z);
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
    // 91x91 地形网格顶点位置。
    float2 uv;
    float uv_x;
    bool uv_x_ok;
    float2 grid_pos = ((((float2(int2(float2(float(uint(gl_VertexIndex) % 91u), float(uint(gl_VertexIndex) / 91u)))) + 0.5f.xx) * 0.010989011265337467193603515625f.xx) * 2.0f) - 1.0f.xx) * 65.72222137451171875f.xx;
    // 地形高度采样,映射到 [-2.5, 24]。
    float3 terrain_pos = float3(grid_pos.x, 0.0f, grid_pos.y);
    terrain_pos.y = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (terrain_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
    float rocky_x;
    do
    {
        uv = (terrain_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        uv_x = uv.x;
        uv_x_ok = !(uv_x < 0.0f);
        bool uv_x_high;
        if (uv_x_ok)
        {
            uv_x_high = uv_x > 1.0f;
        }
        else
        {
            uv_x_high = true;
        }
        bool uv_y_low;
        if (!uv_x_high)
        {
            uv_y_low = uv.y < 0.0f;
        }
        else
        {
            uv_y_low = true;
        }
        bool uv_y_high;
        if (!uv_y_low)
        {
            uv_y_high = uv.y > 1.0f;
        }
        else
        {
            uv_y_high = true;
        }
        if (uv_y_high)
        {
            rocky_x = 0.0f;
            break;
        }
        else
        {
            rocky_x = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f).x;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float rocky_z;
    do
    {
        bool uv_x_high2;
        if (uv_x_ok)
        {
            uv_x_high2 = uv_x > 1.0f;
        }
        else
        {
            uv_x_high2 = true;
        }
        bool uv_y_low2;
        if (!uv_x_high2)
        {
            uv_y_low2 = uv.y < 0.0f;
        }
        else
        {
            uv_y_low2 = true;
        }
        bool uv_y_high2;
        if (!uv_y_low2)
        {
            uv_y_high2 = uv.y > 1.0f;
        }
        else
        {
            uv_y_high2 = true;
        }
        if (uv_y_high2)
        {
            rocky_z = 0.0f;
            break;
        }
        else
        {
            rocky_z = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f).z;
            break;
        }
        break; // unreachable workaround
    } while(false);
    // GatherRed:中心差分计算地形法线。
    float4 gather = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float2 texel_scale = 130.0f.xx / float2(terrain_heightmap_dims);
    float v207 = gather.z;
    float g_y = gather.y;
    float g_w = gather.w;
    float g_x = gather.x;
    float2 grad = ((float2((v207 + g_y) - (g_w + g_x), (g_x + g_y) - (g_w + v207)) * 0.5f.xx) * 26.5f) / texel_scale;
    // 地形法线(由梯度叉积)。
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    float4 gather2 = terrain_heightmap.GatherRed(sampler_default, uv, int2(0, 0));
    float v233 = gather2.z;
    float g2_y = gather2.y;
    float g2_w = gather2.w;
    float g2_x = gather2.x;
    float v247 = length(((float2((v233 + g2_y) - (g2_w + g2_x), (g2_x + g2_y) - (g2_w + v233)) * 0.5f.xx) * 26.5f) / texel_scale);
    // 岩质地表高度混合(沿法线下压)。
    float3 displ_pos;
    if (v247 < 2.0f)
    {
        displ_pos = terrain_pos - (normal * lerp(0.0f, 0.5f, clamp(rocky_x + rocky_z, 0.0f, 1.0f)));
    }
    else
    {
        displ_pos = terrain_pos;
    }
    // 水面下陷:沿水遮罩把地面压到 -0.8。
    float water_y = lerp(displ_pos.y, -0.800000011920928955078125f, smoothstep(0.20000000298023223876953125f, 0.60000002384185791015625f, water_mask.SampleLevel(sampler_llc, (displ_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, v247));
    float3 v267 = displ_pos;
    v267.y = water_y;
    float4 pos4 = float4(displ_pos.x, water_y, displ_pos.z, 1.0f);
    float4 clip_pos;
    if (force_top_down_render != 0u)
    {
        clip_pos = float4(pos4.xz * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), clamp((water_y - (-2.5f)) * 0.0377358496189117431640625f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, pos4);
    }
    // 输出:顶点位置 + 法线(C0)/世界坐标(C1)。
    gl_Position = clip_pos;
    out_var_C0 = normal;
    out_var_C1 = v267;
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
// 提升段:TGV_f0adff76827e73ff 变体,像素着色器(空 frag_main,仅入口占位)。
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
