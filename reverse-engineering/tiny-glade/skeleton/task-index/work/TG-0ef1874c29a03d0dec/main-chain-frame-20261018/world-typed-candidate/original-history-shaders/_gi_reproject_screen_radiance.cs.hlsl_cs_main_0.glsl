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
    layout(offset = 4) uvec2 half_depth_tex_dims;
    layout(offset = 16) uvec2 half_view_normal_tex_dims;
    layout(offset = 24) uvec2 reprojection_tex_dims;
    layout(offset = 32) uvec2 history_tex_dims;
    layout(offset = 40) uvec2 temporal_stats_tex_dims;
    layout(offset = 48) uvec2 output_tex_dims;
    layout(offset = 56) uvec2 temporal_stats_out_tex_dims;
    layout(offset = 64) uint allow_non_determinism;
} _Globals;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 14) uniform texture2D half_depth_tex;
layout(set = 0, binding = 15) uniform texture2D half_view_normal_tex;
layout(set = 0, binding = 16) uniform texture2D reprojection_tex;
layout(set = 0, binding = 17) uniform texture2D history_tex;
layout(set = 0, binding = 18) uniform texture2D temporal_stats_tex;
layout(set = 0, binding = 19, rgba16f) uniform image2D output_tex;
layout(set = 0, binding = 20, rgba8) uniform image2D temporal_stats_out_tex;

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
    do
    {
        vec4 _131 = texelFetch(half_view_normal_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
        vec3 _132 = _131.xyz;
        float _133 = _131.w;
        float _147 = _131.z;
        vec3 _151 = _132 / vec3((abs(_131.x) + abs(_131.y)) + abs(_147));
        bool _153 = _151.z < 0.0;
        vec3 _165;
        if (_153)
        {
            vec2 _163 = (vec2(1.0) - abs(_151.yx)) * ((step(vec2(0.0), _151.xy) * 2.0) - vec2(1.0));
            _165 = vec3(_163.x, _163.y, _151.z);
        }
        else
        {
            _165 = _151;
        }
        vec2 _168 = (_165.xy * 0.5) + vec2(0.5);
        vec4 _169 = vec4(vec4(0.0).x, _168.x, _168.y, vec4(0.0).w);
        bool _179;
        if (_Globals.history_valid != 0u)
        {
            _179 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _179 = false;
        }
        if (!_179)
        {
            imageStore(temporal_stats_out_tex, ivec2(gl_GlobalInvocationID.xy), _169);
            break;
        }
        vec2 _187 = vec2(_Globals.half_depth_tex_dims);
        vec4 _192 = texelFetch(reprojection_tex, ivec2((gl_GlobalInvocationID.xy * uvec2(2u)) + uvec2(uint(abs(_133) > 0.75), uint(_133 >= 0.0))), int(0u));
        vec2 _194 = ((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / _187) + _192.xy;
        float _195 = _192.z;
        vec4 _336;
        bool _337;
        if (_195 > 0.0)
        {
            vec2 _223;
            if (_195 != 1.0)
            {
                vec4 _208 = vec4(notEqual((uvec4(uint((_195 * 15.0) + 0.5)) & uvec4(1u, 2u, 4u, 8u)), uvec4(0u)));
                _223 = _194 + ((((((vec2(-0.5) * _208.x) + (vec2(0.5, -0.5) * _208.y)) + (vec2(-0.5, 0.5) * _208.z)) + (vec2(0.5) * _208.w)) * 2.0) / _187);
            }
            else
            {
                _223 = _194;
            }
            vec2 _228 = vec2(0.5 / float(_Globals.half_depth_tex_dims.x), 0.0);
            vec2 _230 = clamp(_223, _228, vec2(1.0) - _228);
            vec4 _123[3] = vec4[](vec4(0.0), vec4(0.0), vec4(0.0));
            for (uint _232 = 0u; _232 < 3u; )
            {
                _123[_232] = textureLod(sampler2D(history_tex, sampler_lnc), (_230 * vec2(0.3333333432674407958984375, 1.0)) + vec2(float(_232) * 0.3333333432674407958984375, 0.0), 0.0);
                _232++;
                continue;
            }
            vec3 _263;
            if (_153)
            {
                vec2 _261 = (vec2(1.0) - abs(_151.yx)) * ((step(vec2(0.0), _151.xy) * 2.0) - vec2(1.0));
                _263 = vec3(_261.x, _261.y, _151.z);
            }
            else
            {
                _263 = _151;
            }
            vec2 _266 = (_263.xy * 0.5) + vec2(0.5);
            vec4 _273 = abs(_266.xxxx - textureGatherOffset(sampler2D(temporal_stats_tex, sampler_nnc), _230, ivec2(0), 1));
            vec4 _283 = abs(_266.yyyy - textureGatherOffset(sampler2D(temporal_stats_tex, sampler_nnc), _230, ivec2(0), 2));
            bool _292 = (spvNMin(spvNMin(_273.x, _273.y), spvNMin(_273.z, _273.w)) + spvNMin(spvNMin(_283.x, _283.y), spvNMin(_283.z, _283.w))) < 0.20000000298023223876953125;
            vec4 _334;
            if (_292)
            {
                float _297 = clamp((textureLod(sampler2D(temporal_stats_tex, sampler_nnc), _230, 0.0) * vec4(255.0, 1.0, 1.0, 1.0)).x + 1.0, 1.0, 48.0);
                vec4 _298 = _169;
                _298.x = _297;
                float _299 = 1.0 / _297;
                vec4 _124[3];
                for (uint _301 = 0u; _301 < 3u; )
                {
                    vec4 _314 = mix(_123[_301], imageLoad(output_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(_301 * _Globals.half_depth_tex_dims.x, 0u))), vec4(_299));
                    _314.x = spvNMax(0.0, _314.x);
                    vec3 _319 = clamp(_314.yzw, vec3(-1024.0), vec3(1024.0));
                    _124[_301] = vec4(_314.x, _319.x, _319.y, _319.z);
                    _301++;
                    continue;
                }
                for (uint _323 = 0u; _323 < 3u; )
                {
                    imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(_323 * _Globals.half_depth_tex_dims.x, 0u)), _124[_323]);
                    _323++;
                    continue;
                }
                _334 = _298;
            }
            else
            {
                _334 = _169;
            }
            _336 = _334;
            _337 = _292 ? false : true;
        }
        else
        {
            _336 = _169;
            _337 = true;
        }
        if (_337)
        {
            imageStore(temporal_stats_out_tex, ivec2(gl_GlobalInvocationID.xy), _336);
        }
        bool _346;
        if (_337)
        {
            _346 = _Globals.allow_non_determinism != 0u;
        }
        else
        {
            _346 = false;
        }
        vec4 _461;
        if (_346)
        {
            vec4 _350 = texelFetch(half_depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
            float _351 = _350.x;
            float _352 = spvNMax(0.300000011920928955078125, _147);
            vec4 _125[3] = vec4[](vec4(0.0), vec4(0.0), vec4(0.0));
            vec4 _354;
            float _357;
            float _359;
            _354 = vec4(0.0);
            _357 = 0.0;
            _359 = 0.0;
            vec4 _355;
            float _358;
            float _360;
            for (int _361 = -2; _361 <= 2; _354 = _355, _357 = _358, _359 = _360, _361++)
            {
                _355 = _354;
                _358 = _357;
                _360 = _359;
                vec4 _367;
                float _369;
                float _370;
                for (int _371 = -2; _371 <= 2; _355 = _367, _358 = _369, _360 = _370, _371++)
                {
                    uvec2 _379 = gl_GlobalInvocationID.xy + uvec2(ivec2(_371, _361) * ivec2(2));
                    if (any(greaterThanEqual(_379, _Globals.half_depth_tex_dims)))
                    {
                        _367 = _355;
                        _369 = _358;
                        _370 = _360;
                        continue;
                    }
                    float _396 = dot(_132, texelFetch(half_view_normal_tex, ivec2(_379), int(0u)).xyz);
                    float _399 = ((_396 * _396) * (-0.89999997615814208984375)) + 1.0;
                    float _402 = exp2(-spvNMin(30.0, 80.0 * (_352 * abs(1.0 - (texelFetch(half_depth_tex, ivec2(_379), int(0u)).x / _351))))) * (0.010000000707805156707763671875 / (_399 * _399));
                    vec4 _405 = imageLoad(temporal_stats_out_tex, ivec2(_379)) * vec4(255.0, 1.0, 1.0, 1.0);
                    float _407 = spvNMax(_405.x, 1.0);
                    vec4 _408 = _405;
                    _408.x = _407;
                    for (uint _412 = 0u; _412 < 3u; )
                    {
                        _125[_412] += ((imageLoad(output_tex, ivec2(_379 + uvec2(_412 * _Globals.half_depth_tex_dims.x, 0u))) * _402) * _407);
                        _412++;
                        continue;
                    }
                    _367 = _355 + (_408 * _402);
                    _369 = _358 + _402;
                    _370 = _360 + (_402 * _407);
                }
            }
            float _432 = spvNMax(9.9999996826552253889678874634872e-21, _359);
            for (uint _435 = 0u; _435 < 3u; )
            {
                vec4 _448 = _125[_435] / vec4(_432);
                _448.x = spvNMax(0.0, _448.x);
                vec3 _453 = clamp(_448.yzw, vec3(-1024.0), vec3(1024.0));
                imageStore(output_tex, ivec2(gl_GlobalInvocationID.xy + uvec2(_435 * _Globals.half_depth_tex_dims.x, 0u)), vec4(_448.x, _453.x, _453.y, _453.z));
                _435++;
                continue;
            }
            vec4 _457 = _354 * (1.0 / spvNMax(9.9999996826552253889678874634872e-21, _357));
            _457.x = spvNMin(_457.x, 24.0);
            _461 = _457;
        }
        else
        {
            _461 = _336;
        }
        imageStore(temporal_stats_out_tex, ivec2(gl_GlobalInvocationID.xy), _461 * vec4(0.0039215688593685626983642578125, 1.0, 1.0, 1.0));
        break;
    } while(false);
}

