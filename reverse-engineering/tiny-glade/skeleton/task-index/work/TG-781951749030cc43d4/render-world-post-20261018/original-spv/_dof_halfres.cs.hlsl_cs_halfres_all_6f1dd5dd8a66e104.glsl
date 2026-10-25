#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_EXT_samplerless_texture_functions : require
#if defined(GL_EXT_control_flow_attributes)
#extension GL_EXT_control_flow_attributes : require
#define SPIRV_CROSS_FLATTEN [[flatten]]
#define SPIRV_CROSS_BRANCH [[dont_flatten]]
#define SPIRV_CROSS_UNROLL [[unroll]]
#define SPIRV_CROSS_LOOP [[dont_unroll]]
#else
#define SPIRV_CROSS_FLATTEN
#define SPIRV_CROSS_BRANCH
#define SPIRV_CROSS_UNROLL
#define SPIRV_CROSS_LOOP
#endif
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_shuffle : require
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

struct DofTileOffsets
{
    uint offsets[8];
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
    layout(offset = 0) float bokeh_anisotropy;
    layout(offset = 4) uvec2 blue_noise_texture_dims;
    layout(offset = 16) uvec2 color_tex_dims;
    layout(offset = 24) uvec2 coc_tiles_dilated_tex_dims;
    layout(offset = 32) uvec2 depth_tex_dims;
    layout(offset = 40) uvec2 coc_tex_dims;
    layout(offset = 48) uvec2 img_output_dims;
} _Globals;

layout(set = 0, binding = 20, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 21, std430) readonly buffer type_StructuredBuffer_v2uint
{
    uvec2 _m0[];
} tile_coords_buffer;

layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 14) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 15) uniform texture2D color_tex;
layout(set = 0, binding = 16) uniform texture2D coc_tiles_dilated_tex;
layout(set = 0, binding = 18) uniform texture2D coc_tex;
layout(set = 0, binding = 19, rgba16f) uniform writeonly image2D img_output;

