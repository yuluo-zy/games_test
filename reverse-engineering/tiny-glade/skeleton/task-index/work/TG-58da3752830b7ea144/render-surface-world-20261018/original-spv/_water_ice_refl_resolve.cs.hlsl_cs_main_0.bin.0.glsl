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

struct FrexpStructType
{
    vec3 mantissa;
    ivec3 exponent;
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
    uvec2 brdf_fg_lut_dims;
    uint history_valid;
    uvec2 blue_noise_texture_dims;
    uvec2 depth_tex_dims;
    uvec2 ice_roughness_tex_dims;
    uvec2 refl_hit_tex_dims;
    uvec2 refl_hit_radiance_tex_dims;
    uvec2 color_tex_dims;
    uvec2 refl_refr_radiance_history_tex_dims;
    uvec2 refl_refr_radiance_out_tex_dims;
    uvec2 ray_len_history_tex_dims;
    uvec2 ray_len_out_tex_dims;
    uvec2 reprojection_tex_dims;
} _Globals;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 15) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 16) uniform texture2D depth_tex;
layout(set = 0, binding = 17) uniform texture2D ice_roughness_tex;
layout(set = 0, binding = 18) uniform texture2D refl_hit_tex;
layout(set = 0, binding = 19) uniform texture2D refl_hit_radiance_tex;
layout(set = 0, binding = 20, r11f_g11f_b10f) uniform image2D color_tex;
layout(set = 0, binding = 21) uniform texture2D refl_refr_radiance_history_tex;
layout(set = 0, binding = 22, r11f_g11f_b10f) uniform image2D refl_refr_radiance_out_tex;
layout(set = 0, binding = 23) uniform texture2D ray_len_history_tex;
layout(set = 0, binding = 24, rg16f) uniform writeonly image2D ray_len_out_tex;

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
        if (texelFetch(ice_roughness_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x == 1.0)
        {
            imageStore(refl_refr_radiance_out_tex, ivec2(gl_GlobalInvocationID.xy), vec3(0.0).xyzz);
            break;
        }
        vec2 _168 = (vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / vec2(_Globals.depth_tex_dims);
        vec4 _187 = ((texelFetch(blue_noise_texture, ivec2((gl_GlobalInvocationID.xy + uvec2(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(frame_constants.frame_index & 255u)) + vec2(0.5)) * vec2(256.0))) % uvec2(256u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
        vec2 _193 = (_168 - vec2(0.5)) * vec2(2.0, -2.0);
        float _194 = _193.x;
        vec4 _197 = vec4(_194, _193.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view;
        vec4 _199 = vec4(_194, _193.y, texelFetch(depth_tex, ivec2(gl_GlobalInvocationID.xy), int(0u)).x, 1.0);
        vec4 _201 = (_199 * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        vec3 _205 = _201.xyz / vec3(_201.w);
        vec4 _215 = texelFetch(refl_hit_tex, ivec2(gl_GlobalInvocationID.xy / uvec2(2u)), int(0u));
        float _217 = _215.y;
        vec4 _218 = texelFetch(ice_roughness_tex, ivec2(gl_GlobalInvocationID.xy), int(0u));
        float _219 = _218.x;
        float _221 = spvNMax(0.000600000028498470783233642578125, _219 * _219);
        vec3 _224 = normalize(normalize((_197 * frame_constants.view_constants[0u].view_to_world).xyz));
        vec3 _226 = mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * (-_224);
        float _227 = _226.z;
        vec3 _234;
        if (_227 < 0.0)
        {
            vec3 _232 = _226;
            _232.z = _227 * (-0.25);
            _234 = normalize(_232);
        }
        else
        {
            _234 = _226;
        }
        uint _241 = ((gl_GlobalInvocationID.x & 1u) | ((gl_GlobalInvocationID.y & 1u) * 2u)) & 3u;
        float _243 = length((vec4(_205, 1.0) * frame_constants.view_constants[0u].world_to_view).xyz);
        float _247 = -normalize(_197.xyz).z;
        float _248 = spvNMax(9.9999997473787516355514526367188e-06, _243);
        float _249 = 16.0 / _248;
        vec4 _258 = ((_199 * frame_constants.view_constants[0u].clip_to_view) * frame_constants.view_constants[0u].view_to_clip) * frame_constants.view_constants[0u].clip_to_prev_clip;
        vec2 _264 = ((_258.xy / vec2(_258.w)) * vec2(0.5, -0.5)) + vec2(0.5);
        float _272 = float(all(equal(clamp(_264, vec2(0.0), vec2(1.0)), _264)));
        vec2 _279 = _168 + vec4(_264 - ((_199.xy * vec2(0.5, -0.5)) + vec2(0.5)), _272, 1.0).xy;
        float _290 = exp2(-clamp(_249 * length(_215.x), 0.0, 100.0));
        float _292 = (-0.0625) * _248;
        float _296 = spvNMax(0.0, _292 * log2(1.0000000031710768509710513471353e-30 + mix(exp2(-clamp(_249 * textureLod(sampler2D(ray_len_history_tex, sampler_lnc), _279, 0.0).y, 0.0, 100.0)), _290, 0.100000001490116119384765625)));
        float _306 = spvNMax(_296, (((_243 / _247) * frame_constants.view_constants[0u].clip_to_view[1].y) * 0.20000000298023223876953125) * smoothstep(0.0, 0.0500000007450580596923828125 * _243, _296));
        float _313 = (_243 * _247) * frame_constants.view_constants[0u].clip_to_view[1].y;
        float _323 = 1.0 - _221;
        vec3 _327 = vec3(_323 * (sqrt(_323) + _221));
        vec3 _331 = reflect(-normalize(mix(vec3(0.0, 1.0, 0.0), reflect(_224, vec3(0.0, 1.0, 0.0)), _327)), vec3(0.0, 1.0, 0.0));
        vec3 _334 = normalize(cross(vec3(0.0, 1.0, 0.0), _331)) * spvNMax(spvNMin((((_306 / (_306 + _243)) * _243) * _247) * (sqrt(_221) * 0.25), 0.100000001490116119384765625 * _313), (4.0 / float(_Globals.color_tex_dims.y)) * _313);
        vec3 _335 = cross(_331, _334);
        float _339 = float((frame_constants.frame_index * 59u) % 128u) * 1.324717998504638671875;
        vec4 _341;
        float _346;
        _341 = vec4(0.0);
        _346 = 0.0;
        vec4 _342;
        float _347;
        float _344 = 0.0;
        int _348 = 0;
        for (; uint(_348) < 8u; _341 = _342, _344 += 1.0, _346 = _347, _348++)
        {
            float _359 = ((float(_348) + _339) * 2.3999631404876708984375) + (float(_241) * 1.57079637050628662109375);
            vec4 _378 = (vec4(_205 + (((_334 * cos(_359)) + (_335 * sin(_359))) * (pow(_344 + _187.y, 1.0) * 0.125)), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
            uvec2 _392 = uvec2(ivec2(floor((((_378.xyz / vec3(_378.w)).xy * vec2(0.5, -0.5)) + vec2(0.5)) * vec2(_Globals.refl_hit_tex_dims))));
            vec4 _393 = texelFetch(refl_hit_tex, ivec2(_392), int(0u));
            float _394 = _393.x;
            float _395 = _393.y;
            if (_395 != 0.0)
            {
                float _418 = (9.9999999747524270787835121154785e-07 * exp((-1.0) * abs((4.0 / (1.0 + (_296 * _296))) - (4.0 / (1.0 + (_394 * _394)))))) * (9.9999999747524270787835121154785e-07 + clamp(1.0 - (8.0 * abs(_217 - _395)), 0.0, 1.0));
                _342 = _341 + (vec4(texelFetch(refl_hit_radiance_tex, ivec2(_392), int(0u)).xyz, 1.0) * _418);
                _347 = _346 + (_290 * _418);
            }
            else
            {
                _342 = _341;
                _347 = _346;
            }
        }
        float _428 = spvNMax(9.9999998245167004418121337039338e-15, _341.w);
        vec3 _441 = normalize(mix(vec3(0.0, 0.0, 1.0), reflect(-_234, vec3(0.0, 0.0, 1.0)), _327));
        vec3 _454 = (((_341.xyz / vec3(_428)).xyz * mix(0.100000001490116119384765625, 1.0, exp2((-1.0) - (8.656169891357421875 * dot(_441, _234 + _441))))) * (1.0 + _221)) + imageLoad(refl_refr_radiance_out_tex, ivec2(gl_GlobalInvocationID.xy)).xyz;
        bool _464;
        if (_Globals.history_valid != 0u)
        {
            _464 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _464 = false;
        }
        vec3 _479;
        if (_464)
        {
            vec3 _470 = textureLod(sampler2D(refl_refr_radiance_history_tex, sampler_lnc), _279, 0.0).xyz;
            vec3 _478;
            if (all(greaterThan(_470, vec3(0.0))))
            {
                _478 = mix(_470, _454, vec3(mix(1.0, 0.25, _272)));
            }
            else
            {
                _478 = _454;
            }
            _479 = _478;
        }
        else
        {
            _479 = _454;
        }
        FrexpStructType _480;
        _480.mantissa = frexp(_479, _480.exponent);
        imageStore(refl_refr_radiance_out_tex, ivec2(gl_GlobalInvocationID.xy), (_479 + exp2(vec3(_480.exponent) - vec3(8.0, 8.0, 7.0))).xyzz);
        imageStore(color_tex, ivec2(gl_GlobalInvocationID.xy), (imageLoad(color_tex, ivec2(gl_GlobalInvocationID.xy)).xyz + _479).xyzz);
        imageStore(ray_len_out_tex, ivec2(gl_GlobalInvocationID.xy), vec2(spvNMax(0.0, _292 * log2(1.0000000031710768509710513471353e-30 + (_346 / _428))), _296).xyyy);
        break;
    } while(false);
}

