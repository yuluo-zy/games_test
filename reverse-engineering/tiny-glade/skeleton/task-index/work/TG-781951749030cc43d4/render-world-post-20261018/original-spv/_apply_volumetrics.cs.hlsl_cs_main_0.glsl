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
    layout(offset = 0) uint current_view_index;
    layout(offset = 4) uvec2 depth_tex_dims;
    layout(offset = 12) float global_density_scale;
    layout(offset = 16) float global_size_scale;
    layout(offset = 20) uvec2 blue_noise_texture_dims;
    layout(offset = 32) uvec2 color_tex_dims;
} _Globals;

layout(set = 0, binding = 12) uniform samplerShadow sampler_shadow_linear;
layout(set = 0, binding = 14) uniform texture2DArray shadow_map;
layout(set = 0, binding = 16) uniform texture2D depth_tex;
layout(set = 0, binding = 17) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 18, r11f_g11f_b10f) uniform image2D color_tex;

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
        vec2 _167 = (((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / vec2(_Globals.depth_tex_dims)) - vec2(0.5)) * vec2(2.0, -2.0);
        float _168 = _167.x;
        vec4 _175 = (vec4(_168, _167.y, texelFetch(depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        vec3 _178 = -normalize(((vec4(_168, _167.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz);
        if (frame_constants.fog_tint_thickness.w <= 9.9999997473787516355514526367188e-06)
        {
            break;
        }
        vec4 _198 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
        vec3 _202 = _198.xyz / vec3(_198.w);
        vec3 _203 = (_175.xyz / vec3(_175.w)) - _202;
        float _204 = length(_203);
        vec3 _209 = (_203 * (spvNMin(1024.0, _204) / spvNMax(9.9999996826552253889678874634872e-21, _204))) + _202;
        vec4 _212 = vec4(0.0, 0.0, 1.0, 0.0) * frame_constants.view_constants[2u].view_to_world;
        float _218 = 0.045955993235111236572265625 / pow(1.4224998950958251953125 - (dot(_178, _212.xyz) * (-1.2999999523162841796875)), 1.5);
        float _222 = (0.012500000186264514923095703125 * _Globals.global_density_scale) * frame_constants.fog_tint_thickness.w;
        float _223 = _222 * 0.4000000059604644775390625;
        float _224 = _222 * 1.2000000476837158203125;
        float _227 = 30.0 * _Globals.global_size_scale;
        float _228 = 300.0 * _Globals.global_size_scale;
        float _233 = exp((_222 * (-36.0)) / spvNMax(0.300000011920928955078125, _212.y));
        vec4 _238 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[_Globals.current_view_index].view_to_world;
        vec3 _242 = _238.xyz / vec3(_238.w);
        vec3 _243 = _209 - _242;
        float _244 = length(_243);
        vec4 _260 = ((texelFetch(blue_noise_texture, ivec2(((gl_GlobalInvocationID.xy + uvec2(13u)) + uvec2(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(frame_constants.frame_index)) + vec2(0.5)) * vec2(256.0))) % uvec2(256u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
        float _261 = _260.x;
        vec4 _268 = vec4(_242, 1.0) * frame_constants.view_constants[2u].world_to_sample;
        vec4 _273 = vec4(_209, 1.0) * frame_constants.view_constants[2u].world_to_sample;
        vec3 _278;
        float _280;
        _278 = vec3(0.0);
        _280 = _233;
        float _276;
        vec3 _279;
        float _281;
        float _275 = 0.0;
        int _282 = 0;
        for (; _282 < 6; _275 = _276, _278 = _279, _280 = _281, _282++)
        {
            _276 = pow((float(_282) + _261) * 0.16666667163372039794921875, clamp(1.0 + ((-2.0) * _178.y), 0.5, 2.0));
            float _294 = _276 - _275;
            float _295 = mix(_275, _276, 0.4000000059604644775390625);
            vec3 _297 = _242 + (_243 * _295);
            vec4 _299 = mix(_268, _273, vec4(_295));
            vec2 _300 = _299.xy;
            float _317;
            if (any(notEqual(_300, clamp(_300, vec2(-1.0), vec2(1.0)))))
            {
                _317 = 1.0;
            }
            else
            {
                _317 = textureGrad(sampler2DArrayShadow(shadow_map, sampler_shadow_linear), vec4(vec3((_300 * vec2(0.5, -0.5)) + vec2(0.5), 0.0), _299.z), vec2(0.0), vec2(0.0));
            }
            float _337;
            bool _338;
            vec2 _318 = _297.xz;
            float _319 = length(_318);
            float _369;
            do
            {
                _337 = dot(_318, _318);
                _338 = _337 > 1600.0;
                if (_338)
                {
                    _369 = 1.0;
                    break;
                }
                float _342 = dot(frame_constants.fake_light_dir.xz, frame_constants.fake_light_dir.xz);
                float _343 = dot(_318, frame_constants.fake_light_dir.xz);
                float _344 = 2.0 * _343;
                float _354 = ((_343 * (-2.0)) + sqrt((_344 * _344) - ((4.0 * _342) * (_337 - 1600.0)))) / (2.0 * _342);
                bool _359;
                if (_354 == _354)
                {
                    _359 = _354 >= 0.0;
                }
                else
                {
                    _359 = false;
                }
                if (_359)
                {
                    _369 = float((_297.y + (frame_constants.fake_light_dir.y * _354)) > 18.0);
                    break;
                }
                else
                {
                    _369 = 1.0;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            float _408;
            do
            {
                if (_338)
                {
                    _408 = 1.0;
                    break;
                }
                float _381 = dot(frame_constants.fake_light2_dir.xz, frame_constants.fake_light2_dir.xz);
                float _382 = dot(_318, frame_constants.fake_light2_dir.xz);
                float _383 = 2.0 * _382;
                float _393 = ((_382 * (-2.0)) + sqrt((_383 * _383) - ((4.0 * _381) * (_337 - 1600.0)))) / (2.0 * _381);
                bool _398;
                if (_393 == _393)
                {
                    _398 = _393 >= 0.0;
                }
                else
                {
                    _398 = false;
                }
                if (_398)
                {
                    _408 = float((_297.y + (frame_constants.fake_light2_dir.y * _393)) > 18.0);
                    break;
                }
                else
                {
                    _408 = 1.0;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            vec3 _415 = _297 / vec2(_228, _227).xyx;
            float _419 = 0.60000002384185791015625 * exp2(-dot(_415, _415));
            float _455;
            if (frame_constants.glade_theme_id == 2u)
            {
                _455 = spvNMin(_419 * mix(1.0, 3.0, smoothstep(60.0, 200.0, length(_318 / vec2(_Globals.global_size_scale)))), 0.800000011920928955078125);
            }
            else
            {
                float _447;
                if (frame_constants.glade_theme_id == 3u)
                {
                    _447 = _419 * mix(1.0, 8.0, smoothstep(30.0, 200.0, length(_318 / vec2(_Globals.global_size_scale))));
                }
                else
                {
                    float _440;
                    if (frame_constants.glade_theme_id == 4u)
                    {
                        _440 = _419 * mix(1.0, 4.0, smoothstep(100.0, 300.0, _319));
                    }
                    else
                    {
                        _440 = _419 * mix(1.0, 2.0, smoothstep(100.0, 300.0, _319));
                    }
                    _447 = _440;
                }
                _455 = _447;
            }
            float _456 = spvNMax(_455, 0.0);
            _279 = _278 + (((((((((frame_constants.sun_color.xyz * _317) * _218) * 4.0) + (frame_constants.sky_high_color.xyz * mix(4.0, 8.0, 1.0 - exp((-0.00999999977648258209228515625) * _319)))) * _233) + (((frame_constants.fake_light_color.xyz * _369) + (frame_constants.fake_light2_color.xyz * _408)) * vec3(0.3183098733425140380859375))) * (_223 * _456)) * ((1.0 - exp(-spvNMax(1.0000000116860974230803549289703e-07, ((_244 * _294) * _224) * _456))) / spvNMax(9.9999996826552253889678874634872e-21, _224 * _456))) * _280);
            _281 = _280 * exp((((-_244) * _294) * _224) * _456);
        }
        imageStore(color_tex, ivec2(gl_GlobalInvocationID.xy), ((_278 * frame_constants.fog_tint_thickness.xyz) + (imageLoad(color_tex, ivec2(gl_GlobalInvocationID.xy)).xyz * _280)).xyzz);
        break;
    } while(false);
}