shared uint ring_samples[64][9];

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
    uvec2 _188 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[5] + gl_WorkGroupID.x]);
    vec2 _195 = (vec2(ivec2(_188)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _206 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _195, 0.0).x);
    vec4 _222 = ((texelFetch(blue_noise_texture, ivec2(((_188 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    uvec2 _223 = uvec2(7u) + _188;
    float _235 = (fract(52.98291778564453125 * fract((0.067110560834407806396484375 * float(_223.x)) + (0.005837149918079376220703125 * float(_223.y)))) - 0.5) * 0.5;
    float _236 = 0.5 + _235;
    float _237 = _222.y;
    float _238 = _206 * 0.125;
    float _243 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _253 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_188.x) + _243)) + (0.005837149918079376220703125 * (float(_188.y) + _243))));
    bool _254 = _236 < 0.5;
    float _258;
    if (_254)
    {
        _258 = 0.5 - _235;
    }
    else
    {
        _258 = _236;
    }
    float _259 = _254 ? (-1.0) : 1.0;
    vec4 _261;
    float _264;
    float _266;
    float _268;
    _261 = vec4(0.0);
    _264 = 0.0;
    _266 = 0.0;
    _268 = 0.0;
    vec4 _262;
    float _265;
    float _273;
    float _275;
    uint _170[8];
    float _267;
    float _269;
    uint _270 = 0u;
    float _272 = 10000000000.0;
    float _274 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _270 < 2u; _261 = _262, _264 = _265, _266 = _267, _268 = _269, _270++, _272 = _273, _274 = _275)
    {
        float _281 = (float(_270) + _253) * 1.57079637050628662109375;
        vec2 _284 = vec2(cos(_281), sin(_281));
        vec2 _322;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _297 = vec4((_195 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _301 = _297.xy / vec2(_297.z);
            _301.y = _301.y * (-1.0);
            vec2 _305 = normalize(_301);
            mat2 _310 = mat2(_305, vec2(-_305.y, _305.x));
            float _315 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_301, _301)));
            _322 = ((_284 * _310) * vec2(_315, 1.0 / _315)) * transpose(_310);
        }
        else
        {
            _322 = _284;
        }
        vec2 _327 = _322 * (vec2(_259) / vec2(_Globals.color_tex_dims));
        float _336 = -_206;
        float _337 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-8.0) + _237) * _238)), 0.0).x, _336, _206);
        float _338 = abs(_337);
        float _341 = _222.x * 0.20000000298023223876953125;
        float _342 = mix(_264, spvNMax(_264, _338), _341);
        float _354 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-7.0) + _237) * _238)), 0.0).x, _336, _206);
        float _357 = mix(_342, spvNMax(_342, abs(_354)), _341);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_337, 0.0)) | (packHalf2x16(vec2(_354, 0.0)) << 16u);
        float _370 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-6.0) + _237) * _238)), 0.0).x, _336, _206);
        float _371 = abs(_370);
        float _373 = mix(_357, spvNMax(_357, _371), _341);
        float _385 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-5.0) + _237) * _238)), 0.0).x, _336, _206);
        float _388 = mix(_373, spvNMax(_373, abs(_385)), _341);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_370, 0.0)) | (packHalf2x16(vec2(_385, 0.0)) << 16u);
        float _401 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-4.0) + _237) * _238)), 0.0).x, _336, _206);
        float _402 = abs(_401);
        float _404 = mix(_388, spvNMax(_388, _402), _341);
        float _416 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-3.0) + _237) * _238)), 0.0).x, _336, _206);
        float _419 = mix(_404, spvNMax(_404, abs(_416)), _341);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_401, 0.0)) | (packHalf2x16(vec2(_416, 0.0)) << 16u);
        float _432 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-2.0) + _237) * _238)), 0.0).x, _336, _206);
        float _433 = abs(_432);
        float _435 = mix(_419, spvNMax(_419, _433), _341);
        float _447 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (((-1.0) + _237) * _238)), 0.0).x, _336, _206);
        float _450 = mix(_435, spvNMax(_435, abs(_447)), _341);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_432, 0.0)) | (packHalf2x16(vec2(_447, 0.0)) << 16u);
        float _462 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * (_237 * _238)), 0.0).x, _336, _206);
        float _463 = abs(_462);
        float _465 = mix(_450, spvNMax(_450, _463), _341);
        float _477 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((1.0 + _237) * _238)), 0.0).x, _336, _206);
        float _480 = mix(_465, spvNMax(_465, abs(_477)), _341);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_462, 0.0)) | (packHalf2x16(vec2(_477, 0.0)) << 16u);
        float _493 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((2.0 + _237) * _238)), 0.0).x, _336, _206);
        float _494 = abs(_493);
        float _496 = mix(_480, spvNMax(_480, _494), _341);
        float _508 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((3.0 + _237) * _238)), 0.0).x, _336, _206);
        float _511 = mix(_496, spvNMax(_496, abs(_508)), _341);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_493, 0.0)) | (packHalf2x16(vec2(_508, 0.0)) << 16u);
        float _524 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((4.0 + _237) * _238)), 0.0).x, _336, _206);
        float _525 = abs(_524);
        float _527 = mix(_511, spvNMax(_511, _525), _341);
        float _539 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((5.0 + _237) * _238)), 0.0).x, _336, _206);
        float _542 = mix(_527, spvNMax(_527, abs(_539)), _341);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_524, 0.0)) | (packHalf2x16(vec2(_539, 0.0)) << 16u);
        float _555 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((6.0 + _237) * _238)), 0.0).x, _336, _206);
        float _556 = abs(_555);
        float _558 = mix(_542, spvNMax(_542, _556), _341);
        float _570 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((7.0 + _237) * _238)), 0.0).x, _336, _206);
        float _573 = mix(_558, spvNMax(_558, abs(_570)), _341);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_555, 0.0)) | (packHalf2x16(vec2(_570, 0.0)) << 16u);
        float _586 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * ((8.0 + _237) * _238)), 0.0).x, _336, _206);
        float _587 = abs(_586);
        _265 = mix(_573, spvNMax(_573, _587), _341);
        _273 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_272, _338), _371), _402), _433), _463), _494), _525), _556), _587);
        _275 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_274, _338), _371), _402), _433), _463), _494), _525), _556), _587);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_586, 0.0));
        for (uint _593 = 0u; _593 < 8u; )
        {
            _170[_593] = 0u;
            _593++;
            continue;
        }
        for (uint _600 = 0u; _600 < 17u; _600++)
        {
            uint _608 = _600 & 1u;
            uint _609 = _600 >> 1u;
            vec2 _171 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_609] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_609] >> 16u) & 65535u).x);
            float _624 = (float(int(_600 - 8u)) + _237) * _238;
            for (int _626 = 0; _626 < 8; _626++)
            {
                if ((_624 / (((float(_626) + _258) * 0.25) - 1.0)) >= _171[_608])
                {
                    _170[_626] |= (1u << (_600 & 31u));
                }
            }
        }
        vec4 _649;
        _269 = _268;
        _267 = _266;
        _649 = vec4(0.0);
        float _648;
        float _646;
        vec4 _650;
        for (int _651 = 0; _651 < 8; _269 = _646, _267 = _648, _649 = _650, _651++)
        {
            float _659 = ((float(_651) + _258) * 0.25) - 1.0;
            float _660 = abs(_659);
            _648 = _267 + _660;
            if (_170[_651] == 0u)
            {
                _646 = _269;
                _650 = _649;
                continue;
            }
            bool _666 = _659 >= 0.0;
            float _734;
            float _735;
            uint _736;
            if (_666)
            {
                uint _701 = uint(findLSB(_170[_651]));
                uint _705 = uint(max(0, (int(_701) - 1)));
                uint _707 = _701 >> 1u;
                vec2 _172 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_707] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_707] >> 16u) & 65535u).x);
                uint _721 = _705 >> 1u;
                vec2 _173 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_721] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_721] >> 16u) & 65535u).x);
                _734 = _172[_701 & 1u];
                _735 = _173[_705 & 1u];
                _736 = _701;
            }
            else
            {
                uint _670 = uint(findMSB(_170[_651]));
                uint _672 = min(16u, (_670 + 1u));
                uint _674 = _670 >> 1u;
                vec2 _174 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_674] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_674] >> 16u) & 65535u).x);
                uint _688 = _672 >> 1u;
                vec2 _175 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_688] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_688] >> 16u) & 65535u).x);
                _734 = _174[_670 & 1u];
                _735 = _175[_672 & 1u];
                _736 = _670;
            }
            float _742 = (float(int(_736 - 8u)) + _237) * _238;
            float _743 = _742 / _659;
            float _744 = _238 * (_666 ? 1.0 : (-1.0));
            float _747 = _742 - _744;
            float _748 = _735 - (_743 - (_744 / _659));
            float _752 = _660 * float(_170[_651] != 0u);
            float _757 = mix(_747, _742, clamp(_748 / (_748 + (-(_734 - _743))), 0.0, 1.0));
            float _763 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _195 + (_327 * _757), 0.0).x, _336, _206);
            float _771 = log2(spvNMax(0.0, abs(_734) * 0.125)) * smoothstep(4.0, 0.0, _275 - _273);
            float _774 = subgroupShuffle(_771, gl_SubgroupInvocationID ^ 1u);
            float _775 = spvNMin(_771, _774);
            float _777 = subgroupShuffle(_775, gl_SubgroupInvocationID ^ 8u);
            if (_734 > _206)
            {
                _646 = _269;
                _650 = _649;
                continue;
            }
            float _783 = _748 * 0.100000001490116119384765625;
            bool _789;
            if (((_757 / _659) - _763) < _783)
            {
                _789 = (_763 - _734) < _783;
            }
            else
            {
                _789 = false;
            }
            float _801;
            float _802;
            if (_789)
            {
                _801 = _269;
                _802 = _752;
            }
            else
            {
                float _796 = smoothstep(0.0, 30.0, _735) * smoothstep(0.20000000298023223876953125 * _206, 0.0, _734);
                _801 = _269 + (_752 * _796);
                _802 = _752 * (1.0 - _796);
            }
            _646 = _801;
            _650 = _649 + (textureLod(sampler2D(color_tex, sampler_nnc), _195 + (_327 * (_789 ? _757 : _747)), spvNMin(_775, _777)) * _802);
        }
        _262 = _261 + _649;
    }
    float _816 = _268 * smoothstep(0.00999999977648258209228515625 * _261.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _261.w), _268);
    vec4 _877;
    SPIRV_CROSS_BRANCH
    if (int(_816 > 0.0) != 0)
    {
        float _823 = -_206;
        vec4 _825;
        _825 = vec4(0.0);
        vec4 _826;
        float _829;
        float _828 = _823;
        uint _830 = 0u;
        for (; _830 < 16u; _825 = _826, _828 = _829, _830++)
        {
            vec2 _841 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_830)) + vec2(0.5)) + _222.xy);
            float _843 = sqrt(_841.x);
            float _845 = _841.y * 6.283185482025146484375;
            vec2 _856 = _195 + (((vec2(cos(_845) * _843, sin(_845) * _843) * 1.0) / vec2(_Globals.color_tex_dims)) * _264);
            vec4 _859 = textureLod(sampler2D(coc_tex, sampler_nnc), _856, 0.0);
            float _860 = _859.x;
            if (_860 > (_828 - 10.0))
            {
                _826 = _825 + textureLod(sampler2D(color_tex, sampler_llc), _856, 0.0);
                _829 = spvNMax(_828, _860);
            }
            else
            {
                _826 = _825;
                _829 = _828;
            }
        }
        _877 = _261 + ((_825 / vec4(spvNMax(1.0, _825.w))) * _816);
    }
    else
    {
        _877 = _261;
    }
    imageStore(img_output, ivec2(_188), spvNMax(vec4(0.0), _877 / vec4(_266)));
}

