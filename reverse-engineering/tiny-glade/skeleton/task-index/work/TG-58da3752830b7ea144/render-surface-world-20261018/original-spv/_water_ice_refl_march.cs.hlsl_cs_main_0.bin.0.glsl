#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

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

float _159;

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    layout(offset = 0) uvec2 terrain_lighting_texture_dims;
    layout(offset = 8) uvec2 blue_noise_texture_dims;
    layout(offset = 16) uvec2 terrain_heightmap_dims;
    layout(offset = 24) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 32) uvec2 brdf_fg_lut_dims;
    layout(offset = 40) uint rt_bvh_node_count;
    layout(offset = 44) uint rt_triangle_count;
    layout(offset = 48) uint history_valid;
    layout(offset = 52) uvec2 half_depth_tex_dims;
    layout(offset = 64) uvec2 taa_history_tex_dims;
    layout(offset = 72) uvec2 reprojection_tex_dims;
    layout(offset = 80) uvec2 ice_roughness_tex_dims;
    layout(offset = 88) uvec2 refl_hit_tex_dims;
    layout(offset = 96) uvec2 refl_hit_radiance_tex_dims;
} _Globals;

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

layout(set = 0, binding = 25, std430) readonly buffer type_StructuredBuffer_v4uint
{
    uvec4 _m0[];
} ice_refl_count_buffer;

layout(set = 0, binding = 26, std430) readonly buffer type_StructuredBuffer_uint
{
    uint _m0[];
} ice_refl_px_buffer;

