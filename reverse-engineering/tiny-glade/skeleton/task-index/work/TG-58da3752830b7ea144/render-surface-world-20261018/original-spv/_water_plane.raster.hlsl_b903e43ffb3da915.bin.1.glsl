#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
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

struct RippleSourceSsbo
{
    uvec4 params[32];
    uint count;
    uint pad0;
    uint pad1;
    uint pad2;
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

uint _239;
float _240;
const uvec2 _244[4] = uvec2[](uvec2(0u), uvec2(0u, 1u), uvec2(1u), uvec2(1u, 0u));
const float _246[4] = float[](2.0, 4.0, 8.0, 12.0);
const float _247[4] = float[](2.0, 2.0, 0.5, 0.20000000298023223876953125);
const uint _248[4] = uint[](17u, 18u, 20u, 21u);
float _274;
float _286;
vec3 _287;
vec2 _288;

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

layout(set = 0, binding = 32, std430) readonly buffer type_StructuredBuffer_DofFocus
{
    DofFocus _m0[];
} autofocus_buf;

layout(set = 0, binding = 33, std430) buffer type_RWStructuredBuffer_uint
{
    uint _m0[];
} _alias_water_rt_count_buffer;

layout(set = 0, binding = 34, std430) buffer type_RWStructuredBuffer_v4uint
{
    uvec4 _m0[];
} _alias_water_rt_ray_buffer;

layout(set = 0, binding = 0) uniform sampler sampler_lnc;
layout(set = 0, binding = 1) uniform sampler sampler_llr;
layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 11) uniform sampler sampler_shadow_nearest;
layout(set = 0, binding = 12) uniform samplerShadow sampler_shadow_linear;
layout(set = 0, binding = 19) uniform texture2DArray shadow_map;
layout(set = 0, binding = 26) uniform texture2D water_noise_texture;
layout(set = 0, binding = 27) uniform texture2D water_mask;
layout(set = 0, binding = 28) uniform texture2D half_depth_tex;
layout(set = 0, binding = 29) uniform texture2D refraction_tex;
layout(set = 0, binding = 30) uniform texture2D taa_history_tex;
layout(set = 0, binding = 31) uniform texture2D reprojection_tex;

