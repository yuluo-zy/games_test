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

struct DofFocusReadback
{
    vec3 lock_pos_ws;
    uint lock_reponse;
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
    vec2 point_uv;
    uint use_dual_focus;
    uint focus_lock_request;
    uint focus_is_locked;
    float focus_box_height;
    uint use_tilt_shift;
    float aperture_size;
} _Globals;

layout(set = 0, binding = 15, std430) buffer type_RWStructuredBuffer_DofFocus
{
    DofFocus _m0[];
} _alias_autofocus_buf;

layout(set = 0, binding = 16, std430) buffer type_RWStructuredBuffer_DofFocusReadback
{
    DofFocusReadback _m0[];
} _alias_focus_readback_buf;

layout(set = 0, binding = 14) uniform texture2D depth_tex;

shared uint group_focus0_dists[64];
shared uint group_focus1_dists[64];

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
    vec2 _119 = (vec2(gl_GlobalInvocationID.xy) * vec2(0.14285714924335479736328125)) - vec2(0.5);
    barrier();
    bool _123 = !(0u == frame_constants.frame_index);
    float _128 = _alias_autofocus_buf._m0[frame_constants.camera_idx].focus0;
    float _130 = _alias_autofocus_buf._m0[frame_constants.camera_idx].focus1;
    float _132 = _alias_autofocus_buf._m0[frame_constants.camera_idx].lock_bias0;
    float _134 = _alias_autofocus_buf._m0[frame_constants.camera_idx].lock_bias1;
    vec3 _136 = _alias_autofocus_buf._m0[frame_constants.camera_idx].lock_pos_ws;
    vec3 _138 = _alias_autofocus_buf._m0[frame_constants.camera_idx].interp_lock_pos_ws;
    float _140 = _alias_autofocus_buf._m0[frame_constants.camera_idx].interp_lock_bias0;
    float _142 = _alias_autofocus_buf._m0[frame_constants.camera_idx].interp_lock_bias1;
    uint _144 = _alias_autofocus_buf._m0[frame_constants.camera_idx].pad2;
    float _145 = clamp(_128, 0.0, 100000.0);
    float _146 = clamp(_130, 0.0, 100000.0);
    vec2 _156 = vec2(1.0 / (frame_constants.view_constants[0u].view_to_clip[1].y / frame_constants.view_constants[0u].view_to_clip[0].x), 1.0) * _Globals.focus_box_height;
    bool _159 = _Globals.use_dual_focus != 0u;
    vec3 _220;
    if (_159)
    {
        vec2 _167 = _Globals.point_uv + ((_119 * _156) * 0.20000000298023223876953125);
        vec4 _187 = (vec4((_167 - vec2(0.5)) * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(uvec2(ivec2(_167 * vec2(_Globals.depth_tex_dims)))), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        _220 = _187.xyz / vec3(_187.w);
    }
    else
    {
        vec2 _195 = _Globals.point_uv + (_119 * _156);
        vec4 _215 = (vec4((_195 - vec2(0.5)) * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(uvec2(ivec2(_195 * vec2(_Globals.depth_tex_dims)))), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        _220 = _215.xyz / vec3(_215.w);
    }
    vec3 _249;
    if (_159)
    {
        vec2 _224 = _119 * _156;
        vec4 _244 = (vec4(_224 * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(uvec2(ivec2((vec2(0.5) + _224) * vec2(_Globals.depth_tex_dims)))), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        _249 = _244.xyz / vec3(_244.w);
    }
    else
    {
        _249 = _220;
    }
    bool _252 = _Globals.use_tilt_shift != 0u;
    float _254 = float(_252 ? 0 : 1);
    vec3 _257 = _220;
    _257.y = _220.y * _254;
    vec4 _260 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
    vec3 _264 = _260.xyz / vec3(_260.w);
    vec3 _268 = normalize((vec4(0.0, 0.0, -1.0, 0.0) * frame_constants.view_constants[0u].view_to_world).xyz);
    vec3 _272 = _249;
    _272.y = _249.y * _254;
    group_focus0_dists[gl_LocalInvocationIndex] = floatBitsToUint(dot(_257 - _264, _268));
    group_focus1_dists[gl_LocalInvocationIndex] = floatBitsToUint(dot(_272 - _264, _268));
    barrier();
    uint _279 = uint(findMSB(63u));
    uint _281 = 2u << (_279 & 31u);
    for (uint _283 = 2u; _283 <= _281; _283 *= 2u)
    {
        uint _289 = _283 >> 1u;
        for (uint _291 = _289; _291 > 0u; _291 = _291 >> 1u)
        {
            uint _298 = gl_LocalInvocationIndex & (_291 - 1u);
            uint _300 = 2u * (gl_LocalInvocationIndex - _298);
            uint _301 = _300 + _298;
            uint _312;
            if (_291 == _289)
            {
                _312 = (_300 + ((2u * _291) - 1u)) - _298;
            }
            else
            {
                _312 = (_300 + _291) + _298;
            }
            bool _317;
            if (_312 < 64u)
            {
                _317 = _301 < 64u;
            }
            else
            {
                _317 = false;
            }
            if (_317)
            {
                if (group_focus0_dists[_301] > group_focus0_dists[_312])
                {
                    uint _327 = group_focus0_dists[_301];
                    group_focus0_dists[_301] = group_focus0_dists[_312];
                    group_focus0_dists[_312] = _327;
                }
            }
            barrier();
        }
    }
    for (uint _330 = 2u; _330 <= _281; _330 *= 2u)
    {
        uint _336 = _330 >> 1u;
        for (uint _338 = _336; _338 > 0u; _338 = _338 >> 1u)
        {
            uint _345 = gl_LocalInvocationIndex & (_338 - 1u);
            uint _347 = 2u * (gl_LocalInvocationIndex - _345);
            uint _348 = _347 + _345;
            uint _359;
            if (_338 == _336)
            {
                _359 = (_347 + ((2u * _338) - 1u)) - _345;
            }
            else
            {
                _359 = (_347 + _338) + _345;
            }
            bool _364;
            if (_359 < 64u)
            {
                _364 = _348 < 64u;
            }
            else
            {
                _364 = false;
            }
            if (_364)
            {
                if (group_focus1_dists[_348] > group_focus1_dists[_359])
                {
                    uint _374 = group_focus1_dists[_348];
                    group_focus1_dists[_348] = group_focus1_dists[_359];
                    group_focus1_dists[_359] = _374;
                }
            }
            barrier();
        }
    }
    if (0u == gl_LocalInvocationIndex)
    {
        float _381 = uintBitsToFloat(group_focus0_dists[0]);
        float _384 = uintBitsToFloat(group_focus0_dists[63u]);
        float _393 = uintBitsToFloat(group_focus0_dists[32u]);
        float _396 = uintBitsToFloat(group_focus1_dists[32u]);
        float _402;
        float _403;
        if (_252)
        {
            bool _399 = !_159;
            _402 = _399 ? _384 : _396;
            _403 = _399 ? _381 : _393;
        }
        else
        {
            _402 = _396;
            _403 = _393;
        }
        float _450;
        float _451;
        float _452;
        float _453;
        vec3 _454;
        if (_Globals.focus_lock_request == frame_constants.camera_idx)
        {
            vec4 _432 = (vec4((_Globals.point_uv - vec2(0.5)) * vec2(2.0, -2.0), texelFetch(depth_tex, ivec2(uvec2(ivec2(_Globals.point_uv * vec2(_Globals.depth_tex_dims)))), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
            vec3 _436 = _432.xyz / vec3(_432.w);
            vec3 _439 = _436;
            _439.y = _436.y * _254;
            float _441 = dot(_439 - _264, _268);
            _450 = spvNMax(_145, _146) - _441;
            _451 = spvNMax(_403, _402) - _441;
            _452 = spvNMin(_145, _146) - _441;
            _453 = spvNMin(_403, _402) - _441;
            _454 = _436;
        }
        else
        {
            _450 = _142;
            _451 = _134;
            _452 = _140;
            _453 = _132;
            _454 = _136;
        }
        float _539;
        float _540;
        vec3 _541;
        float _542;
        float _543;
        if (_Globals.focus_is_locked != 0u)
        {
            vec3 _469;
            if (_123)
            {
                _469 = mix(_138, _454, vec3(1.0 - exp(frame_constants.delta_time_seconds * (-8.0))));
            }
            else
            {
                _469 = _454;
            }
            float _479;
            if (_123)
            {
                _479 = mix(_452, _453, 1.0 - exp(frame_constants.delta_time_seconds * (-8.0)));
            }
            else
            {
                _479 = _453;
            }
            float _489;
            if (_123)
            {
                _489 = mix(_450, _451, 1.0 - exp(frame_constants.delta_time_seconds * (-8.0)));
            }
            else
            {
                _489 = _451;
            }
            vec3 _492 = _454;
            _492.y = _454.y * _254;
            float _494 = dot(_492 - _264, _268);
            _539 = _494 + _479;
            _540 = _494 + _489;
            _541 = _469;
            _542 = _479;
            _543 = _489;
        }
        else
        {
            float _537;
            float _538;
            if (_123)
            {
                float _505 = 1.0 - exp(frame_constants.delta_time_seconds * (-8.0));
                float _522 = 1.0 - exp(frame_constants.delta_time_seconds * (-4.0));
                _537 = 1.0 / spvNMax(9.9999997473787516355514526367188e-06, mix(1.0 / spvNMax(9.9999997473787516355514526367188e-06, 1.0 / spvNMax(9.9999997473787516355514526367188e-06, mix(1.0 / spvNMax(9.9999997473787516355514526367188e-06, _145), 1.0 / spvNMax(9.9999997473787516355514526367188e-06, clamp(_145, _381, _384)), _505))), 1.0 / spvNMax(9.9999997473787516355514526367188e-06, _403), _522));
                _538 = 1.0 / spvNMax(9.9999997473787516355514526367188e-06, mix(1.0 / spvNMax(9.9999997473787516355514526367188e-06, 1.0 / spvNMax(9.9999997473787516355514526367188e-06, mix(1.0 / spvNMax(9.9999997473787516355514526367188e-06, _146), 1.0 / spvNMax(9.9999997473787516355514526367188e-06, clamp(_146, uintBitsToFloat(group_focus1_dists[0]), uintBitsToFloat(group_focus1_dists[63u]))), _505))), 1.0 / spvNMax(9.9999997473787516355514526367188e-06, _402), _522));
            }
            else
            {
                _537 = _403;
                _538 = _402;
            }
            _539 = _537;
            _540 = _538;
            _541 = _138;
            _542 = _452;
            _543 = _450;
        }
        _alias_autofocus_buf._m0[frame_constants.camera_idx] = DofFocus(_539, _540, _453, _451, _454, _Globals.focus_is_locked, _541, _Globals.use_tilt_shift, _542, _543, _Globals.aperture_size, _144);
        _alias_focus_readback_buf._m0[0u] = DofFocusReadback(_454, _Globals.focus_lock_request);
    }
}

