#define SHADER_VARIANT_DEPTH_ONLY 1
#define TGV_f0adff76827e73ff 1
#line 1 "shaders/instanced_roof_beams.raster.hlsl"




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

struct Affine3Packed
{
    float data[12];
};

struct RoofSupportBeam
{
    Affine3Packed transform;
    float4 circle_normal;
    float roof_profile;
    int pad0;
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

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
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
    Affine3Packed _123;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _123.data[_2ident] = instanced_data.Load<float>(_2ident * 4 + uint(gl_InstanceIndex) * 80 + 0);
    }
    float3x4 _142 = transpose(float4x3(float3(_123.data[0], _123.data[1], _123.data[2]), float3(_123.data[3], _123.data[4], _123.data[5]), float3(_123.data[6], _123.data[7], _123.data[8]), float3(_123.data[9], _123.data[10], _123.data[11])));
    float3 _155 = mul(_142, float4(in_var_Vertex_Position, 1.0f)) + (instanced_data.Load<float4>(uint(gl_InstanceIndex) * 80 + 48).xyz * lerp(0.20000000298023223876953125f, -0.1500000059604644775390625f, instanced_data.Load<float>(uint(gl_InstanceIndex) * 80 + 64)));
    float2 _157 = _155.xz * 0.5f;
    int2 _159 = int2(floor(_157));
    float2 _160 = frac(_157);
    uint2 _161 = uint2(_159);
    uint _163 = _161.y;
    uint _165 = _163 + (_163 << 10u);
    uint _167 = _165 ^ (_165 >> 6u);
    uint _169 = _167 + (_167 << 3u);
    uint _171 = _169 ^ (_169 >> 11u);
    uint _177 = (((_161.x * 1664525u) + (_171 + (_171 << 15u))) + 1013904223u) * 1664525u;
    uint _179 = _177 ^ (_177 >> 11u);
    uint _182 = _179 ^ ((_179 << 7u) & 2636928640u);
    uint _185 = _182 ^ ((_182 << 15u) & 4022730752u);
    uint2 _193 = uint2(_159 + int2(1, 0));
    uint _195 = _193.y;
    uint _197 = _195 + (_195 << 10u);
    uint _199 = _197 ^ (_197 >> 6u);
    uint _201 = _199 + (_199 << 3u);
    uint _203 = _201 ^ (_201 >> 11u);
    uint _209 = (((_193.x * 1664525u) + (_203 + (_203 << 15u))) + 1013904223u) * 1664525u;
    uint _211 = _209 ^ (_209 >> 11u);
    uint _214 = _211 ^ ((_211 << 7u) & 2636928640u);
    uint _217 = _214 ^ ((_214 << 15u) & 4022730752u);
    uint2 _225 = uint2(_159 + int2(0, 1));
    uint _227 = _225.y;
    uint _229 = _227 + (_227 << 10u);
    uint _231 = _229 ^ (_229 >> 6u);
    uint _233 = _231 + (_231 << 3u);
    uint _235 = _233 ^ (_233 >> 11u);
    uint _241 = (((_225.x * 1664525u) + (_235 + (_235 << 15u))) + 1013904223u) * 1664525u;
    uint _243 = _241 ^ (_241 >> 11u);
    uint _246 = _243 ^ ((_243 << 7u) & 2636928640u);
    uint _249 = _246 ^ ((_246 << 15u) & 4022730752u);
    uint2 _257 = uint2(_159 + int2(1, 1));
    uint _259 = _257.y;
    uint _261 = _259 + (_259 << 10u);
    uint _263 = _261 ^ (_261 >> 6u);
    uint _265 = _263 + (_263 << 3u);
    uint _267 = _265 ^ (_265 >> 11u);
    uint _273 = (((_257.x * 1664525u) + (_267 + (_267 << 15u))) + 1013904223u) * 1664525u;
    uint _275 = _273 ^ (_273 >> 11u);
    uint _278 = _275 ^ ((_275 << 7u) & 2636928640u);
    uint _281 = _278 ^ ((_278 << 15u) & 4022730752u);
    float2 _291 = (_160 * _160) * (3.0f.xx - (_160 * 2.0f));
    float _292 = _291.x;
    float _293 = 1.0f - _292;
    float _295 = _291.y;
    float _308 = _155.y + (((((1.0f - _295) * (((asfloat(((_185 ^ (_185 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _293) + ((asfloat(((_217 ^ (_217 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _292))) + (((asfloat(((_249 ^ (_249 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _293) * _295)) + (((asfloat(((_281 ^ (_281 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _292) * _295)) * 0.300000011920928955078125f);
    float3 _309 = _155;
    _309.y = _308;
    float3x3 _332 = float3x3(_142[0].xyz, _142[1].xyz, _142[2].xyz);
    float3 _333 = mul(_332, float3(1.0f, 0.0f, 0.0f));
    float3 _334 = mul(_332, float3(0.0f, 1.0f, 0.0f));
    float3 _335 = mul(_332, float3(0.0f, 0.0f, 1.0f));
    float3 _338 = cross(_333, _334);
    float3 _341 = (1.0f / dot(_335, _338)).xxx;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_155.x, _308, _155.z, 1.0f));
    out_var_C0 = lerp(color_library_fence_color_2.xyz, color_library_fence_color_1.xyz, 0.20000000298023223876953125f.xxx) * float3(0.719999969005584716796875f, 0.647999942302703857421875f, 0.5759999752044677734375f);
    out_var_C1 = mul(in_var_Vertex_Normal, float3x3(cross(_334, _335) * _341, cross(_335, _333) * _341, _338 * _341));
    out_var_C2 = _309;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)

#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C0;
static float3 in_var_C1;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0: Vertex_Position;
    [[vk::location(1)]] float3 in_var_C1: Vertex_Normal;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float3 _56 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _78 = sqrt(clamp(in_var_C0, 0.0f.xxx, 1.0f.xxx));
    uint _97 = (uint((clamp(_78.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_78.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_78.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _98 = uint2(((uint((clamp(_56.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_56.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_56.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3858759680u, _97);
    _98.y = _97 | 117440512u;
    out_var_SV_TARGET0 = _98;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
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

struct Affine3Packed
{
    float data[12];
};

struct RoofSupportBeam
{
    Affine3Packed transform;
    float4 circle_normal;
    float roof_profile;
    int pad0;
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

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
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
    Affine3Packed _123;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _123.data[_2ident] = instanced_data.Load<float>(_2ident * 4 + uint(gl_InstanceIndex) * 80 + 0);
    }
    float3x4 _142 = transpose(float4x3(float3(_123.data[0], _123.data[1], _123.data[2]), float3(_123.data[3], _123.data[4], _123.data[5]), float3(_123.data[6], _123.data[7], _123.data[8]), float3(_123.data[9], _123.data[10], _123.data[11])));
    float3 _155 = mul(_142, float4(in_var_Vertex_Position, 1.0f)) + (instanced_data.Load<float4>(uint(gl_InstanceIndex) * 80 + 48).xyz * lerp(0.20000000298023223876953125f, -0.1500000059604644775390625f, instanced_data.Load<float>(uint(gl_InstanceIndex) * 80 + 64)));
    float2 _157 = _155.xz * 0.5f;
    int2 _159 = int2(floor(_157));
    float2 _160 = frac(_157);
    uint2 _161 = uint2(_159);
    uint _163 = _161.y;
    uint _165 = _163 + (_163 << 10u);
    uint _167 = _165 ^ (_165 >> 6u);
    uint _169 = _167 + (_167 << 3u);
    uint _171 = _169 ^ (_169 >> 11u);
    uint _177 = (((_161.x * 1664525u) + (_171 + (_171 << 15u))) + 1013904223u) * 1664525u;
    uint _179 = _177 ^ (_177 >> 11u);
    uint _182 = _179 ^ ((_179 << 7u) & 2636928640u);
    uint _185 = _182 ^ ((_182 << 15u) & 4022730752u);
    uint2 _193 = uint2(_159 + int2(1, 0));
    uint _195 = _193.y;
    uint _197 = _195 + (_195 << 10u);
    uint _199 = _197 ^ (_197 >> 6u);
    uint _201 = _199 + (_199 << 3u);
    uint _203 = _201 ^ (_201 >> 11u);
    uint _209 = (((_193.x * 1664525u) + (_203 + (_203 << 15u))) + 1013904223u) * 1664525u;
    uint _211 = _209 ^ (_209 >> 11u);
    uint _214 = _211 ^ ((_211 << 7u) & 2636928640u);
    uint _217 = _214 ^ ((_214 << 15u) & 4022730752u);
    uint2 _225 = uint2(_159 + int2(0, 1));
    uint _227 = _225.y;
    uint _229 = _227 + (_227 << 10u);
    uint _231 = _229 ^ (_229 >> 6u);
    uint _233 = _231 + (_231 << 3u);
    uint _235 = _233 ^ (_233 >> 11u);
    uint _241 = (((_225.x * 1664525u) + (_235 + (_235 << 15u))) + 1013904223u) * 1664525u;
    uint _243 = _241 ^ (_241 >> 11u);
    uint _246 = _243 ^ ((_243 << 7u) & 2636928640u);
    uint _249 = _246 ^ ((_246 << 15u) & 4022730752u);
    uint2 _257 = uint2(_159 + int2(1, 1));
    uint _259 = _257.y;
    uint _261 = _259 + (_259 << 10u);
    uint _263 = _261 ^ (_261 >> 6u);
    uint _265 = _263 + (_263 << 3u);
    uint _267 = _265 ^ (_265 >> 11u);
    uint _273 = (((_257.x * 1664525u) + (_267 + (_267 << 15u))) + 1013904223u) * 1664525u;
    uint _275 = _273 ^ (_273 >> 11u);
    uint _278 = _275 ^ ((_275 << 7u) & 2636928640u);
    uint _281 = _278 ^ ((_278 << 15u) & 4022730752u);
    float2 _291 = (_160 * _160) * (3.0f.xx - (_160 * 2.0f));
    float _292 = _291.x;
    float _293 = 1.0f - _292;
    float _295 = _291.y;
    float _308 = _155.y + (((((1.0f - _295) * (((asfloat(((_185 ^ (_185 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _293) + ((asfloat(((_217 ^ (_217 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _292))) + (((asfloat(((_249 ^ (_249 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _293) * _295)) + (((asfloat(((_281 ^ (_281 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _292) * _295)) * 0.300000011920928955078125f);
    float3 _309 = _155;
    _309.y = _308;
    float3x3 _332 = float3x3(_142[0].xyz, _142[1].xyz, _142[2].xyz);
    float3 _333 = mul(_332, float3(1.0f, 0.0f, 0.0f));
    float3 _334 = mul(_332, float3(0.0f, 1.0f, 0.0f));
    float3 _335 = mul(_332, float3(0.0f, 0.0f, 1.0f));
    float3 _338 = cross(_333, _334);
    float3 _341 = (1.0f / dot(_335, _338)).xxx;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_155.x, _308, _155.z, 1.0f));
    out_var_C0 = lerp(color_library_fence_color_2.xyz, color_library_fence_color_1.xyz, 0.20000000298023223876953125f.xxx) * float3(0.719999969005584716796875f, 0.647999942302703857421875f, 0.5759999752044677734375f);
    out_var_C1 = mul(in_var_Vertex_Normal, float3x3(cross(_334, _335) * _341, cross(_335, _333) * _341, _338 * _341));
    out_var_C2 = _309;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
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
