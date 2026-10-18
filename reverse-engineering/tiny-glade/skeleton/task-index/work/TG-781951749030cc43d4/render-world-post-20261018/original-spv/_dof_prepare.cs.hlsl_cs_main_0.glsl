#version 450
#extension GL_EXT_nonuniform_qualifier : require
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

struct DofFocus
{
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    vec3 lock_pos_ws;
    uint is_locked;
    vec3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
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
    uvec2 depth_tex_dims;
    uvec2 coc_out_tex_dims;
    uvec2 tile_output_tex_dims;
} _Globals;

layout(set = 0, binding = 15, std430) readonly buffer type_StructuredBuffer_DofFocus
{
    DofFocus _m0[];
} autofocus_buf;

layout(set = 0, binding = 14) uniform texture2D depth_tex;
layout(set = 0, binding = 16, r16f) uniform writeonly image2D coc_out_tex;
layout(set = 0, binding = 17, rg16f) uniform writeonly image2D tile_output_tex;

shared uint min_abs_coc_asuint;
shared uint max_abs_coc_asuint;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);
spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    bool _97 = 0u == gl_LocalInvocationIndex;
    if (_97)
    {
        min_abs_coc_asuint = 1343554297u;
        max_abs_coc_asuint = 0u;
    }
    barrier();
    float _102 = float(_Globals.depth_tex_dims.y);
    vec4 _124 = (vec4((((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / vec2(_Globals.coc_out_tex_dims)) - vec2(0.5)) * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec3 _128 = _124.xyz / vec3(_124.w);
    float _138 = autofocus_buf._m0[frame_constants.camera_idx].focus_scale;
    float _139 = 1.0 / autofocus_buf._m0[frame_constants.camera_idx].focus0;
    float _140 = 1.0 / autofocus_buf._m0[frame_constants.camera_idx].focus1;
    float _143 = spvNMax(0.001000000047497451305389404296875, spvNMax(_139, _140)) + 9.9999997473787516355514526367188e-06;
    float _146 = spvNMax(0.001000000047497451305389404296875, spvNMin(_139, _140)) - 9.9999997473787516355514526367188e-06;
    bool _147 = autofocus_buf._m0[frame_constants.camera_idx].use_tilt_shift != 0u;
    _128.y = _128.y * float(_147 ? 0 : 1);
    vec4 _153 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
    vec3 _161 = normalize((vec4(0.0, 0.0, -1.0, 0.0) * frame_constants.view_constants[0u].view_to_world).xyz);
    float _164 = 1.0 / spvNMax(1.0000000133514319600180897396058e-10, dot(_128 - (_153.xyz / vec3(_153.w)), _161));
    float _165 = _143 - _146;
    float _167 = spvNMin(0.00999999977648258209228515625, 0.0500000007450580596923828125 * _165);
    float _187;
    if (_164 > _143)
    {
        _187 = (_164 - _143) + _167;
    }
    else
    {
        float _186;
        if (_164 < _146)
        {
            _186 = (_164 - _146) - _167;
        }
        else
        {
            _186 = (-_167) + (((2.0 * _167) * (_164 - _146)) / _165);
        }
        _187 = _186;
    }
    float _194;
    if (_147)
    {
        _194 = _187 / spvNMax(0.00999999977648258209228515625, length(_161.xz));
    }
    else
    {
        _194 = _187;
    }
    float _195 = -_194;
    float _201 = (_102 * 0.001388888922519981861114501953125) * ((_195 * _138) * frame_constants.view_constants[0u].view_to_clip[1].y);
    uint _205 = floatBitsToUint(spvNMin(_102 * 0.0416666679084300994873046875, abs(_201)));
    uint _206 = atomicMin(min_abs_coc_asuint, _205);
    uint _207 = atomicMax(max_abs_coc_asuint, _205);
    barrier();
    if (_97)
    {
        imageStore(tile_output_tex, ivec2(gl_GlobalInvocationID.xy / uvec2(8u)), vec2(uintBitsToFloat(min_abs_coc_asuint), uintBitsToFloat(max_abs_coc_asuint)).xyyy);
    }
    imageStore(coc_out_tex, ivec2(gl_GlobalInvocationID.xy), vec4(_201));
}

