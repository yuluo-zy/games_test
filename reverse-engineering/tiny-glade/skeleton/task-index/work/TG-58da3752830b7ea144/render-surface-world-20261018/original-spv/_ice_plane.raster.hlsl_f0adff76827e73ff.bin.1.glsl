#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_demote_to_helper_invocation : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_KHR_shader_subgroup_quad : require
layout(early_fragment_tests) in;

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

float _195;
float _228;

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

layout(set = 0, binding = 23, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_ice_refl_count_buffer;

layout(set = 0, binding = 24, std430) buffer _alias_ice_refl_px_buffer
{
    uint _m0[];
} _alias_ice_refl_px_buffer_1;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 1) uniform sampler sampler_llr;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 11) uniform sampler sampler_shadow_nearest;
layout(set = 0, binding = 12) uniform samplerShadow sampler_shadow_linear;
layout(set = 0, binding = 15) uniform texture2DArray shadow_map;
layout(set = 0, binding = 18) uniform texture2D half_depth_tex;
layout(set = 0, binding = 19) uniform texture2D refraction_tex;
layout(set = 0, binding = 21) uniform texture2D shadows_history_tex;
layout(set = 0, binding = 22) uniform texture2D surface_roughness_tex;

layout(location = 1) in vec3 in_var_C1;
layout(location = 0) out vec3 out_var_SV_TARGET0;
layout(location = 1) out float out_var_SV_TARGET1;
layout(location = 2) out uint out_var_SV_TARGET2;
layout(location = 3) out uint out_var_SV_TARGET3;
layout(location = 4) out float out_var_SV_TARGET4;

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
    bool _244 = helperInvocationEXT();
    uvec2 _248 = uvec2(gl_FragCoord.xy);
    uvec2 _249 = _248 / uvec2(2u);
    vec2 _253 = vec2(_Globals.half_depth_tex_dims);
    vec2 _263 = (((vec2(ivec2(_249)) + vec2(0.5)) / _253) - vec2(0.5)) * vec2(2.0, -2.0);
    float _264 = _263.x;
    vec4 _269 = vec4(_264, _263.y, gl_FragCoord.z, 1.0);
    vec3 _272 = -normalize(((vec4(_264, _263.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz);
    vec4 _275 = vec4(0.0, 0.0, 1.0, 0.0) * frame_constants.view_constants[2u].view_to_world;
    vec3 _276 = _275.xyz;
    vec2 _280 = vec2(_Globals.surface_roughness_tex_dims);
    vec2 _282 = in_var_C1.xz * 0.100000001490116119384765625;
    vec2 _284 = mat2(vec2(1.0, 0.0), vec2(-0.57735025882720947265625, 1.1547005176544189453125)) * (_282 * 3.4639999866485595703125);
    vec2 _285 = floor(_284);
    vec2 _286 = fract(_284);
    float _287 = _286.x;
    float _288 = _286.y;
    float _289 = 1.0 - _287;
    float _290 = _289 - _288;
    bool _291 = _290 > 0.0;
    vec3 _303;
    if (_291)
    {
        _303 = vec3(_285, _290);
    }
    else
    {
        _303 = vec3(_285 + vec2(1.0), -_290);
    }
    vec3 _316;
    if (_291)
    {
        _316 = vec3(_285 + vec2(0.0, 1.0), _288);
    }
    else
    {
        _316 = vec3(_285 + vec2(1.0, 0.0), 1.0 - _288);
    }
    vec3 _328;
    if (_291)
    {
        _328 = vec3(_285 + vec2(1.0, 0.0), _287);
    }
    else
    {
        _328 = vec3(_285 + vec2(0.0, 1.0), _289);
    }
    vec4 _336 = vec4(dFdx(_282), -dFdy(_282));
    uvec2 _339 = uvec2(_303.xy);
    uvec2 _343 = ((_339 >> uvec2(8u)) ^ _339.yx) * uvec2(1103515245u);
    vec2 _351 = _336.xy;
    vec2 _352 = _336.zw;
    uvec2 _358 = uvec2(_316.xy);
    uvec2 _362 = ((_358 >> uvec2(8u)) ^ _358.yx) * uvec2(1103515245u);
    uvec2 _376 = uvec2(_328.xy);
    uvec2 _380 = ((_376 >> uvec2(8u)) ^ _376.yx) * uvec2(1103515245u);
    vec4 _400 = textureLod(sampler2D(surface_roughness_tex, sampler_llr), vec2(0.0), (log(spvNMin(_280.x, _280.y)) * 1.44269502162933349609375) + 1.0);
    float _410 = 0.0199999995529651641845703125 + (0.300000011920928955078125 * (1.0 - ((((((textureGrad(sampler2D(surface_roughness_tex, sampler_llr), _282 + (vec2(((_343 >> uvec2(8u)) ^ _343.yx) * uvec2(1103515245u)) * 2.3283064365386962890625e-10), _351, _352) * _303.z) + (textureGrad(sampler2D(surface_roughness_tex, sampler_llr), _282 + (vec2(((_362 >> uvec2(8u)) ^ _362.yx) * uvec2(1103515245u)) * 2.3283064365386962890625e-10), _351, _352) * _316.z)) + (textureGrad(sampler2D(surface_roughness_tex, sampler_llr), _282 + (vec2(((_380 >> uvec2(8u)) ^ _380.yx) * uvec2(1103515245u)) * 2.3283064365386962890625e-10), _351, _352) * _328.z)) - _400) / vec4(length(vec3(_303.z, _316.z, _328.z)))) + _400).x));
    vec3 _411 = mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * _272;
    uint _412 = _248.x;
    uint _413 = _248.y;
    uint _420;
    _420 = 0u;
    uint _421;
    uint _416;
    uint _419;
    for (uint _415 = _413, _418 = _412, _422 = 32u; _422 > 0u; _415 = _416, _418 = _419, _420 = _421, _422 /= 2u)
    {
        uint _429 = uint((_418 & _422) > 0u);
        uint _432 = uint((_415 & _422) > 0u);
        _421 = _420 + ((_422 * _422) * ((3u * _429) ^ _432));
        if (_432 == 0u)
        {
            uint _445;
            uint _446;
            if (_429 == 1u)
            {
                _445 = 63u - _415;
                _446 = 63u - _418;
            }
            else
            {
                _445 = _415;
                _446 = _418;
            }
            _416 = _446;
            _419 = _445;
        }
        else
        {
            _416 = _415;
            _419 = _418;
        }
    }
    mat4 _603;
    float _604;
    uint _449 = frame_constants.frame_index % 64u;
    vec2 _455 = fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_420 + (288u * _449))) + vec2(0.5));
    vec3 _464 = normalize(vec3(_411.xy * vec2(0.07500000298023223876953125), _411.z));
    float _466 = 6.283185482025146484375 * _455.x;
    float _468 = _464.z;
    precise float _67 = fma(1.0 - (_455.y * 0.300000011920928955078125), 1.0 + _468, -_468);
    float _474 = sqrt(clamp(1.0 - (_67 * _67), 0.0, 1.0));
    vec3 _480 = vec3(_474 * cos(_466), _474 * sin(_466), _67) + _464;
    float _489 = _272.y;
    float _490 = 1.0 - _489;
    float _491 = _490 * _490;
    float _497 = 5.588237762451171875 * float(_449);
    float _506 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_412) + _497)) + (0.005837149918079376220703125 * (float(_413) + _497))));
    vec4 _512 = (vec4(_264, _263.y, texelFetch(half_depth_tex, ivec2(_249), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec3 _519 = _269.xyz;
    vec4 _535 = (vec4(in_var_C1 + ((reflect(_272, normalize(vec3(_480.xy * vec2(0.07500000298023223876953125), _480.z)) * mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0))) * vec3(-1.0, 1.0, -1.0)) * spvNMin(4.0, 4.0 * length(in_var_C1 - (_512.xyz / vec3(_512.w))))), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
    vec3 _539 = _535.xyz / vec3(_535.w);
    vec4 _550 = _269 + (vec4(_539 - _519, 0.0) * float(int(sign(_539.z))));
    float _551 = _550.w;
    vec3 _560 = (_550 / vec4(float((_551 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_551)))).xyz;
    vec3 _561 = _560 - _519;
    vec3 _565 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_561, vec3(0.0))) - _519) / _561;
    vec3 _571 = _519 + (_561 * spvNMax(0.0, spvNMax(_565.x, _565.y)));
    vec3 _572 = _560 - _571;
    vec3 _576 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_572, vec3(0.0))) - _571) / _572;
    vec3 _584 = _571 + (_572 * spvNMin(1.0, spvNMin(spvNMin(_576.x, _576.y), _576.z)));
    float _782;
    do
    {
        _603 = frame_constants.view_constants[0u].clip_to_view;
        _604 = _603[3].z;
        float _605 = 0.100000001490116119384765625 * _604;
        uint _606 = uint(int(max(2u, min(3u, uint(int(floor(length((((_584.xy * vec2(0.5, -0.5)) + vec2(0.5)) - ((_571.xy * vec2(0.5, -0.5)) + vec2(0.5))) * _253))))))));
        float _699;
        float _767;
        float _768;
        float _769;
        float _770;
        bool _771;
        do
        {
            vec3 _611 = _584 - _571;
            float _619;
            if (_584.z > 9.5367431640625e-07)
            {
                _619 = float(_606);
            }
            else
            {
                _619 = spvNMax(1.0, float(_606) - 1.0);
            }
            float _620 = 1.0 / _619;
            float _693;
            float _694;
            float _695;
            float _696;
            float _697;
            bool _698;
            if (_606 > 0u)
            {
                float _626 = mix(0.0, 1.0, pow(_506 * _620, 1.0));
                vec3 _628 = _571 + (_611 * _626);
                float _633 = 1.0 / _628.z;
                vec4 _636 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_628.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _637 = _636.x;
                float _640 = (1.00000202655792236328125 / _637) - _633;
                bool _643 = (_640 < 0.0) ? true : false;
                float _686;
                float _687;
                float _688;
                float _689;
                float _690;
                bool _691;
                float _692;
                if (_643)
                {
                    _686 = 0.0;
                    _687 = _640;
                    _688 = _633 - (1.0 / _637);
                    _689 = _605;
                    _690 = _626;
                    _691 = _643;
                    _692 = 0.0;
                }
                else
                {
                    float _651;
                    float _655;
                    _651 = _640;
                    _655 = _626;
                    float _652;
                    bool _654;
                    float _656;
                    float _681;
                    float _682;
                    float _683;
                    float _684;
                    bool _685;
                    uint _648 = 1u;
                    bool _653 = _643;
                    for (;;)
                    {
                        if (_648 < _606)
                        {
                            _656 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_648) + _506) * _620, 1.0)));
                            vec3 _666 = _571 + (_611 * _656);
                            float _671 = 1.0 / _666.z;
                            vec4 _673 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_666.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                            float _674 = _673.x;
                            _652 = (1.00000202655792236328125 / _674) - _671;
                            _654 = (_652 < 0.0) ? true : false;
                            if (_654)
                            {
                                _681 = _652;
                                _682 = _671 - (1.0 / _674);
                                _683 = _605;
                                _684 = _656;
                                _685 = _654;
                                break;
                            }
                            _648++;
                            _651 = _652;
                            _653 = _654;
                            _655 = _656;
                            continue;
                        }
                        else
                        {
                            _681 = 0.0;
                            _682 = 0.0;
                            _683 = 0.0;
                            _684 = 1.0;
                            _685 = _653;
                            break;
                        }
                    }
                    _686 = _651;
                    _687 = _681;
                    _688 = _682;
                    _689 = _683;
                    _690 = _684;
                    _691 = _685;
                    _692 = _655;
                }
                _693 = _686;
                _694 = _687;
                _695 = _688;
                _696 = _689;
                _697 = _690;
                _698 = _691;
                _699 = _692;
            }
            else
            {
                _693 = 0.0;
                _694 = 0.0;
                _695 = 0.0;
                _696 = 0.0;
                _697 = 1.0;
                _698 = false;
                _699 = 0.0;
            }
            if (_698)
            {
                float _704;
                float _707;
                float _709;
                float _711;
                float _713;
                float _715;
                _704 = _699;
                _707 = _693;
                _709 = _694;
                _711 = _695;
                _713 = _696;
                _715 = _697;
                for (uint _717 = 0u; _717 < 1u; )
                {
                    float _721 = _704 + _715;
                    float _722 = _721 * 0.5;
                    vec3 _724 = _571 + (_611 * _722);
                    float _729 = 1.0 / _724.z;
                    vec4 _732 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_724.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _733 = _732.x;
                    float _736 = (1.00000202655792236328125 / _733) - _729;
                    bool _739 = (_736 < 0.0) ? true : false;
                    _704 = _739 ? _704 : _722;
                    _707 = _739 ? _707 : _736;
                    _709 = _739 ? _736 : _709;
                    _711 = _739 ? (_729 - (1.0 / _733)) : _711;
                    _713 = _739 ? _605 : _713;
                    _715 = _739 ? _722 : _715;
                    _717++;
                    continue;
                }
                float _743 = mix(_704, _715, _707 / (_707 + (-_709)));
                vec3 _745 = _571 + (_611 * _743);
                float _750 = 1.0 / _745.z;
                vec4 _753 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_745.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _754 = _753.x;
                float _757 = (1.00000202655792236328125 / _754) - _750;
                bool _762 = (abs(_757) < (_707 * 0.89999997615814208984375)) ? true : false;
                _767 = _762 ? _743 : _715;
                _768 = _762 ? _757 : _709;
                _769 = _762 ? _605 : _713;
                _770 = _762 ? (_750 - (1.0 / _754)) : _711;
                _771 = true;
                break;
            }
            else
            {
                _767 = _699;
                _768 = _195;
                _769 = _195;
                _770 = _195;
                _771 = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _779;
        if (_771)
        {
            bool _778;
            if (_770 < _769)
            {
                _778 = _768 < _769;
            }
            else
            {
                _778 = false;
            }
            _779 = _778;
        }
        else
        {
            _779 = false;
        }
        if (_779)
        {
            _782 = _767;
            break;
        }
        _782 = _699;
        break;
    } while(false);
    vec3 _784 = mix(_571, _584, vec3(_782));
    float _785 = _784.x;
    vec2 _790 = (vec4(_785, _784.y, _228, 1.0).xy * vec2(0.5, -0.5)) + vec2(0.5);
    vec4 _797 = (vec4(_785, _784.y, textureLod(sampler2D(half_depth_tex, sampler_nnc), _790, 0.0).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec3 _801 = _797.xyz / vec3(_797.w);
    vec3 _807 = _801 - in_var_C1;
    vec3 _813 = -_276;
    ivec2 _814 = ivec2(_248);
    vec2 _1022;
    float _1023;
    do
    {
        vec4 _822 = vec4(in_var_C1, 1.0) * frame_constants.view_constants[2u].world_to_sample;
        if (_822.z > 1.0)
        {
            _1022 = vec2(0.0);
            _1023 = 0.0;
            break;
        }
        vec2 _827 = _822.xy;
        bool _830 = any(greaterThan(abs(_827), vec2(0.9833333492279052734375)));
        vec2 _834;
        if (_830)
        {
            _834 = _827 * 0.5;
        }
        else
        {
            _834 = _827;
        }
        float _835 = _830 ? 5.0 : 10.0;
        float _837 = _830 ? 0.5 : 1.0;
        bool _843 = any(greaterThan(abs(_834), vec2(1.0) - (vec2(0.000833333353511989116668701171875) * (2.0 * _835))));
        float _849;
        vec2 _850;
        float _851;
        if (_843)
        {
            _849 = _835 * 0.2857142984867095947265625;
            _850 = _834 * 0.2857142984867095947265625;
            _851 = _837 * 0.2857142984867095947265625;
        }
        else
        {
            _849 = _835;
            _850 = _834;
            _851 = _837;
        }
        float _854 = float(_843 ? 2u : uint(_830));
        uint _856 = 2u + uint(_854);
        float _859 = _813.y;
        vec4 _871 = vec4(_850.x, _850.y, _822.z, _822.w) + (vec4((vec3(0.0, 1.0, 0.0) * (0.066666670143604278564453125 * clamp(1.0 - _859, 0.0, 1.0))) / vec3(_851), 0.0) * frame_constants.view_constants[_856].world_to_sample);
        vec2 _874 = (_871.xy * vec2(0.5, -0.5)) + vec2(0.5);
        vec3 _875 = vec3(_874.x, _874.y, _871.z);
        float _880 = spvNMax((0.300000011920928955078125 / _851) * (1.0 - abs(_859)), 0.02500000037252902984619140625);
        float _891 = float((uint((_814.x * 11) + (_814.y * 7)) + (frame_constants.frame_index * 5u)) % 64u) * 1.61803400516510009765625;
        uvec2 _892 = uvec2(_814);
        float _904 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_892.x) + _497)) + (0.005837149918079376220703125 * (float(_892.y) + _497))));
        vec4 _906 = vec4(0.0, 1.0, 0.0, 0.0) * frame_constants.view_constants[_856].world_to_sample;
        float _908;
        int _911;
        _908 = 0.0;
        _911 = 0;
        float _909;
        int _912;
        for (int _913 = 0; _913 < 5; _908 = _909, _911 = _912, _913++)
        {
            float _918 = float(_913);
            float _920 = (_918 + _891) * 2.3999631404876708984375;
            vec3 _933 = _875 + vec3((((vec2(cos(_920), sin(_920)) * (_918 + _904)) * vec2(0.20000000298023223876953125)) * _849) * vec2(0.000833333353511989116668701171875), _880 * (-0.0009765625));
            vec4 _940 = textureLod(sampler2DArray(shadow_map, sampler_shadow_nearest), vec3(_933.xy, _854), 0.0);
            float _941 = _940.x;
            if (_933.z > _941)
            {
                _909 = _908 + _941;
                _912 = _911 + 1;
            }
            else
            {
                _909 = _908;
                _912 = _911;
            }
        }
        float _949 = _908 / float(_911);
        if (_911 == 0)
        {
            _1022 = vec2(0.0);
            _1023 = 0.0;
            break;
        }
        float _963 = spvNMax(((_871.z - _949) * ((17.0666675567626953125 * _849) * pow(10.0, smoothstep(0.0, 0.25, frame_constants.fog_tint_thickness.w)))) / _949, 0.0);
        float _970 = (_963 / pow(pow(_963 / (_849 - 1.0), 5.0) + 1.0, 0.20000000298023223876953125)) + 1.0;
        float _972 = _906.z;
        vec2 _982 = (((_906.xy * float(int(sign(_972)))) / vec2(spvNMax(0.1500000059604644775390625, abs(_972)))) * vec2(0.0009765625)) * 2.0;
        vec2 _984;
        _984 = vec2(0.0);
        for (int _987 = 0; _987 < 10; )
        {
            float _991 = float(_987);
            float _993 = (_991 + _891) * 2.3999631404876708984375;
            vec2 _1000 = ((vec2(cos(_993), sin(_993)) * (_991 + _904)) * vec2(0.100000001490116119384765625)) * _970;
            vec3 _1006 = _875 + vec3(_1000 * vec2(0.000833333353511989116668701171875), _880 * (-0.0009765625));
            float _1018 = 1.0 - textureGrad(sampler2DArrayShadow(shadow_map, sampler_shadow_linear), vec4(vec3(_1006.xy, _854), _1006.z + spvNMin(0.0, dot(_982, _1000))), vec2(0.0), vec2(0.0));
            _984 += vec2(_1018, _1018 * _1018);
            _987++;
            continue;
        }
        _1022 = _984 * 0.100000001490116119384765625;
        _1023 = _970;
        break;
    } while(false);
    float _1027 = clamp((_1022.x - 0.100000001490116119384765625) * 1.4285714626312255859375, 0.0, 1.0);
    vec2 _1040 = vec2(_1027) + ((vec2(-1.0, 1.0) * sqrt(spvNMax(0.0, _1022.y - (_1022.x * _1022.x)))) * (3.0 * spvNMax(0.0, 1.0 - (_1023 * 0.20000000298023223876953125))));
    vec2 _1046 = (vec2(_814) + vec2(0.5)) / vec2(_Globals.framebuffer_dims);
    vec4 _1055 = vec4((_1046 - vec2(0.5)) * vec2(2.0, -2.0), gl_FragCoord.z, 1.0);
    vec4 _1058 = ((_1055 * _603) * frame_constants.view_constants[0u].view_to_clip) * frame_constants.view_constants[0u].clip_to_prev_clip;
    vec2 _1064 = ((_1058.xy / vec2(_1058.w)) * vec2(0.5, -0.5)) + vec2(0.5);
    vec4 _1082 = vec4(_1064 - ((_1055.xy * vec2(0.5, -0.5)) + vec2(0.5)), float(all(equal(clamp(_1064, vec2(0.0), vec2(1.0)), _1064))), 1.0) * vec4(1.0, 1.0, clamp(1.0 - length(frame_constants.view_constants[0u].clip_to_prev_clip[3].xyz), 0.0, 1.0), 1.0);
    float _1085 = _1082.z;
    bool _1099;
    if (_1085 > 0.0)
    {
        bool _1098;
        if (_Globals.history_valid != 0u)
        {
            _1098 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _1098 = false;
        }
        _1099 = _1098;
    }
    else
    {
        _1099 = false;
    }
    float _1107;
    if (_1099)
    {
        _1107 = textureLod(sampler2D(shadows_history_tex, sampler_lnc), _1046 + _1082.xy, 0.0).x;
    }
    else
    {
        _1107 = _1027;
    }
    float _1112 = mix(clamp(_1107, _1040.x, _1040.y), _1027, mix(1.0, 0.125, _1085));
    float _1113 = 1.0 - _1112;
    vec4 _1121 = (vec4(in_var_C1 + (_276 * 0.5), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
    vec3 _1125 = _1121.xyz / vec3(_1121.w);
    vec4 _1136 = _269 + (vec4(_1125 - _519, 0.0) * float(int(sign(_1125.z))));
    float _1137 = _1136.w;
    vec3 _1146 = (_1136 / vec4(float((_1137 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_1137)))).xyz;
    vec3 _1147 = _1146 - _519;
    vec3 _1151 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_1147, vec3(0.0))) - _519) / _1147;
    vec3 _1157 = _519 + (_1147 * spvNMax(0.0, spvNMax(_1151.x, _1151.y)));
    vec3 _1158 = _1146 - _1157;
    vec3 _1162 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_1158, vec3(0.0))) - _1157) / _1158;
    vec3 _1170 = _1157 + (_1158 * spvNMin(1.0, spvNMin(spvNMin(_1162.x, _1162.y), _1162.z)));
    bool _1313;
    float _1314;
    float _1315;
    do
    {
        float _1188 = 0.5 * _604;
        uint _1189 = uint(int(max(2u, min(3u, uint(int(floor(length((((_1170.xy * vec2(0.5, -0.5)) + vec2(0.5)) - ((_1157.xy * vec2(0.5, -0.5)) + vec2(0.5))) * _253))))))));
        float _1293;
        float _1297;
        float _1298;
        float _1299;
        float _1300;
        bool _1301;
        do
        {
            vec3 _1194 = _1170 - _1157;
            float _1202;
            if (_1170.z > 9.5367431640625e-07)
            {
                _1202 = float(_1189);
            }
            else
            {
                _1202 = spvNMax(1.0, float(_1189) - 1.0);
            }
            float _1203 = 1.0 / _1202;
            float _1288;
            float _1289;
            float _1290;
            float _1291;
            bool _1292;
            if (_1189 > 0u)
            {
                float _1209 = mix(0.0, 1.0, pow(_506 * _1203, 1.0));
                vec3 _1211 = _1157 + (_1194 * _1209);
                vec2 _1214 = (_1211.xy * vec2(0.5, -0.5)) + vec2(0.5);
                float _1216 = 1.0 / _1211.z;
                float _1221 = 1.0 / textureLod(sampler2D(half_depth_tex, sampler_llc), _1214, 0.0).x;
                float _1225 = 1.0 / textureLod(sampler2D(half_depth_tex, sampler_nnc), _1214, 0.0).x;
                float _1229 = (spvNMax(_1221, _1225) * 1.00000202655792236328125) - _1216;
                float _1230 = _1216 - spvNMin(_1221, _1225);
                bool _1233 = (_1229 < 0.0) ? (_1230 < _1188) : false;
                float _1282;
                float _1283;
                float _1284;
                float _1285;
                bool _1286;
                float _1287;
                if (_1233)
                {
                    _1282 = _1229;
                    _1283 = _1230;
                    _1284 = _1188;
                    _1285 = _1209;
                    _1286 = _1233;
                    _1287 = 0.0;
                }
                else
                {
                    float _1243;
                    _1243 = _1209;
                    bool _1242;
                    float _1244;
                    float _1277;
                    float _1278;
                    float _1279;
                    float _1280;
                    bool _1281;
                    uint _1238 = 1u;
                    bool _1241 = _1233;
                    for (;;)
                    {
                        if (_1238 < _1189)
                        {
                            _1244 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_1238) + _506) * _1203, 1.0)));
                            vec3 _1254 = _1157 + (_1194 * _1244);
                            vec2 _1257 = (_1254.xy * vec2(0.5, -0.5)) + vec2(0.5);
                            float _1259 = 1.0 / _1254.z;
                            float _1263 = 1.0 / textureLod(sampler2D(half_depth_tex, sampler_llc), _1257, 0.0).x;
                            float _1267 = 1.0 / textureLod(sampler2D(half_depth_tex, sampler_nnc), _1257, 0.0).x;
                            float _1271 = (spvNMax(_1263, _1267) * 1.00000202655792236328125) - _1259;
                            float _1272 = _1259 - spvNMin(_1263, _1267);
                            _1242 = (_1271 < 0.0) ? (_1272 < _1188) : false;
                            if (_1242)
                            {
                                _1277 = _1271;
                                _1278 = _1272;
                                _1279 = _1188;
                                _1280 = _1244;
                                _1281 = _1242;
                                break;
                            }
                            _1238++;
                            _1241 = _1242;
                            _1243 = _1244;
                            continue;
                        }
                        else
                        {
                            _1277 = 0.0;
                            _1278 = 0.0;
                            _1279 = 0.0;
                            _1280 = 1.0;
                            _1281 = _1241;
                            break;
                        }
                    }
                    _1282 = _1277;
                    _1283 = _1278;
                    _1284 = _1279;
                    _1285 = _1280;
                    _1286 = _1281;
                    _1287 = _1243;
                }
                _1288 = _1282;
                _1289 = _1283;
                _1290 = _1284;
                _1291 = _1285;
                _1292 = _1286;
                _1293 = _1287;
            }
            else
            {
                _1288 = 0.0;
                _1289 = 0.0;
                _1290 = 0.0;
                _1291 = 1.0;
                _1292 = false;
                _1293 = 0.0;
            }
            if (_1292)
            {
                _1297 = _1291;
                _1298 = _1288;
                _1299 = _1290;
                _1300 = _1289;
                _1301 = true;
                break;
            }
            else
            {
                _1297 = _1293;
                _1298 = _195;
                _1299 = _195;
                _1300 = _195;
                _1301 = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _1309;
        if (_1301)
        {
            bool _1308;
            if (_1300 < _1299)
            {
                _1308 = _1298 < _1299;
            }
            else
            {
                _1308 = false;
            }
            _1309 = _1308;
        }
        else
        {
            _1309 = false;
        }
        if (_1309)
        {
            _1313 = true;
            _1314 = _1297;
            _1315 = _1300 / _1299;
            break;
        }
        _1313 = false;
        _1314 = _1293;
        _1315 = 0.0;
        break;
    } while(false);
    float _1323;
    if (_1313)
    {
        _1323 = spvNMin(_1113, smoothstep(0.5, 0.0, (1.0 - _1315) * (1.0 - _1314)));
    }
    else
    {
        _1323 = _1113;
    }
    uint _1334;
    if (_244)
    {
        _1334 = 0u;
    }
    else
    {
        uvec2 _1327 = _248 & uvec2(1u);
        _1334 = 1u << ((_1327.x + (_1327.y * 2u)) & 31u);
    }
    uint _1336 = _1334 + subgroupQuadSwapHorizontal(_1334);
    uvec2 _1339 = _248 & uvec2(1u);
    if ((_1339.x + (_1339.y * 2u)) == uint(findMSB(_1336 + subgroupQuadSwapVertical(_1336))))
    {
        uint _1349 = atomicAdd(_alias_ice_refl_count_buffer._m0[3u], 1u);
        _alias_ice_refl_px_buffer_1._m0[_1349] = _249.x + (_249.y * 65536u);
        if ((_1349 & 63u) == 0u)
        {
            uint _1360 = atomicAdd(_alias_ice_refl_count_buffer._m0[0u], 1u);
        }
    }
    float _1363;
    vec3 _1415;
    do
    {
        _1363 = _275.y;
        bool _1369;
        if (!(_1363 <= 0.0))
        {
            _1369 = _489 <= 0.0;
        }
        else
        {
            _1369 = true;
        }
        if (_1369)
        {
            _1415 = vec3(0.0);
            break;
        }
        float _1372 = _410 * _410;
        vec3 _1374 = normalize(_272 + _276);
        float _1375 = _1374.y;
        float _1378 = spvNMax(0.0, 1.0 - dot(_1374, _276));
        float _1379 = _1378 * _1378;
        float _1406 = ((_1375 * _1375) * (_1372 - 1.0)) + 1.0;
        _1415 = ((mix(vec3(0.02999999932944774627685546875), vec3(1.0), vec3((_1379 * _1379) * _1378)) * (((2.0 * _1363) * _489) / ((_1363 * sqrt(((((-_489) * _1372) + _489) * _489) + _1372)) + (_489 * sqrt(((((-_1363) * _1372) + _1363) * _1363) + _1372))))) * (_1372 / ((3.1415927410125732421875 * _1406) * _1406))) / vec3((4.0 * _489) * _1363);
        break;
    } while(false);
    vec3 _1440 = mat3(vec3(1.0, -0.0, 0.0), vec3(-0.0, 0.0, 1.0), vec3(-0.0, -1.0, 0.0)) * (_807 * vec3(1.0, -1.0, 1.0));
    float _1442 = length(_1440);
    vec3 _1444 = _1440 * (1.0 / _1442);
    vec3 _1465 = clamp((exp(vec3(-0.5950000286102294921875, -0.374000012874603271484375, -0.3060000240802764892578125) * ((abs(in_var_C1.y - _801.y) + length(_807)) + 0.699999988079071044921875)) * (0.959999978542327880859375 - (0.959999978542327880859375 * ((_491 * _491) * _490)))) * textureLod(sampler2D(refraction_tex, sampler_lnc), _790, 0.0).xyz, vec3(0.0), vec3(65408.0));
    float _1475 = uintBitsToFloat((floatBitsToUint(spvNMax(spvNMax(1.52587890625e-05, _1465.x), spvNMax(_1465.y, _1465.z))) + 125845504u) & 2139095040u);
    uvec3 _1478 = floatBitsToUint(_1465 + vec3(_1475));
    out_var_SV_TARGET0 = mix((frame_constants.sun_color.xyz * _1415.x) * (_1323 * clamp(_1363, 0.0, 1.0)), ((frame_constants.sun_color.xyz * (0.699999988079071044921875 * _1323)) * vec3(0.3183098733425140380859375)) + frame_constants.sky_high_color.xyz, vec3(0.00999999977648258209228515625));
    out_var_SV_TARGET1 = sqrt(_410);
    out_var_SV_TARGET2 = ((uint((clamp((_1444.x * 0.5) + 0.5, 0.0, 1.0) * 255.0) + 0.5) << 24u) | (uint((clamp((_1444.y * 0.5) + 0.5, 0.0, 1.0) * 255.0) + 0.5) << 16u)) | packHalf2x16(vec2(_1442, 0.0));
    out_var_SV_TARGET3 = ((((floatBitsToUint(_1475) << 4u) + 268435456u) | (_1478.z << 18u)) | (_1478.y << 9u)) | (_1478.x & 511u);
    out_var_SV_TARGET4 = _1112 + 0.00196078442968428134918212890625;
}