layout(location = 1) in vec3 in_var_C1;
layout(location = 0) out vec4 out_var_SV_TARGET;

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
    bool _308;
    bool _298 = helperInvocationEXT();
    vec3 _299 = in_var_C1;
    uvec2 _302 = uvec2(gl_FragCoord.xy);
    uint _626;
    uint _627;
    bool _628;
    bool _629;
    uint _630;
    uint _631;
    bool _632;
    bool _633;
    vec3 _634;
    do
    {
        _308 = frame_constants.glade_theme_id == 4u;
        if (_308)
        {
            _626 = _239;
            _627 = _239;
            _628 = false;
            _629 = false;
            _630 = _239;
            _631 = _239;
            _632 = false;
            _633 = false;
            _634 = vec3(0.0, 1.0, 0.0);
            break;
        }
        uint _510;
        uint _511;
        bool _512;
        bool _513;
        uint _514;
        uint _515;
        bool _516;
        bool _517;
        vec3 _518;
        do
        {
            if (_308)
            {
                _510 = _239;
                _511 = _239;
                _512 = false;
                _513 = false;
                _514 = _239;
                _515 = _239;
                _516 = false;
                _517 = false;
                _518 = vec3(0.0);
                break;
            }
            float _320 = float(frame_constants.game_time_sec) + frame_constants.game_time_frac;
            uint _325;
            uint _327;
            bool _329;
            bool _331;
            uint _333;
            uint _335;
            bool _337;
            bool _339;
            _325 = _239;
            _327 = _239;
            _329 = false;
            _331 = false;
            _333 = _239;
            _335 = _239;
            _337 = false;
            _339 = false;
            vec2 _297[4];
            for (uint _322 = 0u; _322 < 4u; )
            {
                float _345 = float(_248[_322]);
                int _347 = int(floor(_345));
                float _348 = fract(_345);
                uint _349 = uint(_347);
                uint _351 = _349 + (_349 << 10u);
                uint _353 = _351 ^ (_351 >> 6u);
                uint _355 = _353 + (_353 << 3u);
                uint _357 = _355 ^ (_355 >> 11u);
                bool _360 = _339 ? _339 : true;
                uint _361 = _339 ? _335 : 1664525u;
                bool _362 = _337 ? _337 : true;
                uint _363 = _337 ? _333 : 1013904223u;
                uint _367 = (((_349 * _361) + (_357 + (_357 << 15u))) + _363) * _361;
                uint _369 = _367 ^ (_367 >> 11u);
                uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
                uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
                uint _377 = _375 ^ (_375 >> 18u);
                uint _379 = _377 + (_377 << 10u);
                uint _381 = _379 ^ (_379 >> 6u);
                uint _383 = _381 + (_381 << 3u);
                uint _385 = _383 ^ (_383 >> 11u);
                bool _388 = _331 ? _331 : true;
                uint _389 = _331 ? _327 : 8388607u;
                bool _390 = _329 ? _329 : true;
                uint _391 = _329 ? _325 : 1065353216u;
                bool _396 = _388 ? _388 : true;
                uint _397 = _388 ? _389 : 8388607u;
                bool _398 = _390 ? _390 : true;
                uint _399 = _390 ? _391 : 1065353216u;
                uint _406 = uint(_347 + 1);
                uint _408 = _406 + (_406 << 10u);
                uint _410 = _408 ^ (_408 >> 6u);
                uint _412 = _410 + (_410 << 3u);
                uint _414 = _412 ^ (_412 >> 11u);
                uint _336 = _360 ? _361 : 1664525u;
                uint _334 = _362 ? _363 : 1013904223u;
                uint _420 = (((_406 * _336) + (_414 + (_414 << 15u))) + _334) * _336;
                uint _422 = _420 ^ (_420 >> 11u);
                uint _425 = _422 ^ ((_422 << 7u) & 2636928640u);
                uint _428 = _425 ^ ((_425 << 15u) & 4022730752u);
                uint _430 = _428 ^ (_428 >> 18u);
                uint _432 = _430 + (_430 << 10u);
                uint _434 = _432 ^ (_432 >> 6u);
                uint _436 = _434 + (_434 << 3u);
                uint _438 = _436 ^ (_436 >> 11u);
                bool _441 = _396 ? _396 : true;
                uint _442 = _396 ? _397 : 8388607u;
                bool _443 = _398 ? _398 : true;
                uint _444 = _398 ? _399 : 1065353216u;
                uint _328 = _441 ? _442 : 8388607u;
                uint _326 = _443 ? _444 : 1065353216u;
                float _457 = (_348 * _348) * (3.0 - (2.0 * _348));
                _297[_322] = (normalize(((vec2(uintBitsToFloat((_377 & _389) | _391) - 1.0, uintBitsToFloat(((_385 + (_385 << 15u)) & _397) | _399) - 1.0) * (1.0 - _457)) + (vec2(uintBitsToFloat((_430 & _442) | _444) - 1.0, uintBitsToFloat(((_438 + (_438 << 15u)) & _328) | _326) - 1.0) * _457)) - vec2(0.5)) * _320) * 3.0;
                _322++;
                _325 = _326;
                _327 = _328;
                _329 = _443 ? _443 : true;
                _331 = _441 ? _441 : true;
                _333 = _334;
                _335 = _336;
                _337 = _362 ? _362 : true;
                _339 = _360 ? _360 : true;
                continue;
            }
            vec3 _468;
            _468 = vec3(0.0);
            for (uint _471 = 0u; _471 < 4u; )
            {
                uint _472 = _471 + 1u;
                float _476 = float(_472) * 10.16640758514404296875;
                float _477 = cos(_476);
                float _478 = sin(_476);
                mat2 _482 = mat2(vec2(_477, _478), vec2(-_478, _477));
                vec2 _509 = _468.yz + (_482 * ((((texture(sampler2D(water_noise_texture, sampler_llr), (((_299.xz * _246[_471]) + _297[_471]) * _482) * 0.01200000010430812835693359375).xyz * vec3(1.0, 2.0, 2.0)) + vec3(-0.5, -1.0, -1.0)) * _247[_471]) * vec2(1.0, _246[_471] * 0.01200000010430812835693359375).xyy).yz);
                _468 = vec3(_468.x, _509.x, _509.y);
                _471 = _472;
                continue;
            }
            _510 = _325;
            _511 = _327;
            _512 = _329;
            _513 = _331;
            _514 = _333;
            _515 = _335;
            _516 = _337;
            _517 = _339;
            _518 = _468;
            break;
        } while(false);
        vec3 _611;
        do
        {
            if (_308)
            {
                _611 = vec3(0.0);
                break;
            }
            uint _525 = min(32u, ripple_source_buffer._m0[0u].count);
            vec3 _527;
            _527 = vec3(0.0);
            for (uint _530 = 0u; _530 < _525; )
            {
                vec2 _544 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_530].x & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_530].x >> 16u) & 65535u).x);
                vec2 _553 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_530].y & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_530].y >> 16u) & 65535u).x);
                vec2 _556 = mix(_544, _553, vec2(0.5));
                vec2 _557 = _556 - _299.xz;
                float _558 = length(_557);
                vec2 _560 = (_553 - _544) * 3.0;
                float _562 = _558 + dot(_557, _560);
                float _570 = (float(frame_constants.game_time_sec) + frame_constants.game_time_frac) * 6.0;
                float _573 = spvNMax(_558, _562);
                float _574 = _573 * _573;
                float _577 = sin((_562 * 16.3649997711181640625) - _570) / (1.0 + (_574 * _574));
                vec2 _579 = _556 - (_299.xz - vec2(0.00999999977648258209228515625, 0.0));
                float _580 = length(_579);
                float _582 = _580 + dot(_579, _560);
                float _586 = spvNMax(_580, _582);
                float _587 = _586 * _586;
                vec2 _593 = _556 - (_299.xz - vec2(0.0, 0.00999999977648258209228515625));
                float _594 = length(_593);
                float _596 = _594 + dot(_593, _560);
                float _600 = spvNMax(_594, _596);
                float _601 = _600 * _600;
                vec3 _606 = vec3(_577, (sin((_582 * 16.3649997711181640625) - _570) / (1.0 + (_587 * _587))) - _577, (sin((_596 * 16.3649997711181640625) - _570) / (1.0 + (_601 * _601))) - _577);
                vec2 _608 = _606.yz * 0.5;
                _527 += (vec3(_606.x, _608.x, _608.y) * uintBitsToFloat(ripple_source_buffer._m0[0u].params[_530].z));
                _530++;
                continue;
            }
            _611 = _527;
            break;
        } while(false);
        vec3 _614 = vec3(_518.y, 1.0, _518.z);
        float _619 = abs(1.0);
        vec2 _623 = _614.xz + (vec3(_611.y, 1.0, _611.z).xz / vec2(_619));
        _626 = _510;
        _627 = _511;
        _628 = _512;
        _629 = _513;
        _630 = _514;
        _631 = _515;
        _632 = _516;
        _633 = _517;
        _634 = normalize(vec3(_623.x, _614.y, _623.y));
        break;
    } while(false);
    float _799;
    vec2 _655 = (((vec2(ivec2((_302 + uvec2(1u)) - _244[frame_constants.frame_index & 3u])) + vec2(0.5)) / vec2(_Globals.framebuffer_dims)) - vec2(0.5)) * vec2(2.0, -2.0);
    float _656 = _655.x;
    vec4 _661 = vec4(_656, _655.y, gl_FragCoord.z, 1.0);
    vec4 _662 = _661 * frame_constants.view_constants[0u].sample_to_view;
    vec4 _663 = _662 * frame_constants.view_constants[0u].view_to_world;
    vec3 _665 = normalize(((vec4(_656, _655.y, 0.0, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world).xyz);
    vec3 _666 = -_665;
    vec4 _669 = vec4(0.0, 0.0, 1.0, 0.0) * frame_constants.view_constants[2u].view_to_world;
    vec3 _670 = _669.xyz;
    float _672 = 1.0 - dot(_666, _634);
    float _673 = _672 * _672;
    float _676 = 0.959999978542327880859375 * ((_673 * _673) * _672);
    float _682 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _692 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_302.x) + _682)) + (0.005837149918079376220703125 * (float(_302.y) + _682))));
    uvec2 _693 = _302 / uvec2(2u);
    vec4 _699 = (vec4(_656, _655.y, texelFetch(half_depth_tex, ivec2(_693), int(0u)).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec2 _713 = vec2(_Globals.half_depth_tex_dims);
    vec3 _714 = _661.xyz;
    vec4 _730 = (vec4(_299 + (normalize(_665 - ((vec3(_634.x, 0.0, _634.z) * 0.5) * 2.0)) * spvNMin(4.0, 4.0 * length(_299 - (_699.xyz / vec3(_699.w))))), 1.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_sample;
    vec3 _734 = _730.xyz / vec3(_730.w);
    vec4 _745 = _661 + (vec4(_734 - _714, 0.0) * float(int(sign(_734.z))));
    float _746 = _745.w;
    vec3 _755 = (_745 / vec4(float((_746 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_746)))).xyz;
    vec3 _756 = _755 - _714;
    vec3 _760 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_756, vec3(0.0))) - _714) / _756;
    vec3 _766 = _714 + (_756 * spvNMax(0.0, spvNMax(_760.x, _760.y)));
    vec3 _767 = _755 - _766;
    vec3 _771 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_767, vec3(0.0))) - _766) / _767;
    vec3 _779 = _766 + (_767 * spvNMin(1.0, spvNMin(spvNMin(_771.x, _771.y), _771.z)));
    float _977;
    do
    {
        _799 = frame_constants.view_constants[0u].clip_to_view[3].z;
        float _800 = 0.100000001490116119384765625 * _799;
        uint _801 = uint(int(max(2u, min(3u, uint(int(floor(length((((_779.xy * vec2(0.5, -0.5)) + vec2(0.5)) - ((_766.xy * vec2(0.5, -0.5)) + vec2(0.5))) * _713))))))));
        float _894;
        float _962;
        float _963;
        float _964;
        float _965;
        bool _966;
        do
        {
            vec3 _806 = _779 - _766;
            float _814;
            if (_779.z > 9.5367431640625e-07)
            {
                _814 = float(_801);
            }
            else
            {
                _814 = spvNMax(1.0, float(_801) - 1.0);
            }
            float _815 = 1.0 / _814;
            float _888;
            float _889;
            float _890;
            float _891;
            float _892;
            bool _893;
            if (_801 > 0u)
            {
                float _821 = mix(0.0, 1.0, pow(_692 * _815, 1.0));
                vec3 _823 = _766 + (_806 * _821);
                float _828 = 1.0 / _823.z;
                vec4 _831 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_823.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _832 = _831.x;
                float _835 = (1.00000202655792236328125 / _832) - _828;
                bool _838 = (_835 < 0.0) ? true : false;
                float _881;
                float _882;
                float _883;
                float _884;
                float _885;
                bool _886;
                float _887;
                if (_838)
                {
                    _881 = 0.0;
                    _882 = _835;
                    _883 = _828 - (1.0 / _832);
                    _884 = _800;
                    _885 = _821;
                    _886 = _838;
                    _887 = 0.0;
                }
                else
                {
                    float _846;
                    float _850;
                    _846 = _835;
                    _850 = _821;
                    float _847;
                    bool _849;
                    float _851;
                    float _876;
                    float _877;
                    float _878;
                    float _879;
                    bool _880;
                    uint _843 = 1u;
                    bool _848 = _838;
                    for (;;)
                    {
                        if (_843 < _801)
                        {
                            _851 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_843) + _692) * _815, 1.0)));
                            vec3 _861 = _766 + (_806 * _851);
                            float _866 = 1.0 / _861.z;
                            vec4 _868 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_861.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                            float _869 = _868.x;
                            _847 = (1.00000202655792236328125 / _869) - _866;
                            _849 = (_847 < 0.0) ? true : false;
                            if (_849)
                            {
                                _876 = _847;
                                _877 = _866 - (1.0 / _869);
                                _878 = _800;
                                _879 = _851;
                                _880 = _849;
                                break;
                            }
                            _843++;
                            _846 = _847;
                            _848 = _849;
                            _850 = _851;
                            continue;
                        }
                        else
                        {
                            _876 = 0.0;
                            _877 = 0.0;
                            _878 = 0.0;
                            _879 = 1.0;
                            _880 = _848;
                            break;
                        }
                    }
                    _881 = _846;
                    _882 = _876;
                    _883 = _877;
                    _884 = _878;
                    _885 = _879;
                    _886 = _880;
                    _887 = _850;
                }
                _888 = _881;
                _889 = _882;
                _890 = _883;
                _891 = _884;
                _892 = _885;
                _893 = _886;
                _894 = _887;
            }
            else
            {
                _888 = 0.0;
                _889 = 0.0;
                _890 = 0.0;
                _891 = 0.0;
                _892 = 1.0;
                _893 = false;
                _894 = 0.0;
            }
            if (_893)
            {
                float _899;
                float _902;
                float _904;
                float _906;
                float _908;
                float _910;
                _899 = _894;
                _902 = _888;
                _904 = _889;
                _906 = _890;
                _908 = _891;
                _910 = _892;
                for (uint _912 = 0u; _912 < 1u; )
                {
                    float _916 = _899 + _910;
                    float _917 = _916 * 0.5;
                    vec3 _919 = _766 + (_806 * _917);
                    float _924 = 1.0 / _919.z;
                    vec4 _927 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_919.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _928 = _927.x;
                    float _931 = (1.00000202655792236328125 / _928) - _924;
                    bool _934 = (_931 < 0.0) ? true : false;
                    _899 = _934 ? _899 : _917;
                    _902 = _934 ? _902 : _931;
                    _904 = _934 ? _931 : _904;
                    _906 = _934 ? (_924 - (1.0 / _928)) : _906;
                    _908 = _934 ? _800 : _908;
                    _910 = _934 ? _917 : _910;
                    _912++;
                    continue;
                }
                float _938 = mix(_899, _910, _902 / (_902 + (-_904)));
                vec3 _940 = _766 + (_806 * _938);
                float _945 = 1.0 / _940.z;
                vec4 _948 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_940.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _949 = _948.x;
                float _952 = (1.00000202655792236328125 / _949) - _945;
                bool _957 = (abs(_952) < (_902 * 0.89999997615814208984375)) ? true : false;
                _962 = _957 ? _938 : _910;
                _963 = _957 ? _952 : _904;
                _964 = _957 ? _800 : _908;
                _965 = _957 ? (_945 - (1.0 / _949)) : _906;
                _966 = true;
                break;
            }
            else
            {
                _962 = _894;
                _963 = _240;
                _964 = _240;
                _965 = _240;
                _966 = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _974;
        if (_966)
        {
            bool _973;
            if (_965 < _964)
            {
                _973 = _963 < _964;
            }
            else
            {
                _973 = false;
            }
            _974 = _973;
        }
        else
        {
            _974 = false;
        }
        if (_974)
        {
            _977 = _962;
            break;
        }
        _977 = _894;
        break;
    } while(false);
    vec3 _979 = mix(_766, _779, vec3(_977));
    float _980 = _979.x;
    vec2 _985 = (vec4(_980, _979.y, _274, 1.0).xy * vec2(0.5, -0.5)) + vec2(0.5);
    vec4 _992 = (vec4(_980, _979.y, textureLod(sampler2D(half_depth_tex, sampler_nnc), _985, 0.0).x, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
    vec3 _996 = _992.xyz / vec3(_992.w);
    float _1005 = _996.y;
    float _1007 = abs(_299.y - _1005);
    ivec2 _1009 = ivec2(_302);
    vec2 _1218;
    do
    {
        vec4 _1017 = vec4(_299, 1.0) * frame_constants.view_constants[2u].world_to_sample;
        if (_1017.z > 1.0)
        {
            _1218 = vec2(0.0);
            break;
        }
        vec2 _1022 = _1017.xy;
        bool _1025 = any(greaterThan(abs(_1022), vec2(0.9833333492279052734375)));
        vec2 _1029;
        if (_1025)
        {
            _1029 = _1022 * 0.5;
        }
        else
        {
            _1029 = _1022;
        }
        float _1030 = _1025 ? 5.0 : 10.0;
        float _1032 = _1025 ? 0.5 : 1.0;
        bool _1038 = any(greaterThan(abs(_1029), vec2(1.0) - (vec2(0.000833333353511989116668701171875) * (2.0 * _1030))));
        float _1044;
        vec2 _1045;
        float _1046;
        if (_1038)
        {
            _1044 = _1030 * 0.2857142984867095947265625;
            _1045 = _1029 * 0.2857142984867095947265625;
            _1046 = _1032 * 0.2857142984867095947265625;
        }
        else
        {
            _1044 = _1030;
            _1045 = _1029;
            _1046 = _1032;
        }
        float _1049 = float(_1038 ? 2u : uint(_1025));
        uint _1051 = 2u + uint(_1049);
        float _1054 = dot(_634, -_670);
        vec4 _1066 = vec4(_1045.x, _1045.y, _1017.z, _1017.w) + (vec4((_634 * (0.066666670143604278564453125 * clamp(1.0 - _1054, 0.0, 1.0))) / vec3(_1046), 0.0) * frame_constants.view_constants[_1051].world_to_sample);
        vec2 _1069 = (_1066.xy * vec2(0.5, -0.5)) + vec2(0.5);
        vec3 _1070 = vec3(_1069.x, _1069.y, _1066.z);
        float _1075 = spvNMax((0.300000011920928955078125 / _1046) * (1.0 - abs(_1054)), 0.02500000037252902984619140625);
        float _1086 = float((uint((_1009.x * 11) + (_1009.y * 7)) + (frame_constants.frame_index * 5u)) % 64u) * 1.61803400516510009765625;
        uvec2 _1087 = uvec2(_1009);
        float _1099 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_1087.x) + _682)) + (0.005837149918079376220703125 * (float(_1087.y) + _682))));
        vec4 _1103 = vec4(_634, 0.0) * frame_constants.view_constants[_1051].world_to_sample;
        float _1105;
        int _1108;
        _1105 = 0.0;
        _1108 = 0;
        float _1106;
        int _1109;
        for (int _1110 = 0; _1110 < 5; _1105 = _1106, _1108 = _1109, _1110++)
        {
            float _1115 = float(_1110);
            float _1117 = (_1115 + _1086) * 2.3999631404876708984375;
            vec3 _1130 = _1070 + vec3((((vec2(cos(_1117), sin(_1117)) * (_1115 + _1099)) * vec2(0.20000000298023223876953125)) * _1044) * vec2(0.000833333353511989116668701171875), _1075 * (-0.0009765625));
            vec4 _1137 = textureLod(sampler2DArray(shadow_map, sampler_shadow_nearest), vec3(_1130.xy, _1049), 0.0);
            float _1138 = _1137.x;
            if (_1130.z > _1138)
            {
                _1106 = _1105 + _1138;
                _1109 = _1108 + 1;
            }
            else
            {
                _1106 = _1105;
                _1109 = _1108;
            }
        }
        float _1146 = _1105 / float(_1108);
        if (_1108 == 0)
        {
            _1218 = vec2(0.0);
            break;
        }
        float _1160 = spvNMax(((_1066.z - _1146) * ((17.0666675567626953125 * _1044) * pow(10.0, smoothstep(0.0, 0.25, frame_constants.fog_tint_thickness.w)))) / _1146, 0.0);
        float _1167 = (_1160 / pow(pow(_1160 / (_1044 - 1.0), 5.0) + 1.0, 0.20000000298023223876953125)) + 1.0;
        float _1169 = _1103.z;
        vec2 _1179 = (((_1103.xy * float(int(sign(_1169)))) / vec2(spvNMax(0.1500000059604644775390625, abs(_1169)))) * vec2(0.0009765625)) * 2.0;
        vec2 _1181;
        _1181 = vec2(0.0);
        for (int _1184 = 0; _1184 < 10; )
        {
            float _1188 = float(_1184);
            float _1190 = (_1188 + _1086) * 2.3999631404876708984375;
            vec2 _1197 = ((vec2(cos(_1190), sin(_1190)) * (_1188 + _1099)) * vec2(0.100000001490116119384765625)) * _1167;
            vec3 _1203 = _1070 + vec3(_1197 * vec2(0.000833333353511989116668701171875), _1075 * (-0.0009765625));
            _1181 += vec2(1.0 - textureGrad(sampler2DArrayShadow(shadow_map, sampler_shadow_linear), vec4(vec3(_1203.xy, _1049), _1203.z + spvNMin(0.0, dot(_1179, _1197))), vec2(0.0), vec2(0.0)), _286);
            _1184++;
            continue;
        }
        _1218 = _1181 * 0.100000001490116119384765625;
        break;
    } while(false);
    float _1230 = autofocus_buf._m0[frame_constants.camera_idx].focus_scale;
    float _1231 = 1.0 / autofocus_buf._m0[frame_constants.camera_idx].focus0;
    float _1232 = 1.0 / autofocus_buf._m0[frame_constants.camera_idx].focus1;
    float _1235 = spvNMax(0.001000000047497451305389404296875, spvNMax(_1231, _1232)) + 9.9999997473787516355514526367188e-06;
    float _1238 = spvNMax(0.001000000047497451305389404296875, spvNMin(_1231, _1232)) - 9.9999997473787516355514526367188e-06;
    bool _1239 = autofocus_buf._m0[frame_constants.camera_idx].use_tilt_shift != 0u;
    vec3 _1243 = _299;
    _1243.y = _299.y * float(_1239 ? 0 : 1);
    vec4 _1244 = vec4(0.0, 0.0, 0.0, 1.0) * frame_constants.view_constants[0u].view_to_world;
    vec3 _1252 = normalize((vec4(0.0, 0.0, -1.0, 0.0) * frame_constants.view_constants[0u].view_to_world).xyz);
    float _1255 = 1.0 / spvNMax(1.0000000133514319600180897396058e-10, dot(_1243 - (_1244.xyz / vec3(_1244.w)), _1252));
    float _1256 = _1235 - _1238;
    float _1258 = spvNMin(0.00999999977648258209228515625, 0.0500000007450580596923828125 * _1256);
    float _1278;
    if (_1255 > _1235)
    {
        _1278 = (_1255 - _1235) + _1258;
    }
    else
    {
        float _1275;
        if (_1255 < _1238)
        {
            _1275 = (_1255 - _1238) - _1258;
        }
        else
        {
            _1275 = (-_1258) + (((2.0 * _1258) * (_1255 - _1238)) / _1256);
        }
        _1278 = _1275;
    }
    float _1285;
    if (_1239)
    {
        _1285 = _1278 / spvNMax(0.00999999977648258209228515625, length(_1252.xz));
    }
    else
    {
        _1285 = _1278;
    }
    float _1304 = sqrt(mix(0.16000001132488250732421875, 9.9999997473787516355514526367188e-05, exp2((-0.014999999664723873138427734375) * sqrt(spvNMax(abs(((-_1285) * _1230) * frame_constants.view_constants[0u].view_to_clip[1].y), (_662.xyz / vec3(_662.w)).z * (-0.0030000000260770320892333984375))))));
    vec3 _1306 = -reflect(_666, _634);
    float _1307 = _1306.y;
    vec3 _1313;
    if (_1307 < 0.0)
    {
        vec3 _1312 = _1306;
        _1312.y = -_1307;
        _1313 = _1312;
    }
    else
    {
        _1313 = _1306;
    }
    vec4 _1320 = _661 + ((vec4(_1313, 0.0) * frame_constants.view_constants[0u].world_to_view) * frame_constants.view_constants[0u].view_to_clip);
    float _1321 = _1320.w;
    vec3 _1330 = (_1320 / vec4(float((_1321 >= 0.0) ? 1 : (-1)) * spvNMax(1.0000000133514319600180897396058e-10, abs(_1321)))).xyz;
    vec3 _1331 = _1330 - _714;
    vec3 _1335 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), lessThan(_1331, vec3(0.0))) - _714) / _1331;
    vec3 _1341 = _714 + (_1331 * spvNMax(0.0, spvNMax(_1335.x, _1335.y)));
    vec3 _1342 = _1330 - _1341;
    vec3 _1346 = (mix(vec3(-1.0, -1.0, 0.0), vec3(1.0), greaterThanEqual(_1342, vec3(0.0))) - _1341) / _1342;
    vec3 _1353 = _1341 + (_1342 * spvNMin(spvNMin(_1346.x, _1346.y), _1346.z));
    bool _1544;
    float _1545;
    do
    {
        uint _1371 = uint(int(max(2u, min(8u, uint(int(floor(length((((_1353.xy * vec2(0.5, -0.5)) + vec2(0.5)) - ((_1341.xy * vec2(0.5, -0.5)) + vec2(0.5))) * _713))))))));
        float _1463;
        float _1529;
        float _1530;
        float _1531;
        float _1532;
        bool _1533;
        do
        {
            vec3 _1376 = _1353 - _1341;
            float _1384;
            if (_1353.z > 9.5367431640625e-07)
            {
                _1384 = float(_1371);
            }
            else
            {
                _1384 = spvNMax(1.0, float(_1371) - 1.0);
            }
            float _1385 = 1.0 / _1384;
            float _1457;
            float _1458;
            float _1459;
            float _1460;
            float _1461;
            bool _1462;
            if (_1371 > 0u)
            {
                float _1391 = mix(0.0, 1.0, pow(_692 * _1385, 1.0));
                vec3 _1393 = _1341 + (_1376 * _1391);
                float _1398 = 1.0 / _1393.z;
                vec4 _1400 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_1393.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _1401 = _1400.x;
                float _1404 = (1.00000202655792236328125 / _1401) - _1398;
                bool _1407 = (_1404 < 0.0) ? true : false;
                float _1450;
                float _1451;
                float _1452;
                float _1453;
                float _1454;
                bool _1455;
                float _1456;
                if (_1407)
                {
                    _1450 = 0.0;
                    _1451 = _1404;
                    _1452 = _1398 - (1.0 / _1401);
                    _1453 = _799;
                    _1454 = _1391;
                    _1455 = _1407;
                    _1456 = 0.0;
                }
                else
                {
                    float _1415;
                    float _1419;
                    _1415 = _1404;
                    _1419 = _1391;
                    float _1416;
                    bool _1418;
                    float _1420;
                    float _1445;
                    float _1446;
                    float _1447;
                    float _1448;
                    bool _1449;
                    uint _1412 = 1u;
                    bool _1417 = _1407;
                    for (;;)
                    {
                        if (_1412 < _1371)
                        {
                            _1420 = mix(0.0, 1.0, spvNMin(0.9999847412109375, pow((float(_1412) + _692) * _1385, 1.0)));
                            vec3 _1430 = _1341 + (_1376 * _1420);
                            float _1435 = 1.0 / _1430.z;
                            vec4 _1437 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_1430.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                            float _1438 = _1437.x;
                            _1416 = (1.00000202655792236328125 / _1438) - _1435;
                            _1418 = (_1416 < 0.0) ? true : false;
                            if (_1418)
                            {
                                _1445 = _1416;
                                _1446 = _1435 - (1.0 / _1438);
                                _1447 = _799;
                                _1448 = _1420;
                                _1449 = _1418;
                                break;
                            }
                            _1412++;
                            _1415 = _1416;
                            _1417 = _1418;
                            _1419 = _1420;
                            continue;
                        }
                        else
                        {
                            _1445 = 0.0;
                            _1446 = 0.0;
                            _1447 = 0.0;
                            _1448 = 1.0;
                            _1449 = _1417;
                            break;
                        }
                    }
                    _1450 = _1415;
                    _1451 = _1445;
                    _1452 = _1446;
                    _1453 = _1447;
                    _1454 = _1448;
                    _1455 = _1449;
                    _1456 = _1419;
                }
                _1457 = _1450;
                _1458 = _1451;
                _1459 = _1452;
                _1460 = _1453;
                _1461 = _1454;
                _1462 = _1455;
                _1463 = _1456;
            }
            else
            {
                _1457 = 0.0;
                _1458 = 0.0;
                _1459 = 0.0;
                _1460 = 0.0;
                _1461 = 1.0;
                _1462 = false;
                _1463 = 0.0;
            }
            if (_1462)
            {
                float _1468;
                float _1471;
                float _1473;
                float _1475;
                float _1477;
                float _1479;
                _1468 = _1463;
                _1471 = _1457;
                _1473 = _1458;
                _1475 = _1459;
                _1477 = _1460;
                _1479 = _1461;
                for (uint _1481 = 0u; _1481 < 3u; )
                {
                    float _1485 = _1468 + _1479;
                    float _1486 = _1485 * 0.5;
                    vec3 _1488 = _1341 + (_1376 * _1486);
                    float _1493 = 1.0 / _1488.z;
                    vec4 _1495 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_1488.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                    float _1496 = _1495.x;
                    float _1499 = (1.00000202655792236328125 / _1496) - _1493;
                    bool _1502 = (_1499 < 0.0) ? true : false;
                    _1468 = _1502 ? _1468 : _1486;
                    _1471 = _1502 ? _1471 : _1499;
                    _1473 = _1502 ? _1499 : _1473;
                    _1475 = _1502 ? (_1493 - (1.0 / _1496)) : _1475;
                    _1477 = _1502 ? _799 : _1477;
                    _1479 = _1502 ? _1486 : _1479;
                    _1481++;
                    continue;
                }
                float _1506 = mix(_1468, _1479, _1471 / (_1471 + (-_1473)));
                vec3 _1508 = _1341 + (_1376 * _1506);
                float _1513 = 1.0 / _1508.z;
                vec4 _1515 = textureLod(sampler2D(half_depth_tex, sampler_nnc), (_1508.xy * vec2(0.5, -0.5)) + vec2(0.5), 0.0);
                float _1516 = _1515.x;
                float _1519 = (1.00000202655792236328125 / _1516) - _1513;
                bool _1524 = (abs(_1519) < (_1471 * 0.89999997615814208984375)) ? true : false;
                _1529 = _1524 ? _1506 : _1479;
                _1530 = _1524 ? _1519 : _1473;
                _1531 = _1524 ? _799 : _1477;
                _1532 = _1524 ? (_1513 - (1.0 / _1516)) : _1475;
                _1533 = true;
                break;
            }
            else
            {
                _1529 = _1463;
                _1530 = _240;
                _1531 = _240;
                _1532 = _240;
                _1533 = false;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _1541;
        if (_1533)
        {
            bool _1540;
            if (_1532 < _1531)
            {
                _1540 = _1530 < _1531;
            }
            else
            {
                _1540 = false;
            }
            _1541 = _1540;
        }
        else
        {
            _1541 = false;
        }
        if (_1541)
        {
            _1544 = true;
            _1545 = _1529;
            break;
        }
        _1544 = false;
        _1545 = _1463;
        break;
    } while(false);
    float _1546 = subgroupQuadSwapHorizontal(_1545);
    float _1548 = subgroupQuadSwapHorizontal(float(_1544));
    bool _1554 = (int(_1548 != 0.0) & int(!_1544)) != 0;
    bool _1555 = _1554 ? true : _1544;
    float _1556 = _1554 ? _1546 : _1545;
    float _1557 = subgroupQuadSwapVertical(_1556);
    float _1559 = subgroupQuadSwapVertical(float(_1555));
    bool _1565 = (int(_1559 != 0.0) & int(!_1555)) != 0;
    float _1566 = _1565 ? _1557 : _1556;
    vec3 _1685;
    if (_1565 ? true : _1555)
    {
        vec2 _1662 = (mix(_1341, _1353, vec3(_1566)).xy * vec2(0.5, -0.5)) + vec2(0.5);
        bool _1675;
        if (_Globals.history_valid != 0u)
        {
            _1675 = !(0u == frame_constants.frame_index);
        }
        else
        {
            _1675 = false;
        }
        vec3 _1684;
        if (_1675)
        {
            _1684 = textureLod(sampler2D(taa_history_tex, sampler_lnc), textureLod(sampler2D(reprojection_tex, sampler_nnc), _1662, 0.0).xy + _1662, 0.0).xyz;
        }
        else
        {
            _1684 = vec3(0.0);
        }
        _1685 = _1684;
    }
    else
    {
        vec3 _1572 = mix(_1341, _1353, vec3(_1566));
        vec4 _1578 = (vec4(_1572, 1.0) * frame_constants.view_constants[0u].sample_to_view) * frame_constants.view_constants[0u].view_to_world;
        uint _1593;
        if (_298)
        {
            _1593 = 0u;
        }
        else
        {
            uvec2 _1586 = _302 & uvec2(1u);
            _1593 = 1u << ((_1586.x + (_1586.y * 2u)) & 31u);
        }
        uint _1594 = subgroupQuadSwapHorizontal(_1593);
        uint _1595 = _1593 + _1594;
        uint _1596 = subgroupQuadSwapVertical(_1595);
        uint _1597 = _1595 + _1596;
        uvec2 _1598 = _302 & uvec2(1u);
        if ((_1598.x + (_1598.y * 2u)) == uint(findMSB(_1597)))
        {
            uint _1608 = atomicAdd(_alias_water_rt_count_buffer._m0[0u], 1u);
            vec3 _1620 = _634 / vec3((abs(_634.x) + abs(_634.y)) + abs(_634.z));
            vec3 _1634;
            if (_1620.z < 0.0)
            {
                vec2 _1632 = (vec2(1.0) - abs(_1620.yx)) * ((step(vec2(0.0), _1620.xy) * 2.0) - vec2(1.0));
                _1634 = vec3(_1632.x, _1632.y, _1620.z);
            }
            else
            {
                _1634 = _1620;
            }
            vec2 _1637 = (_1634.xy * 0.5) + vec2(0.5);
            _alias_water_rt_ray_buffer._m0[_1608] = uvec4(_693.x + (_693.y * 65536u), packHalf2x16(vec2(_1637.x, 0.0)) | (packHalf2x16(vec2(_1637.y, 0.0)) << 16u), floatBitsToUint(dot(_1313, (_1578.xyz / vec3(_1578.w)) - (_663.xyz / vec3(_663.w)))), (floatBitsToUint(gl_FragCoord.z) & 4294967280u) + _1597);
        }
        _1685 = vec3(0.0);
    }
    float _1689;
    float _1686 = _1304 * _1304;
    vec3 _1743;
    do
    {
        _1689 = dot(_634, _670);
        bool _1696;
        if (!(_1689 <= 0.0))
        {
            _1696 = dot(_634, _666) <= 0.0;
        }
        else
        {
            _1696 = true;
        }
        if (_1696)
        {
            _1743 = vec3(0.0);
            break;
        }
        float _1699 = _1686 * _1686;
        vec3 _1701 = normalize(_666 + _670);
        float _1702 = dot(_634, _1701);
        float _1705 = spvNMax(0.0, 1.0 - dot(_1701, _670));
        float _1706 = _1705 * _1705;
        float _1711 = dot(_634, _666);
        float _1734 = ((_1702 * _1702) * (_1699 - 1.0)) + 1.0;
        _1743 = ((mix(vec3(0.039999999105930328369140625), vec3(1.0), vec3((_1706 * _1706) * _1705)) * (((2.0 * _1689) * _1711) / ((_1689 * sqrt(((((-_1711) * _1699) + _1711) * _1711) + _1699)) + (_1711 * sqrt(((((-_1689) * _1699) + _1689) * _1689) + _1699))))) * (_1699 / ((3.1415927410125732421875 * _1734) * _1734))) / vec3((4.0 * _1711) * _1689);
        break;
    } while(false);
    vec2 _1773 = (_996 - ((_670 * _1005) / vec3(_669.y))).xz;
    uint _1973;
    uint _1974;
    bool _1975;
    bool _1976;
    uint _1977;
    uint _1978;
    bool _1979;
    bool _1980;
    vec3 _1981;
    do
    {
        if (_308)
        {
            _1973 = _626;
            _1974 = _627;
            _1975 = _628;
            _1976 = _629;
            _1977 = _630;
            _1978 = _631;
            _1979 = _632;
            _1980 = _633;
            _1981 = vec3(0.0);
            break;
        }
        float _1783 = float(frame_constants.game_time_sec) + frame_constants.game_time_frac;
        uint _1788;
        uint _1790;
        bool _1792;
        bool _1794;
        uint _1796;
        uint _1798;
        bool _1800;
        bool _1802;
        _1788 = _626;
        _1790 = _627;
        _1792 = _628;
        _1794 = _629;
        _1796 = _630;
        _1798 = _631;
        _1800 = _632;
        _1802 = _633;
        vec2 _293[4];
        for (uint _1785 = 0u; _1785 < 4u; )
        {
            float _1808 = float(_248[_1785]);
            int _1810 = int(floor(_1808));
            float _1811 = fract(_1808);
            uint _1812 = uint(_1810);
            uint _1814 = _1812 + (_1812 << 10u);
            uint _1816 = _1814 ^ (_1814 >> 6u);
            uint _1818 = _1816 + (_1816 << 3u);
            uint _1820 = _1818 ^ (_1818 >> 11u);
            bool _1823 = _1802 ? _1802 : true;
            uint _1824 = _1802 ? _1798 : 1664525u;
            bool _1825 = _1800 ? _1800 : true;
            uint _1826 = _1800 ? _1796 : 1013904223u;
            uint _1830 = (((_1812 * _1824) + (_1820 + (_1820 << 15u))) + _1826) * _1824;
            uint _1832 = _1830 ^ (_1830 >> 11u);
            uint _1835 = _1832 ^ ((_1832 << 7u) & 2636928640u);
            uint _1838 = _1835 ^ ((_1835 << 15u) & 4022730752u);
            uint _1840 = _1838 ^ (_1838 >> 18u);
            uint _1842 = _1840 + (_1840 << 10u);
            uint _1844 = _1842 ^ (_1842 >> 6u);
            uint _1846 = _1844 + (_1844 << 3u);
            uint _1848 = _1846 ^ (_1846 >> 11u);
            bool _1851 = _1794 ? _1794 : true;
            uint _1852 = _1794 ? _1790 : 8388607u;
            bool _1853 = _1792 ? _1792 : true;
            uint _1854 = _1792 ? _1788 : 1065353216u;
            bool _1859 = _1851 ? _1851 : true;
            uint _1860 = _1851 ? _1852 : 8388607u;
            bool _1861 = _1853 ? _1853 : true;
            uint _1862 = _1853 ? _1854 : 1065353216u;
            uint _1869 = uint(_1810 + 1);
            uint _1871 = _1869 + (_1869 << 10u);
            uint _1873 = _1871 ^ (_1871 >> 6u);
            uint _1875 = _1873 + (_1873 << 3u);
            uint _1877 = _1875 ^ (_1875 >> 11u);
            uint _1799 = _1823 ? _1824 : 1664525u;
            uint _1797 = _1825 ? _1826 : 1013904223u;
            uint _1883 = (((_1869 * _1799) + (_1877 + (_1877 << 15u))) + _1797) * _1799;
            uint _1885 = _1883 ^ (_1883 >> 11u);
            uint _1888 = _1885 ^ ((_1885 << 7u) & 2636928640u);
            uint _1891 = _1888 ^ ((_1888 << 15u) & 4022730752u);
            uint _1893 = _1891 ^ (_1891 >> 18u);
            uint _1895 = _1893 + (_1893 << 10u);
            uint _1897 = _1895 ^ (_1895 >> 6u);
            uint _1899 = _1897 + (_1897 << 3u);
            uint _1901 = _1899 ^ (_1899 >> 11u);
            bool _1904 = _1859 ? _1859 : true;
            uint _1905 = _1859 ? _1860 : 8388607u;
            bool _1906 = _1861 ? _1861 : true;
            uint _1907 = _1861 ? _1862 : 1065353216u;
            uint _1791 = _1904 ? _1905 : 8388607u;
            uint _1789 = _1906 ? _1907 : 1065353216u;
            float _1920 = (_1811 * _1811) * (3.0 - (2.0 * _1811));
            _293[_1785] = (normalize(((vec2(uintBitsToFloat((_1840 & _1852) | _1854) - 1.0, uintBitsToFloat(((_1848 + (_1848 << 15u)) & _1860) | _1862) - 1.0) * (1.0 - _1920)) + (vec2(uintBitsToFloat((_1893 & _1905) | _1907) - 1.0, uintBitsToFloat(((_1901 + (_1901 << 15u)) & _1791) | _1789) - 1.0) * _1920)) - vec2(0.5)) * _1783) * 3.0;
            _1785++;
            _1788 = _1789;
            _1790 = _1791;
            _1792 = _1906 ? _1906 : true;
            _1794 = _1904 ? _1904 : true;
            _1796 = _1797;
            _1798 = _1799;
            _1800 = _1825 ? _1825 : true;
            _1802 = _1823 ? _1823 : true;
            continue;
        }
        vec3 _1931;
        _1931 = vec3(0.0);
        for (uint _1934 = 0u; _1934 < 4u; )
        {
            uint _1935 = _1934 + 1u;
            float _1939 = float(_1935) * 10.16640758514404296875;
            float _1940 = cos(_1939);
            float _1941 = sin(_1939);
            vec3 _1972;
            _1972.x = _1931.x + ((((texture(sampler2D(water_noise_texture, sampler_llr), (((_1773 * _246[_1934]) + _293[_1934]) * mat2(vec2(_1940, _1941), vec2(-_1941, _1940))) * 0.01200000010430812835693359375).xyz * vec3(1.0, 2.0, 2.0)) + vec3(-0.5, -1.0, -1.0)) * _247[_1934]) * vec2(1.0, _246[_1934] * 0.01200000010430812835693359375).xyy).x;
            _1931 = vec3(_1972.x, _288.x, _288.y);
            _1934 = _1935;
            continue;
        }
        _1973 = _1788;
        _1974 = _1790;
        _1975 = _1792;
        _1976 = _1794;
        _1977 = _1796;
        _1978 = _1798;
        _1979 = _1800;
        _1980 = _1802;
        _1981 = _1931;
        break;
    } while(false);
    vec3 _2044;
    do
    {
        if (_308)
        {
            _2044 = vec3(0.0);
            break;
        }
        uint _1988 = min(32u, ripple_source_buffer._m0[0u].count);
        vec3 _1990;
        _1990 = vec3(0.0);
        for (uint _1993 = 0u; _1993 < _1988; )
        {
            vec2 _2007 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_1993].x & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_1993].x >> 16u) & 65535u).x);
            vec2 _2016 = vec2(unpackHalf2x16(ripple_source_buffer._m0[0u].params[_1993].y & 65535u).x, unpackHalf2x16((ripple_source_buffer._m0[0u].params[_1993].y >> 16u) & 65535u).x);
            vec2 _2020 = mix(_2007, _2016, vec2(0.5)) - _1773;
            float _2021 = length(_2020);
            float _2025 = _2021 + dot(_2020, (_2016 - _2007) * 3.0);
            float _2036 = spvNMax(_2021, _2025);
            float _2037 = _2036 * _2036;
            _1990 += (vec3(vec3(sin((_2025 * 16.3649997711181640625) - ((float(frame_constants.game_time_sec) + frame_constants.game_time_frac) * 6.0)) / (1.0 + (_2037 * _2037)), _286, _286).x, _288.x, _288.y) * uintBitsToFloat(ripple_source_buffer._m0[0u].params[_1993].z));
            _1993++;
            continue;
        }
        _2044 = _1990;
        break;
    } while(false);
    float _2048 = 0.60000002384185791015625 + ((0.4000000059604644775390625 * (exp2((-0.4000000059604644775390625) * _1007) * smoothstep(0.0, 1.0, _1007))) * (_1981 + _2044).x);
    vec3 _2251;
    if (_308)
    {
        vec2 _2064 = _299.xz * 1.0;
        ivec2 _2066 = ivec2(floor(_2064));
        vec2 _2067 = fract(_2064);
        uvec2 _2068 = uvec2(_2066);
        uint _2070 = _2068.y;
        uint _2072 = _2070 + (_2070 << 10u);
        uint _2074 = _2072 ^ (_2072 >> 6u);
        uint _2076 = _2074 + (_2074 << 3u);
        uint _2078 = _2076 ^ (_2076 >> 11u);
        bool _2081 = _1980 ? _1980 : true;
        uint _2082 = _1980 ? _1978 : 1664525u;
        bool _2083 = _1979 ? _1979 : true;
        uint _2084 = _1979 ? _1977 : 1013904223u;
        uint _2088 = (((_2068.x * _2082) + (_2078 + (_2078 << 15u))) + _2084) * _2082;
        uint _2090 = _2088 ^ (_2088 >> 11u);
        uint _2093 = _2090 ^ ((_2090 << 7u) & 2636928640u);
        uint _2096 = _2093 ^ ((_2093 << 15u) & 4022730752u);
        bool _2099 = _1976 ? _1976 : true;
        uint _2100 = _1976 ? _1974 : 8388607u;
        bool _2101 = _1975 ? _1975 : true;
        uint _2102 = _1975 ? _1973 : 1065353216u;
        uvec2 _2108 = uvec2(_2066 + ivec2(1, 0));
        uint _2110 = _2108.y;
        uint _2112 = _2110 + (_2110 << 10u);
        uint _2114 = _2112 ^ (_2112 >> 6u);
        uint _2116 = _2114 + (_2114 << 3u);
        uint _2118 = _2116 ^ (_2116 >> 11u);
        bool _2121 = _2081 ? _2081 : true;
        uint _2122 = _2081 ? _2082 : 1664525u;
        bool _2123 = _2083 ? _2083 : true;
        uint _2124 = _2083 ? _2084 : 1013904223u;
        uint _2128 = (((_2108.x * _2122) + (_2118 + (_2118 << 15u))) + _2124) * _2122;
        uint _2130 = _2128 ^ (_2128 >> 11u);
        uint _2133 = _2130 ^ ((_2130 << 7u) & 2636928640u);
        uint _2136 = _2133 ^ ((_2133 << 15u) & 4022730752u);
        bool _2139 = _2099 ? _2099 : true;
        uint _2140 = _2099 ? _2100 : 8388607u;
        bool _2141 = _2101 ? _2101 : true;
        uint _2142 = _2101 ? _2102 : 1065353216u;
        uvec2 _2148 = uvec2(_2066 + ivec2(0, 1));
        uint _2150 = _2148.y;
        uint _2152 = _2150 + (_2150 << 10u);
        uint _2154 = _2152 ^ (_2152 >> 6u);
        uint _2156 = _2154 + (_2154 << 3u);
        uint _2158 = _2156 ^ (_2156 >> 11u);
        uint _2162 = _2121 ? _2122 : 1664525u;
        uint _2164 = _2123 ? _2124 : 1013904223u;
        uint _2168 = (((_2148.x * _2162) + (_2158 + (_2158 << 15u))) + _2164) * _2162;
        uint _2170 = _2168 ^ (_2168 >> 11u);
        uint _2173 = _2170 ^ ((_2170 << 7u) & 2636928640u);
        uint _2176 = _2173 ^ ((_2173 << 15u) & 4022730752u);
        uint _2180 = _2139 ? _2140 : 8388607u;
        uint _2182 = _2141 ? _2142 : 1065353216u;
        uvec2 _2188 = uvec2(_2066 + ivec2(1));
        uint _2190 = _2188.y;
        uint _2192 = _2190 + (_2190 << 10u);
        uint _2194 = _2192 ^ (_2192 >> 6u);
        uint _2196 = _2194 + (_2194 << 3u);
        uint _2198 = _2196 ^ (_2196 >> 11u);
        uint _2201 = (_2121 ? _2121 : true) ? _2162 : 1664525u;
        uint _2206 = (((_2188.x * _2201) + (_2198 + (_2198 << 15u))) + ((_2123 ? _2123 : true) ? _2164 : 1013904223u)) * _2201;
        uint _2208 = _2206 ^ (_2206 >> 11u);
        uint _2211 = _2208 ^ ((_2208 << 7u) & 2636928640u);
        uint _2214 = _2211 ^ ((_2211 << 15u) & 4022730752u);
        vec2 _2226 = (_2067 * _2067) * (vec2(3.0) - (_2067 * 2.0));
        float _2227 = _2226.x;
        float _2228 = 1.0 - _2227;
        float _2230 = _2226.y;
        float _2240 = (((1.0 - _2230) * (((uintBitsToFloat(((_2096 ^ (_2096 >> 18u)) & _2100) | _2102) - 1.0) * _2228) + ((uintBitsToFloat(((_2136 ^ (_2136 >> 18u)) & _2140) | _2142) - 1.0) * _2227))) + (((uintBitsToFloat(((_2176 ^ (_2176 >> 18u)) & _2180) | _2182) - 1.0) * _2228) * _2230)) + (((uintBitsToFloat(((_2214 ^ (_2214 >> 18u)) & ((_2139 ? _2139 : true) ? _2180 : 8388607u)) | ((_2141 ? _2141 : true) ? _2182 : 1065353216u)) - 1.0) * _2227) * _2230);
        _2251 = mix(vec3(0.0), vec3(2.0), vec3(clamp((1.0 - smoothstep(0.699999988079071044921875, 0.85000002384185791015625, textureLod(sampler2D(water_mask, sampler_llc), (_299.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0).y + ((_2240 * _2240) * 0.20000000298023223876953125))) * 0.20000000298023223876953125, 0.0, 1.0) + 0.100000001490116119384765625));
    }
    else
    {
        _2251 = vec3(0.0);
    }
    out_var_SV_TARGET = vec4(((((exp((-_Globals.water_color) * ((_1007 + length(_996 - _299)) + 0.699999988079071044921875)) * (0.959999978542327880859375 - _676)) * textureLod(sampler2D(refraction_tex, sampler_lnc), _985, 0.0).xyz) * (1.0 + ((_2048 * _2048) * _2048))) + ((_1685 + ((frame_constants.sun_color.xyz * spvNMin(30.0, _1743.x)) * ((1.0 - _1218.x) * clamp(_1689, 0.0, 1.0)))) * (0.039999999105930328369140625 + _676))) + _2251, 0.0);
}

