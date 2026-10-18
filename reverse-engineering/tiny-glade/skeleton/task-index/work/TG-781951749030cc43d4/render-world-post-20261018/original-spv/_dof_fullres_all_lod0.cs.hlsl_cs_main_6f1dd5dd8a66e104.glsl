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

shared uint ring_samples[64][5];

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
    uvec2 _179 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[1] + gl_WorkGroupID.x]);
    vec2 _186 = (vec2(ivec2(_179)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _197 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _186, 0.0).x);
    vec4 _213 = ((texelFetch(blue_noise_texture, ivec2(((_179 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    uvec2 _214 = uvec2(7u) + _179;
    float _226 = (fract(52.98291778564453125 * fract((0.067110560834407806396484375 * float(_214.x)) + (0.005837149918079376220703125 * float(_214.y)))) - 0.5) * 0.5;
    float _227 = 0.5 + _226;
    float _228 = _213.y;
    float _229 = _197 * 0.25;
    float _234 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _244 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_179.x) + _234)) + (0.005837149918079376220703125 * (float(_179.y) + _234))));
    bool _245 = _227 < 0.5;
    float _249;
    if (_245)
    {
        _249 = 0.5 - _226;
    }
    else
    {
        _249 = _227;
    }
    float _250 = _245 ? (-1.0) : 1.0;
    vec4 _252;
    float _255;
    float _257;
    float _259;
    _252 = vec4(0.0);
    _255 = 0.0;
    _257 = 0.0;
    _259 = 0.0;
    vec4 _253;
    float _256;
    float _264;
    float _266;
    uint _161[4];
    float _258;
    float _260;
    uint _261 = 0u;
    float _263 = 10000000000.0;
    float _265 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _261 < 2u; _252 = _253, _255 = _256, _257 = _258, _259 = _260, _261++, _263 = _264, _265 = _266)
    {
        float _272 = (float(_261) + _244) * 1.57079637050628662109375;
        vec2 _275 = vec2(cos(_272), sin(_272));
        vec2 _313;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _288 = vec4((_186 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _292 = _288.xy / vec2(_288.z);
            _292.y = _292.y * (-1.0);
            vec2 _296 = normalize(_292);
            mat2 _301 = mat2(_296, vec2(-_296.y, _296.x));
            float _306 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_292, _292)));
            _313 = ((_275 * _301) * vec2(_306, 1.0 / _306)) * transpose(_301);
        }
        else
        {
            _313 = _275;
        }
        vec2 _318 = _313 * (vec2(_250) / vec2(_Globals.color_tex_dims));
        float _327 = -_197;
        float _328 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * (((-4.0) + _228) * _229)), 0.0).x, _327, _197);
        float _329 = abs(_328);
        float _332 = _213.x * 0.20000000298023223876953125;
        float _333 = mix(_255, spvNMax(_255, _329), _332);
        float _345 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * (((-3.0) + _228) * _229)), 0.0).x, _327, _197);
        float _348 = mix(_333, spvNMax(_333, abs(_345)), _332);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_328, 0.0)) | (packHalf2x16(vec2(_345, 0.0)) << 16u);
        float _361 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * (((-2.0) + _228) * _229)), 0.0).x, _327, _197);
        float _362 = abs(_361);
        float _364 = mix(_348, spvNMax(_348, _362), _332);
        float _376 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * (((-1.0) + _228) * _229)), 0.0).x, _327, _197);
        float _379 = mix(_364, spvNMax(_364, abs(_376)), _332);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_361, 0.0)) | (packHalf2x16(vec2(_376, 0.0)) << 16u);
        float _391 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * (_228 * _229)), 0.0).x, _327, _197);
        float _392 = abs(_391);
        float _394 = mix(_379, spvNMax(_379, _392), _332);
        float _406 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * ((1.0 + _228) * _229)), 0.0).x, _327, _197);
        float _409 = mix(_394, spvNMax(_394, abs(_406)), _332);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_391, 0.0)) | (packHalf2x16(vec2(_406, 0.0)) << 16u);
        float _422 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * ((2.0 + _228) * _229)), 0.0).x, _327, _197);
        float _423 = abs(_422);
        float _425 = mix(_409, spvNMax(_409, _423), _332);
        float _437 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * ((3.0 + _228) * _229)), 0.0).x, _327, _197);
        float _440 = mix(_425, spvNMax(_425, abs(_437)), _332);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_422, 0.0)) | (packHalf2x16(vec2(_437, 0.0)) << 16u);
        float _453 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * ((4.0 + _228) * _229)), 0.0).x, _327, _197);
        float _454 = abs(_453);
        _256 = mix(_440, spvNMax(_440, _454), _332);
        _264 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_263, _329), _362), _392), _423), _454);
        _266 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_265, _329), _362), _392), _423), _454);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_453, 0.0));
        for (uint _460 = 0u; _460 < 4u; )
        {
            _161[_460] = 0u;
            _460++;
            continue;
        }
        for (uint _467 = 0u; _467 < 9u; _467++)
        {
            uint _475 = _467 & 1u;
            uint _476 = _467 >> 1u;
            vec2 _162 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_476] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_476] >> 16u) & 65535u).x);
            float _491 = (float(int(_467 - 4u)) + _228) * _229;
            for (int _493 = 0; _493 < 4; _493++)
            {
                if ((_491 / (((float(_493) + _249) * 0.5) - 1.0)) >= _162[_475])
                {
                    _161[_493] |= (1u << (_467 & 31u));
                }
            }
        }
        vec4 _516;
        _260 = _259;
        _258 = _257;
        _516 = vec4(0.0);
        float _515;
        float _513;
        vec4 _517;
        for (int _518 = 0; _518 < 4; _260 = _513, _258 = _515, _516 = _517, _518++)
        {
            float _526 = ((float(_518) + _249) * 0.5) - 1.0;
            float _527 = abs(_526);
            _515 = _258 + _527;
            if (_161[_518] == 0u)
            {
                _513 = _260;
                _517 = _516;
                continue;
            }
            bool _533 = _526 >= 0.0;
            float _601;
            float _602;
            uint _603;
            if (_533)
            {
                uint _568 = uint(findLSB(_161[_518]));
                uint _572 = uint(max(0, (int(_568) - 1)));
                uint _574 = _568 >> 1u;
                vec2 _163 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_574] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_574] >> 16u) & 65535u).x);
                uint _588 = _572 >> 1u;
                vec2 _164 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_588] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_588] >> 16u) & 65535u).x);
                _601 = _163[_568 & 1u];
                _602 = _164[_572 & 1u];
                _603 = _568;
            }
            else
            {
                uint _537 = uint(findMSB(_161[_518]));
                uint _539 = min(8u, (_537 + 1u));
                uint _541 = _537 >> 1u;
                vec2 _165 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_541] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_541] >> 16u) & 65535u).x);
                uint _555 = _539 >> 1u;
                vec2 _166 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_555] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_555] >> 16u) & 65535u).x);
                _601 = _165[_537 & 1u];
                _602 = _166[_539 & 1u];
                _603 = _537;
            }
            float _609 = (float(int(_603 - 4u)) + _228) * _229;
            float _610 = _609 / _526;
            float _611 = _229 * (_533 ? 1.0 : (-1.0));
            float _614 = _609 - _611;
            float _615 = _602 - (_610 - (_611 / _526));
            float _619 = _527 * float(_161[_518] != 0u);
            float _624 = mix(_614, _609, clamp(_615 / (_615 + (-(_601 - _610))), 0.0, 1.0));
            float _630 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _186 + (_318 * _624), 0.0).x, _327, _197);
            float _638 = log2(spvNMax(0.0, abs(_601) * 0.125)) * smoothstep(4.0, 0.0, _266 - _264);
            float _641 = subgroupShuffle(_638, gl_SubgroupInvocationID ^ 1u);
            float _642 = spvNMin(_638, _641);
            float _644 = subgroupShuffle(_642, gl_SubgroupInvocationID ^ 8u);
            if (_601 > _197)
            {
                _513 = _260;
                _517 = _516;
                continue;
            }
            float _650 = _615 * 0.100000001490116119384765625;
            bool _656;
            if (((_624 / _526) - _630) < _650)
            {
                _656 = (_630 - _601) < _650;
            }
            else
            {
                _656 = false;
            }
            float _668;
            float _669;
            if (_656)
            {
                _668 = _260;
                _669 = _619;
            }
            else
            {
                float _663 = smoothstep(0.0, 30.0, _602) * smoothstep(0.20000000298023223876953125 * _197, 0.0, _601);
                _668 = _260 + (_619 * _663);
                _669 = _619 * (1.0 - _663);
            }
            _513 = _668;
            _517 = _516 + (textureLod(sampler2D(color_tex, sampler_nnc), _186 + (_318 * (_656 ? _624 : _614)), spvNMin(_642, _644)) * _669);
        }
        _253 = _252 + _516;
    }
    float _683 = _259 * smoothstep(0.00999999977648258209228515625 * _252.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _252.w), _259);
    vec4 _744;
    SPIRV_CROSS_BRANCH
    if (int(_683 > 0.0) != 0)
    {
        float _690 = -_197;
        vec4 _692;
        _692 = vec4(0.0);
        vec4 _693;
        float _696;
        float _695 = _690;
        uint _697 = 0u;
        for (; _697 < 16u; _692 = _693, _695 = _696, _697++)
        {
            vec2 _708 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_697)) + vec2(0.5)) + _213.xy);
            float _710 = sqrt(_708.x);
            float _712 = _708.y * 6.283185482025146484375;
            vec2 _723 = _186 + (((vec2(cos(_712) * _710, sin(_712) * _710) * 1.0) / vec2(_Globals.color_tex_dims)) * _255);
            vec4 _726 = textureLod(sampler2D(coc_tex, sampler_nnc), _723, 0.0);
            float _727 = _726.x;
            if (_727 > (_695 - 10.0))
            {
                _693 = _692 + textureLod(sampler2D(color_tex, sampler_llc), _723, 0.0);
                _696 = spvNMax(_695, _727);
            }
            else
            {
                _693 = _692;
                _696 = _695;
            }
        }
        _744 = _252 + ((_692 / vec4(spvNMax(1.0, _692.w))) * _683);
    }
    else
    {
        _744 = _252;
    }
    vec4 _746 = _744 / vec4(_257);
    imageStore(img_output, ivec2(_179), vec4((_746 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _746.w))).xyz, 1.0));
}

