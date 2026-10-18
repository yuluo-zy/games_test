#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require

struct ViewConstants
{
    mat4 view_to_clip;
    mat4 clip_to_view;
    mat4 view_to_sample;
    mat4 sample_to_view;
    mat4 world_to_view;
    mat4 view_to_world;
    mat4 world_to_sample;
    mat4 clip_to_prev_clip;
    mat4 prev_view_to_prev_clip;
    mat4 prev_clip_to_prev_view;
    mat4 prev_world_to_prev_view;
    mat4 prev_view_to_prev_world;
    vec2 sample_offset_pixels;
    vec2 sample_offset_clip;
    vec2 prev_sample_offset_pixels;
    vec2 prev_sample_offset_clip;
};

layout(set = 1, binding = 0, std140) uniform type_ConstantBuffer_FrameConstants
{
    vec4 sun_color;
    vec4 sky_low_color;
    vec4 sky_high_color;
    vec4 fog_tint_thickness;
    vec4 fake_light_color;
    vec4 fake_light_dir;
    vec4 fake_light2_color;
    vec4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uvec2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    vec3 editing_highlight_color;
    float time_of_day;
    vec3 dir_to_sun;
    uint camera_idx;
    vec3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    vec4 debug_tweakers;
    layout(row_major) ViewConstants view_constants[5];
} frame_constants;

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    layout(offset = 0) uint current_view_index;
    layout(row_major, offset = 16) mat4 model_to_world;
    layout(offset = 80) uvec2 terrain_heightmap_dims;
    layout(offset = 88) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 96) uint history_valid;
    layout(offset = 100) uvec2 terrain_lighting_texture_dims;
    layout(offset = 112) uvec2 blue_noise_texture_dims;
    layout(offset = 120) uvec2 brdf_fg_lut_dims;
    layout(offset = 128) uint rt_bvh_node_count;
    layout(offset = 132) uint rt_triangle_count;
    layout(offset = 136) uvec2 water_noise_texture_dims;
    layout(offset = 144) uvec2 framebuffer_dims;
    layout(offset = 152) uvec2 water_mask_dims;
    layout(offset = 160) uvec2 half_depth_tex_dims;
    layout(offset = 168) uvec2 refraction_tex_dims;
    layout(offset = 176) uvec2 taa_history_tex_dims;
    layout(offset = 184) uvec2 reprojection_tex_dims;
    layout(offset = 192) vec3 water_color;
} _Globals;

layout(location = 0) in vec3 in_var_Vertex_Position;
layout(location = 0) out vec3 out_var_C0;
layout(location = 1) out vec3 out_var_C1;

void main()
{
    vec4 _92 = vec4(in_var_Vertex_Position, 1.0) * _Globals.model_to_world;
    vec2 _94 = _92.xz * vec2(130.0);
    vec4 _95 = vec4(_94.x, _92.y, _94.y, _92.w);
    _95.y = -0.00999999977648258209228515625;
    gl_Position = _95 * frame_constants.view_constants[_Globals.current_view_index].world_to_sample;
    out_var_C0 = vec3(0.0, 1.0, 0.0);
    out_var_C1 = _95.xyz;
}

