#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_demote_to_helper_invocation : require
#extension GL_EXT_spirv_intrinsics : require
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

struct RippleSourceSsbo
{
    uvec4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
};

uint _185;
float _186;
const uvec2 _190[4] = uvec2[](uvec2(0u), uvec2(0u, 1u), uvec2(1u), uvec2(1u, 0u));
const float _191[4] = float[](2.0, 4.0, 8.0, 12.0);
const float _192[4] = float[](2.0, 2.0, 0.5, 0.20000000298023223876953125);
const uint _193[4] = uint[](17u, 18u, 20u, 21u);

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
    layout(row_major, offset = 16) mat4 model_to_world;
    layout(offset = 80) uvec2 terrain_heightmap_dims;
    layout(offset = 88) uvec2 visual_terrain_heightmap_dims;
    layout(offset = 96) uint history_valid;
    layout(offset = 100) uvec2 terrain_lighting_texture_dims;
    layout(offset = 112) uvec2 blue_noise_texture_dims;
    layout(offset = 120) uvec2 brdf_fg_lut_dims;
    layout(offset = 128) uint rt_bvh_node_count;
    layout(offset = 132) uint rt_triangle_count;
    layout(offset = 136) uvec2 water_noise_texture_dims;
    layout(offset = 144) uvec2 framebuffer_dims;
    layout(offset = 152) uvec2 water_mask_dims;
    layout(offset = 160) uvec2 half_depth_tex_dims;
    layout(offset = 168) uvec2 refraction_tex_dims;
    layout(offset = 176) uvec2 taa_history_tex_dims;
    layout(offset = 184) uvec2 reprojection_tex_dims;
    layout(offset = 192) vec3 water_color;
} _Globals;

layout(set = 0, binding = 25, std430) readonly buffer type_StructuredBuffer_RippleSourceSsbo
{
    RippleSourceSsbo _m0[];
} ripple_source_buffer;

