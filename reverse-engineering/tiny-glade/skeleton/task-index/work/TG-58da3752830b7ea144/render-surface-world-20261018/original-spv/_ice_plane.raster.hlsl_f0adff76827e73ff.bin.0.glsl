#version 450
#extension GL_EXT_nonuniform_qualifier : require

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

layout(set = 0, binding = 13, std140) uniform type_Globals
{
    uint current_view_index;
    uint history_valid;
    uvec2 blue_noise_texture_dims;
    uvec2 framebuffer_dims;
    uvec2 water_mask_dims;
    uvec2 half_depth_tex_dims;
    uvec2 refraction_tex_dims;
    uvec2 reprojection_tex_dims;
    uvec2 shadows_history_tex_dims;
    uvec2 surface_roughness_tex_dims;
    layout(row_major) mat4 model_to_world;
} _Globals;

layout(location = 0) in vec3 in_var_Vertex_Position;
layout(location = 0) out vec3 out_var_C0;
layout(location = 1) out vec3 out_var_C1;

void main()
{
    vec4 _78 = vec4(in_var_Vertex_Position, 1.0) * _Globals.model_to_world;
    vec2 _80 = _78.xz * vec2(130.0);
    vec4 _81 = vec4(_80.x, _78.y, _80.y, _78.w);
    _81.y = 0.0;
    gl_Position = _81 * frame_constants.view_constants[_Globals.current_view_index].world_to_sample;
    out_var_C0 = vec3(0.0, 1.0, 0.0);
    out_var_C1 = _81.xyz;
}