layout(set = 0, binding = 33, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_ice_rt_count_buffer;

layout(set = 0, binding = 34, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_ice_rt_ray_buffer;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 27) uniform texture2D half_depth_tex;
layout(set = 0, binding = 28) uniform texture2D taa_history_tex;
layout(set = 0, binding = 29) uniform texture2D reprojection_tex;
layout(set = 0, binding = 30) uniform texture2D ice_roughness_tex;
layout(set = 0, binding = 31, rg16f) uniform writeonly image2D refl_hit_tex;
layout(set = 0, binding = 32, r32ui) uniform writeonly uimage2D refl_hit_radiance_tex;

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
        if (gl_GlobalInvocationID.x >= ice_refl_count_buffer._m0[0u].w)
        {
            break;
        }
        uint _187 = ice_refl_px_buffer._m0[gl_GlobalInvocationID.x];
        uint _188 = _187 & 65535u;
        uint _190 = (_187 >> 16u) & 65535u;
        uvec2 _191 = uvec2(_188, _190);
        vec2 _196 = vec2(_Globals.half_depth_tex_dims);
        vec2 _205 = (((vec2(ivec2(_191)) + vec2(0.5)) / _196) - vec2(0.5)) * vec2(2.0, -2.0);
        float _206 = _205.x;
        vec3 _212 = normalize(((vec4(_206, _205.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz);
        vec3 _213 = -_212;
        vec4 _214 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
        vec3 _218 = _214.xyz / vec3(_214.w);
        vec3 _224 = _218 + (_212 * ((-_218).y / _212.y));
        vec4 _234 = (vec4(_224, 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
        vec4 _240 = vec4(_206, _205.y, (_234.xyz / vec3(_234.w)).z, 1.0);
        uint _243 = frame_constants.frame_index % 64u;
        float _246 = 5.588237762451171875 * float(_243);
        float _255 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_188) + _246)) + (0.005837149918079376220703125 * (float(_190) + _246))));
        vec4 _257 = texelFetch(ice_roughness_tex, ivec2(_191 * uvec2(2u)), int(0u));
        float _258 = _257.x;
        float _260 = spvNMax(0.000600000028498470783233642578125, _258 * _258);
        vec3 _261 = mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * _213;
        vec3 _383;
        uint _263 = 0u;
        for (;;)
        {
            if (_263 < 8u)
            {
                uint _275;
                _275 = 0u;
                uint _276;
                uint _271;
                uint _274;
                for (uint _270 = _190, _273 = _188, _277 = 32u; _277 > 0u; _270 = _271, _273 = _274, _275 = _276, _277 /= 2u)
                {
                    uint _284 = uint((_273 & _277) > 0u);
                    uint _287 = uint((_270 & _277) > 0u);
                    _276 = _275 + ((_277 * _277) * ((3u * _284) ^ _287));
                    if (_287 == 0u)
                    {
                        uint _300;
                        uint _301;
                        if (_284 == 1u)
                        {
                            _300 = 63u - _270;
                            _301 = 63u - _273;
                        }
                        else
                        {
                            _300 = _270;
                            _301 = _273;
                        }
                        _271 = _301;
                        _274 = _300;
                    }
                    else
                    {
                        _271 = _270;
                        _274 = _273;
                    }
                }
                vec2 _307 = fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_275 + (288u * _243))) + vec2(0.5));
                vec3 _377;
                do
                {
                    float _315 = _261.z;
                    vec3 _317 = normalize(vec3(_260 * _261.x, _260 * _261.y, _315));
                    float _318 = _317.z;
                    vec3 _325;
                    if (_318 < 0.99989998340606689453125)
                    {
                        _325 = normalize(cross(vec3(0.0, 0.0, 1.0), _317));
                    }
                    else
                    {
                        _325 = vec3(1.0, 0.0, 0.0);
                    }
                    float _328 = sqrt(_307.x);
                    float _329 = _307.y * 3.1415927410125732421875;
                    float _331 = _328 * cos(_329);
                    float _335 = 0.5 * (1.0 + _318);
                    float _338 = 1.0 - (_331 * _331);
                    float _342 = ((1.0 - _335) * sqrt(_338)) + (_335 * (_328 * sin(_329)));
                    vec3 _351 = ((_325 * _331) + (cross(_317, _325) * _342)) + (_317 * sqrt(spvNMax(0.0, _338 - (_342 * _342))));
                    vec3 _359 = normalize(vec3(_260 * _351.x, _260 * _351.y, spvNMax(0.0, _351.z)));
                    bool _369;
                    if (!(_359.z <= 9.9999997473787516355514526367188e-06))
                    {
                        _369 = reflect(-_261, _359).z <= 9.9999997473787516355514526367188e-06;
                    }
                    else
                    {
                        _369 = true;
                    }
                    bool _374;
                    if (!_369)
                    {
                        _374 = _315 <= 9.9999997473787516355514526367188e-06;
                    }
                    else
                    {
                        _374 = true;
                    }
                    if (_374)
                    {
                        _377 = vec3(0.0);
                        break;
                    }
                    _377 = _359;
                    break;
                } while(false);
                if (reflect(_261, _377).z < 0.0)
                {
                    _383 = _377;
                    break;
                }
                _263++;
                continue;
            }
            else
            {
                _383 = vec3(0.0, 1.0, 0.0);
                break;
            }
        }
        vec3 _386 = -reflect(_213, _383 * mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)));
        float _387 = _386.y;
        vec3 _393;
        if (_387 < 0.0)
        {
            vec3 _392 = _386;
            _392.y = -_387;
            _393 = _392;
        }
        else
        {
            _393 = _386;
        }
        mat4 _456;
        vec3 _395 = _240.xyz;
        vec4 _404 = _240 + ((vec4(_393, 0.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_clip);
        float _405 = _404.w;
        vec3 _414 = (_404 / vec4(float((_405 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_405)))).xyz;
        vec3 _415 = _414 - _395;
        vec3 _419 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_415, vec3(0.0))) - _395) / _415;
        vec3 _425 = _395 + (_415 * spvNMax(0.0, spvNMax(_419.x, _419.y)));
        vec3 _426 = _414 - _425;
        vec3 _430 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_426, vec3(0.0))) - _425) / _426;
        vec3 _437 = _425 + (_426 * spvNMin(spvNMin(_430.x, _430.y), _430.z));
        bool _638;
        float _639;
        vec2 _640;
        do
        {
            vec2 _442 = (_425.xy * vec2(0.5, -0.5)) + vec2(0.5);
            vec2 _445 = (_437.xy * vec2(0.5, -0.5)) + vec2(0.5);
            _456 = frame_constants.view_constants[0u].clip_to_view;
            uint _458 = uint(int(max(2u, min(16u, uint(int(floor(length((_445 - _442) * _196))))))));
            float _551;
            float _619;
            float _620;
            float _621;
            float _622;
            bool _623;
            do
            {
                vec3 _463 = _437 - _425;
                float _471;
                if (_437.z > 9.5367431640625e-07)
                {
                    _471 = float(_458);
                }
                else
                {
                    _471 = spvNMax(1.0, float(_458) - 1.0);
                }
                float _472 = 1.0 / _471;
                float _545;
                float _546;
                float _547;
                float _548;
                float _549;
                bool _550;
                if (_458 > 0u)
                {
                    float _478 = mix(0.0, 1.0, pow(_255 * _472, 2.0));
                    vec3 _480 = _425 + (_463 * _478);
                    float _485 = 1.0 / _480.z;
                    vec4 _488 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_480.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _489 = _488.x;
                    float _492 = (1.00000202655792236328125 / _489) - _485;
                    bool _495 = (_492 < 0.0) ? true : false;
                    float _538;
                    float _539;
                    float _540;
                    float _541;
                    float _542;
                    bool _543;
                    float _544;
                    if (_495)
                    {
                        _538 = 0.0;
                        _539 = _492;
                        _540 = _485 - (1.0 / _489);
                        _541 = _456[3].z;
                        _542 = _478;
                        _543 = _495;
                        _544 = 0.0;
                    }
                    else
                    {
                        float _503;
                        float _507;
                        _503 = _492;
                        _507 = _478;
                        float _504;
                        bool _506;
                        float _508;
                        float _533;
                        float _534;
                        float _535;
                        float _536;
                        bool _537;
                        uint _500 = 1u;
                        bool _505 = _495;
                        for (;;)
                        {
                            if (_500 < _458)
                            {
                                _508 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_500) + _255) * _472, 2.0)));
                                vec3 _518 = _425 + (_463 * _508);
                                float _523 = 1.0 / _518.z;
                                vec4 _525 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_518.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                                float _526 = _525.x;
                                _504 = (1.00000202655792236328125 / _526) - _523;
                                _506 = (_504 < 0.0) ? true : false;
                                if (_506)
                                {
                                    _533 = _504;
                                    _534 = _523 - (1.0 / _526);
                                    _535 = _456[3].z;
                                    _536 = _508;
                                    _537 = _506;
                                    break;
                                }
                                _500++;
                                _503 = _504;
                                _505 = _506;
                                _507 = _508;
                                continue;
                            }
                            else
                            {
                                _533 = 0.0;
                                _534 = 0.0;
                                _535 = 0.0;
                                _536 = 1.0;
                                _537 = _505;
                                break;
                            }
                        }
                        _538 = _503;
                        _539 = _533;
                        _540 = _534;
                        _541 = _535;
                        _542 = _536;
                        _543 = _537;
                        _544 = _507;
                    }
                    _545 = _538;
                    _546 = _539;
                    _547 = _540;
                    _548 = _541;
                    _549 = _542;
                    _550 = _543;
                    _551 = _544;
                }
                else
                {
                    _545 = 0.0;
                    _546 = 0.0;
                    _547 = 0.0;
                    _548 = 0.0;
                    _549 = 1.0;
                    _550 = false;
                    _551 = 0.0;
                }
                if (_550)
                {
                    float _556;
                    float _559;
                    float _561;
                    float _563;
                    float _565;
                    float _567;
                    _556 = _551;
                    _559 = _545;
                    _561 = _546;
                    _563 = _547;
                    _565 = _548;
                    _567 = _549;
                    for (uint _569 = 0u; _569 < 3u; )
                    {
                        float _573 = _556 + _567;
                        float _574 = _573 * 0.5;
                        vec3 _576 = _425 + (_463 * _574);
                        float _581 = 1.0 / _576.z;
                        vec4 _584 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_576.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                        float _585 = _584.x;
                        float _588 = (1.00000202655792236328125 / _585) - _581;
                        bool _591 = (_588 < 0.0) ? true : false;
                        _556 = _591 ? _556 : _574;
                        _559 = _591 ? _559 : _588;
                        _561 = _591 ? _588 : _561;
                        _563 = _591 ? (_581 - (1.0 / _585)) : _563;
                        _565 = _591 ? _456[3].z : _565;
                        _567 = _591 ? _574 : _567;
                        _569++;
                        continue;
                    }
                    float _595 = mix(_556, _567, _559 / (_559 + (-_561)));
                    vec3 _597 = _425 + (_463 * _595);
                    float _602 = 1.0 / _597.z;
                    vec4 _605 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_597.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _606 = _605.x;
                    float _609 = (1.00000202655792236328125 / _606) - _602;
                    bool _614 = (abs(_609) < (_559 * 0.89999997615814208984375)) ? true : false;
                    _619 = _614 ? _595 : _567;
                    _620 = _614 ? _609 : _561;
                    _621 = _614 ? _456[3].z : _565;
                    _622 = _614 ? (_602 - (1.0 / _606)) : _563;
                    _623 = true;
                    break;
                }
                else
                {
                    _619 = _551;
                    _620 = _159;
                    _621 = _159;
                    _622 = _159;
                    _623 = false;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            bool _631;
            if (_623)
            {
                bool _630;
                if (_622 < _621)
                {
                    _630 = _620 < _621;
                }
                else
                {
                    _630 = false;
                }
                _631 = _630;
            }
            else
            {
                _631 = false;
            }
            if (_631)
            {
                _638 = true;
                _639 = _619;
                _640 = mix(_442, _445, vec2(_619));
                break;
            }
            _638 = false;
            _639 = _551;
            _640 = mix(_442, _445, vec2(_551));
            break;
        } while(false);
        vec4 _646 = vec4(mix(_425, _437, vec3(_639)), 1.0);
        float _722;
        vec3 _723;
        if (_638)
        {
            bool _704;
            if (_Globals.history_valid != 0u)
            {
                _704 = !(0u == frame_constants.frame_index);
            }
            else
            {
                _704 = false;
            }
            vec3 _714;
            if (_704)
            {
                _714 = textureLod(sampler2D(taa_history_tex, sampler_lnc), textureLod(sampler2D(reprojection_tex, sampler_nnc), _640, 0.0).xy + _640, 0.0).xyz;
            }
            else
            {
                _714 = vec3(0.0);
            }
            vec4 _716 = (_646 * _456) * frame_constants.view_constants[0u].view_to_world;
            _722 = distance(_716.xyz / vec3(_716.w), _224);
            _723 = _714;
        }
        else
        {
            vec4 _651 = (_646 * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
            vec3 _655 = _651.xyz / vec3(_651.w);
            uint _657 = atomicAdd(_alias_ice_rt_count_buffer._m0[3u], 1u);
            vec3 _658 = _655 - _218;
            _alias_ice_rt_ray_buffer._m0[_657] = uvec4(_188 + (_190 * 65536u), packHalf2x16(vec2(_658.x, 0.0)) | (packHalf2x16(vec2(_393.x, 0.0)) << 16u), packHalf2x16(vec2(_658.y, 0.0)) | (packHalf2x16(vec2(_393.y, 0.0)) << 16u), packHalf2x16(vec2(_658.z, 0.0)) | (packHalf2x16(vec2(_393.z, 0.0)) << 16u));
            if ((_657 & 63u) == 0u)
            {
                uint _689 = atomicAdd(_alias_ice_rt_count_buffer._m0[0u], 1u);
            }
            _722 = distance(_655, _224);
            _723 = vec3(0.0);
        }
        imageStore(refl_hit_tex, ivec2(_191), vec2(_722, _260).xyyy);
        vec3 _726 = clamp(_723, vec3(0.0), vec3(65408.0));
        float _736 = uintBitsToFloat((floatBitsToUint(spvNMax(spvNMax(1.52587890625e-05, _726.x), spvNMax(_726.y, _726.z))) + 125845504u) & 2139095040u);
        uvec3 _739 = floatBitsToUint(_726 + vec3(_736));
        imageStore(refl_hit_radiance_tex, ivec2(_191), uvec4(((((floatBitsToUint(_736) << 4u) + 268435456u) | (_739.z << 18u)) | (_739.y << 9u)) | (_739.x & 511u)));
        break;
    } while(false);
}

