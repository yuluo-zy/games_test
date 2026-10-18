#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_samplerless_texture_functions : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

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

const uvec2 _92[4] = uvec2[](uvec2(0u), uvec2(0u, 1u), uvec2(1u), uvec2(1u, 0u));

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

layout(set = 0, binding = 26) uniform texture2D depth_tex;
layout(set = 0, binding = 33) uniform texture2D candidate_radiance_tex;
layout(set = 0, binding = 34) uniform texture2D candidate_hit_tex;
layout(set = 0, binding = 35, rgba16f) uniform writeonly image2D restir_radiance_out_tex;
layout(set = 0, binding = 36, rgba16f) uniform writeonly image2D restir_ray_dir_out_tex;
layout(set = 0, binding = 37, rg32ui) uniform writeonly uimage2D restir_reservoir_out_tex;

void main()
{
    imageStore(restir_radiance_out_tex, ivec2(gl_GlobalInvocationID.xy), texelFetch(candidate_radiance_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)));
    imageStore(restir_ray_dir_out_tex, ivec2(gl_GlobalInvocationID.xy), vec4(texelFetch(candidate_hit_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).xyz, texelFetch(depth_tex, ivec2((gl_GlobalInvocationID.xy * uvec2(2u)) + _92[frame_constants.frame_index & 3u]), int(0u)).x));
    uint _121 = packHalf2x16(vec2(1.0, 0.0));
    imageStore(restir_reservoir_out_tex, ivec2(gl_GlobalInvocationID.xy), uvec2(gl_GlobalInvocationID.x | (gl_GlobalInvocationID.y << 16u), _121 | (_121 << 16u)).xyyy);
}

