#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
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
    layout(offset = 0) uint history_valid;
    layout(offset = 4) uvec2 input_tex_dims;
    layout(offset = 16) uvec2 history_tex_dims;
    layout(offset = 24) uvec2 reprojection_tex_dims;
    layout(offset = 32) uvec2 final_output_tex_dims;
    layout(offset = 40) uvec2 history_output_tex_dims;
} _Globals;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 14) uniform texture2D input_tex;
layout(set = 0, binding = 15) uniform texture2D history_tex;
layout(set = 0, binding = 16) uniform texture2D reprojection_tex;
layout(set = 0, binding = 17, r8) uniform writeonly image2D final_output_tex;
layout(set = 0, binding = 18, r8) uniform writeonly image2D history_output_tex;

spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    bool _87 = _Globals.history_valid != 0u;
    bool _94;
    if (_87)
    {
        _94 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _94 = false;
    }
    vec4 _201;
    if (_94)
    {
        vec4 _106 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
        bool _115;
        if (_87)
        {
            _115 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _115 = false;
        }
        vec4 _125;
        if (_115)
        {
            _125 = textureLod(sampler2D(history_tex, sampler_lnc), ((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / vec2(_Globals.input_tex_dims)) + texelFetch(reprojection_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).xy, 0.0);
        }
        else
        {
            _125 = _106;
        }
        vec4 _127 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(4294967294u)), int(0u));
        vec4 _132 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(0u, 4294967294u)), int(0u));
        vec4 _139 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(2u, 4294967294u)), int(0u));
        vec4 _146 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(4294967294u, 0u)), int(0u));
        vec4 _152 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
        vec4 _159 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(2u, 0u)), int(0u));
        vec4 _166 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(4294967294u, 2u)), int(0u));
        vec4 _173 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(0u, 2u)), int(0u));
        vec4 _180 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(2u)), int(0u));
        vec4 _186 = (((((((((_127 * 0.223130166530609130859375) + (_132 * 0.4723665416240692138671875)) + (_139 * 0.223130166530609130859375)) + (_146 * 0.4723665416240692138671875)) + (_152 * 1.0)) + (_159 * 0.4723665416240692138671875)) + (_166 * 0.223130166530609130859375)) + (_173 * 0.4723665416240692138671875)) + (_180 * 0.223130166530609130859375)) * vec4(0.264411270618438720703125);
        vec4 _192 = mix(_106, _186, vec4(0.25));
        vec4 _194 = (sqrt(spvNMax(vec4(0.0), (((((((((((_127 * _127) * 0.223130166530609130859375) + ((_132 * _132) * 0.4723665416240692138671875)) + ((_139 * _139) * 0.223130166530609130859375)) + ((_146 * _146) * 0.4723665416240692138671875)) + ((_152 * _152) * 1.0)) + ((_159 * _159) * 0.4723665416240692138671875)) + ((_166 * _166) * 0.223130166530609130859375)) + ((_173 * _173) * 0.4723665416240692138671875)) + ((_180 * _180) * 0.223130166530609130859375)) * vec4(0.264411270618438720703125)) - (_186 * _186))) * 0.5) * 5.0;
        _201 = mix(clamp(_125, _192 - _194, _192 + _194), _106, vec4(0.125));
    }
    else
    {
        _201 = texelFetch(input_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
    }
    vec4 _203 = vec4(_201.x);
    imageStore(history_output_tex, ivec2(gl_GlobalInvocationID.xy), _203);
    imageStore(final_output_tex, ivec2(gl_GlobalInvocationID.xy), _203);
}