layout(set = 0, binding = 33, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_water_rt_count_buffer;

layout(set = 0, binding = 34, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_water_rt_ray_buffer;

layout(set = 0, binding = 1) uniform sampler sampler_llr;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 26) uniform texture2D water_noise_texture;
layout(set = 0, binding = 28) uniform texture2D half_depth_tex;

layout(location = 1) in vec3 in_var_C1;

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
    bool _213 = helperInvocationEXT();
    uvec2 _217 = uvec2(gl_FragCoord.xy);
    vec3 _533;
    do
    {
        bool _223 = frame_constants.glade_theme_id == 4u;
        if (_223)
        {
            _533 = vec3(0.0, 1.0, 0.0);
            break;
        }
        vec3 _425;
        do
        {
            if (_223)
            {
                _425 = vec3(0.0);
                break;
            }
            float _235 = float(frame_constants.game_time_sec) + frame_constants.game_time_frac;
            vec2 _212[4];
            uint _237 = 0u;
            uint _240;
            uint _242;
            bool _244 = false;
            bool _246 = false;
            uint _248;
            uint _250;
            bool _252 = false;
            bool _254 = false;
            for (; _237 < 4u; )
            {
                float _260 = float(_193[_237]);
                int _262 = int(floor(_260));
                float _263 = fract(_260);
                uint _264 = uint(_262);
                uint _266 = _264 + (_264 << 10u);
                uint _268 = _266 ^ (_266 >> 6u);
                uint _270 = _268 + (_268 << 3u);
                uint _272 = _270 ^ (_270 >> 11u);
                bool _275 = _254 ? _254 : true;
                uint _276 = _254 ? _250 : 1664525u;
                bool _277 = _252 ? _252 : true;
                uint _278 = _252 ? _248 : 1013904223u;
                uint _282 = (((_264 * _276) + (_272 + (_272 << 15u))) + _278) * _276;
                uint _284 = _282 ^ (_282 >> 11u);
                uint _287 = _284 ^ ((_284 << 7u) & 2636928640u);
                uint _290 = _287 ^ ((_287 << 15u) & 4022730752u);
                uint _292 = _290 ^ (_290 >> 18u);
                uint _294 = _292 + (_292 << 10u);
                uint _296 = _294 ^ (_294 >> 6u);
                uint _298 = _296 + (_296 << 3u);
                uint _300 = _298 ^ (_298 >> 11u);
                bool _303 = _246 ? _246 : true;
                uint _304 = _246 ? _242 : 8388607u;
                bool _305 = _244 ? _244 : true;
                uint _306 = _244 ? _240 : 1065353216u;
                bool _311 = _303 ? _303 : true;
                uint _312 = _303 ? _304 : 8388607u;
                bool _313 = _305 ? _305 : true;
                uint _314 = _305 ? _306 : 1065353216u;
                uint _321 = uint(_262 + 1);
                uint _323 = _321 + (_321 << 10u);
                uint _325 = _323 ^ (_323 >> 6u);
                uint _327 = _325 + (_325 << 3u);
                uint _329 = _327 ^ (_327 >> 11u);
                uint _251 = _275 ? _276 : 1664525u;
                uint _249 = _277 ? _278 : 1013904223u;
                uint _335 = (((_321 * _251) + (_329 + (_329 << 15u))) + _249) * _251;
                uint _337 = _335 ^ (_335 >> 11u);
                uint _340 = _337 ^ ((_337 << 7u) & 2636928640u);
                uint _343 = _340 ^ ((_340 << 15u) & 4022730752u);
                uint _345 = _343 ^ (_343 >> 18u);
                uint _347 = _345 + (_345 << 10u);
                uint _349 = _347 ^ (_347 >> 6u);
                uint _351 = _349 + (_349 << 3u);
                uint _353 = _351 ^ (_351 >> 11u);
                bool _356 = _311 ? _311 : true;
                uint _357 = _311 ? _312 : 8388607u;
                bool _358 = _313 ? _313 : true;
                uint _359 = _313 ? _314 : 1065353216u;
                uint _243 = _356 ? _357 : 8388607u;
                uint _241 = _358 ? _359 : 1065353216u;
                float _372 = (_263 * _263) * (3.0 - (2.0 * _263));
                _212[_237] = (normalize(((vec2(uintBitsToFloat((_292 & _304) | _306) - 1.0, uintBitsToFloat(((_300 + (_300 << 15u)) & _312) | _314) - 1.0) * (1.0 - _372)) + (vec2(uintBitsToFloat((_345 & _357) | _359) - 1.0, uintBitsToFloat(((_353 + (_353 << 15u)) & _243) | _241) - 1.0) * _372)) - vec2(0.5)) * _235) * 3.0;
                _237++;
                _240 = _241;
                _242 = _243;
                _244 = _358 ? _358 : true;
                _246 = _356 ? _356 : true;
                _248 = _249;
                _250 = _251;
                _252 = _277 ? _277 : true;
                _254 = _275 ? _275 : true;
                continue;
            }
            vec3 _383;
            _383 = vec3(0.0);
            for (uint _386 = 0u; _386 < 4u; )
            {
                uint _387 = _386 + 1u;
                float _391 = float(_387) * 10.16640758514404296875;
                float _392 = cos(_391);
                float _393 = sin(_391);
                mat2 _397 = mat2(vec2(_392, _393), vec2(-_393, _392));
                vec2 _424 = _383.yz + (_397 * ((((texture(sampler2D(water_noise_texture, sampler_llr), (((in_var_C1.xz * _191[_386]) + _212[_386]) * _397) * 0.01200000010430812835693359375).xyz * vec3(1.0, 2.0, 2.0)) + vec3(-0.5, -1.0, -1.0)) * _192[_386]) * vec2(1.0, _191[_386] * 0.01200000010430812835693359375).xyy).yz);
                _383 = vec3(_383.x, _424.x, _424.y);
                _386 = _387;
                continue;
            }
            _425 = _383;
            break;
        } while(false);
        vec3 _518;
        do
        {
            if (_223)
            {
                _518 = vec3(0.0);
                break;
            }
            uint _432 = min(32u, ripple_source_buffer._m0[0u].count);
            vec3 _434;
            _434 = vec3(0.0);
            for (uint _437 = 0u; _437 < _432; )
            {
                vec2 _451 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_437].x & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_437].x >> 16u) & 65535u).x);
                vec2 _460 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_437].y & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_437].y >> 16u) & 65535u).x);
                vec2 _463 = mix(_451, _460, vec2(0.5));
                vec2 _464 = _463 - in_var_C1.xz;
                float _465 = length(_464);
                vec2 _467 = (_460 - _451) * 3.0;
                float _469 = _465 + dot(_464, _467);
                float _477 = (float(frame_constants.game_time_sec) + frame_constants.game_time_frac) * 6.0;
                float _480 = spvNMax(_465, _469);
                float _481 = _480 * _480;
                float _484 = sin((_469 * 16.3649997711181640625) - _477) / (1.0 + (_481 * _481));
                vec2 _486 = _463 - (in_var_C1.xz - vec2(0.00999999977648258209228515625, 0.0));
                float _487 = length(_486);
                float _489 = _487 + dot(_486, _467);
                float _493 = spvNMax(_487, _489);
                float _494 = _493 * _493;
                vec2 _500 = _463 - (in_var_C1.xz - vec2(0.0, 0.00999999977648258209228515625));
                float _501 = length(_500);
                float _503 = _501 + dot(_500, _467);
                float _507 = spvNMax(_501, _503);
                float _508 = _507 * _507;
                vec3 _513 = vec3(_484, (sin((_489 * 16.3649997711181640625) - _477) / (1.0 + (_494 * _494))) - _484, (sin((_503 * 16.3649997711181640625) - _477) / (1.0 + (_508 * _508))) - _484);
                vec2 _515 = _513.yz * 0.5;
                _434 += (vec3(_513.x, _515.x, _515.y) * uintBitsToFloat(ripple_source_buffer._m0[0u].params[_437].z));
                _437++;
                continue;
            }
            _518 = _434;
            break;
        } while(false);
        vec3 _521 = vec3(_425.y, 1.0, _425.z);
        float _526 = abs(1.0);
        vec2 _530 = _521.xz + (vec3(_518.y, 1.0, _518.z).xz / vec2(_526));
        _533 = normalize(vec3(_530.x, _521.y, _530.y));
        break;
    } while(false);
    vec2 _554 = (((vec2(ivec2((_217 + uvec2(1u)) - _190[frame_constants.frame_index & 3u])) + vec2(0.5)) / vec2(_Globals.framebuffer_dims)) - vec2(0.5)) * vec2(2.0, -2.0);
    float _555 = _554.x;
    vec4 _560 = vec4(_555, _554.y, gl_FragCoord.z, 1.0);
    vec4 _562 = (_560 * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    float _570 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _580 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_217.x) + _570)) + (0.005837149918079376220703125 * (float(_217.y) + _570))));
    vec3 _582 = -reflect(-normalize(((vec4(_555, _554.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz), _533);
    float _583 = _582.y;
    vec3 _589;
    if (_583 < 0.0)
    {
        vec3 _588 = _582;
        _588.y = -_583;
        _589 = _588;
    }
    else
    {
        _589 = _582;
    }
    vec3 _594 = _560.xyz;
    vec4 _605 = _560 + ((vec4(_589, 0.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_clip);
    float _606 = _605.w;
    vec3 _615 = (_605 / vec4(float((_606 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_606)))).xyz;
    vec3 _616 = _615 - _594;
    vec3 _620 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_616, vec3(0.0))) - _594) / _616;
    vec3 _626 = _594 + (_616 * spvNMax(0.0, spvNMax(_620.x, _620.y)));
    vec3 _627 = _615 - _626;
    vec3 _631 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_627, vec3(0.0))) - _626) / _627;
    vec3 _638 = _626 + (_627 * spvNMin(spvNMin(_631.x, _631.y), _631.z));
    bool _835;
    float _836;
    do
    {
        uint _659 = uint(int(max(2u, min(8u, uint(int(floor(length((((_638.xy * vec2(0.5, -0.5)) + vec2(0.5)) - ((_626.xy * vec2(0.5, -0.5)) + vec2(0.5))) * vec2(_Globals.half_depth_tex_dims)))))))));
        float _752;
        float _820;
        float _821;
        float _822;
        float _823;
        bool _824;
        do
        {
            vec3 _664 = _638 - _626;
            float _672;
            if (_638.z > 9.5367431640625e-07)
            {
                _672 = float(_659);
            }
            else
            {
                _672 = spvNMax(1.0, float(_659) - 1.0);
            }
            float _673 = 1.0 / _672;
            float _746;
            float _747;
            float _748;
            float _749;
            float _750;
            bool _751;
            if (_659 > 0u)
            {
                float _679 = mix(0.0, 1.0, pow(_580 * _673, 1.0));
                vec3 _681 = _626 + (_664 * _679);
                float _686 = 1.0 / _681.z;
                vec4 _689 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_681.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _690 = _689.x;
                float _693 = (1.00000202655792236328125 / _690) - _686;
                bool _696 = (_693 < 0.0) ? true : false;
                float _739;
                float _740;
                float _741;
                float _742;
                float _743;
                bool _744;
                float _745;
                if (_696)
                {
                    _739 = 0.0;
                    _740 = _693;
                    _741 = _686 - (1.0 / _690);
                    _742 = frame_constants.view_constants[0u].clip_to_view[3].z;
                    _743 = _679;
                    _744 = _696;
                    _745 = 0.0;
                }
                else
                {
                    float _704;
                    float _708;
                    _704 = _693;
                    _708 = _679;
                    float _705;
                    bool _707;
                    float _709;
                    float _734;
                    float _735;
                    float _736;
                    float _737;
                    bool _738;
                    uint _701 = 1u;
                    bool _706 = _696;
                    for (;;)
                    {
                        if (_701 < _659)
                        {
                            _709 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_701) + _580) * _673, 1.0)));
                            vec3 _719 = _626 + (_664 * _709);
                            float _724 = 1.0 / _719.z;
                            vec4 _726 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_719.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                            float _727 = _726.x;
                            _705 = (1.00000202655792236328125 / _727) - _724;
                            _707 = (_705 < 0.0) ? true : false;
                            if (_707)
                            {
                                _734 = _705;
                                _735 = _724 - (1.0 / _727);
                                _736 = frame_constants.view_constants[0u].clip_to_view[3].z;
                                _737 = _709;
                                _738 = _707;
                                break;
                            }
                            _701++;
                            _704 = _705;
                            _706 = _707;
                            _708 = _709;
                            continue;
                        }
                        else
                        {
                            _734 = 0.0;
                            _735 = 0.0;
                            _736 = 0.0;
                            _737 = 1.0;
                            _738 = _706;
                            break;
                        }
                    }
                    _739 = _704;
                    _740 = _734;
                    _741 = _735;
                    _742 = _736;
                    _743 = _737;
                    _744 = _738;
                    _745 = _708;
                }
                _746 = _739;
                _747 = _740;
                _748 = _741;
                _749 = _742;
                _750 = _743;
                _751 = _744;
                _752 = _745;
            }
            else
            {
                _746 = 0.0;
                _747 = 0.0;
                _748 = 0.0;
                _749 = 0.0;
                _750 = 1.0;
                _751 = false;
                _752 = 0.0;
            }
            if (_751)
            {
                float _757;
                float _760;
                float _762;
                float _764;
                float _766;
                float _768;
                _757 = _752;
                _760 = _746;
                _762 = _747;
                _764 = _748;
                _766 = _749;
                _768 = _750;
                for (uint _770 = 0u; _770 < 3u; )
                {
                    float _774 = _757 + _768;
                    float _775 = _774 * 0.5;
                    vec3 _777 = _626 + (_664 * _775);
                    float _782 = 1.0 / _777.z;
                    vec4 _785 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_777.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _786 = _785.x;
                    float _789 = (1.00000202655792236328125 / _786) - _782;
                    bool _792 = (_789 < 0.0) ? true : false;
                    _757 = _792 ? _757 : _775;
                    _760 = _792 ? _760 : _789;
                    _762 = _792 ? _789 : _762;
                    _764 = _792 ? (_782 - (1.0 / _786)) : _764;
                    _766 = _792 ? frame_constants.view_constants[0u].clip_to_view[3].z : _766;
                    _768 = _792 ? _775 : _768;
                    _770++;
                    continue;
                }
                float _796 = mix(_757, _768, _760 / (_760 + (-_762)));
                vec3 _798 = _626 + (_664 * _796);
                float _803 = 1.0 / _798.z;
                vec4 _806 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_798.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _807 = _806.x;
                float _810 = (1.00000202655792236328125 / _807) - _803;
                bool _815 = (abs(_810) < (_760 * 0.89999997615814208984375)) ? true : false;
                _820 = _815 ? _796 : _768;
                _821 = _815 ? _810 : _762;
                _822 = _815 ? frame_constants.view_constants[0u].clip_to_view[3].z : _766;
                _823 = _815 ? (_803 - (1.0 / _807)) : _764;
                _824 = true;
                break;
            }
            else
            {
                _820 = _752;
                _821 = _186;
                _822 = _186;
                _823 = _186;
                _824 = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _832;
        if (_824)
        {
            bool _831;
            if (_823 < _822)
            {
                _831 = _821 < _822;
            }
            else
            {
                _831 = false;
            }
            _832 = _831;
        }
        else
        {
            _832 = false;
        }
        if (_832)
        {
            _835 = true;
            _836 = _820;
            break;
        }
        _835 = false;
        _836 = _752;
        break;
    } while(false);
    float _837 = subgroupQuadSwapHorizontal(_836);
    float _839 = subgroupQuadSwapHorizontal(float(_835));
    bool _845 = (int(_839 != 0.0) & int(!_835)) != 0;
    bool _846 = _845 ? true : _835;
    float _847 = _845 ? _837 : _836;
    float _848 = subgroupQuadSwapVertical(_847);
    float _850 = subgroupQuadSwapVertical(float(_846));
    bool _856 = (int(_850 != 0.0) & int(!_846)) != 0;
    if (_856 ? true : _846)
    {
    }
    else
    {
        vec3 _863 = mix(_626, _638, vec3(_856 ? _848 : _847));
        vec4 _869 = (vec4(_863, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        uint _884;
        if (_213)
        {
            _884 = 0u;
        }
        else
        {
            uvec2 _877 = _217 & uvec2(1u);
            _884 = 1u << ((_877.x + (_877.y * 2u)) & 31u);
        }
        uint _885 = subgroupQuadSwapHorizontal(_884);
        uint _886 = _884 + _885;
        uint _887 = subgroupQuadSwapVertical(_886);
        uint _888 = _886 + _887;
        uvec2 _889 = _217 & uvec2(1u);
        if ((_889.x + (_889.y * 2u)) == uint(findMSB(_888)))
        {
            uint _899 = atomicAdd(_alias_water_rt_count_buffer._m0[0u], 1u);
            uvec2 _900 = _217 / uvec2(2u);
            vec3 _914 = _533 / vec3((abs(_533.x) + abs(_533.y)) + abs(_533.z));
            vec3 _928;
            if (_914.z < 0.0)
            {
                vec2 _926 = (vec2(1.0) - abs(_914.yx)) * ((step(vec2(0.0), _914.xy) * 2.0) - vec2(1.0));
                _928 = vec3(_926.x, _926.y, _914.z);
            }
            else
            {
                _928 = _914;
            }
            vec2 _931 = (_928.xy * 0.5) + vec2(0.5);
            _alias_water_rt_ray_buffer._m0[_899] = uvec4(_900.x + (_900.y * 65536u), packHalf2x16(vec2(_931.x, 0.0)) | (packHalf2x16(vec2(_931.y, 0.0)) << 16u), floatBitsToUint(dot(_589, (_869.xyz / vec3(_869.w)) - (_562.xyz / vec3(_562.w)))), (floatBitsToUint(gl_FragCoord.z) & 4294967280u) + _888);
        }
    }
}

