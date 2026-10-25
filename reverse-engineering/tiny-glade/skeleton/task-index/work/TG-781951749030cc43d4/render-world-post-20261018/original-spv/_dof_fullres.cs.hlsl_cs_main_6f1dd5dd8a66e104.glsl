#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
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
    layout(offset = 56) uvec2 halfres_tex_dims;
} _Globals;

layout(set = 0, binding = 21, std140) uniform type_spatial_resolve_offsets
{
    ivec4 spatial_resolve_offsets_array[512];
} spatial_resolve_offsets;

layout(set = 0, binding = 22, std430) readonly buffer type_StructuredBuffer_DofTileOffsets
{
    DofTileOffsets _m0[];
} tile_offsets_buffer;

layout(set = 0, binding = 23, std430) readonly buffer type_StructuredBuffer_v2uint
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
layout(set = 0, binding = 20) uniform texture2D halfres_tex;

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
    uvec2 _201 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[0] + gl_WorkGroupID.x]);
    uvec2 _202 = _201 & uvec2(1u);
    uint _206 = _202.x + (_202.y * 2u);
    vec4 _208;
    _208 = vec4(0.0);
    for (uint _211 = 0u; _211 < 8u; )
    {
        _208 += texelFetch(halfres_tex, ivec2((_201 / uvec2(2u)) + uvec2(spatial_resolve_offsets.spatial_resolve_offsets_array[(_206 * 16u) + _211].xy)), int(0u));
        _211++;
        continue;
    }
    vec2 _232 = (vec2(ivec2(_201)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _243 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _232, 0.0).x);
    vec4 _259 = ((texelFetch(blue_noise_texture, ivec2(((_201 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    uvec2 _260 = uvec2(7u) + _201;
    float _272 = (fract(52.98291778564453125 * fract((0.067110560834407806396484375 * float(_260.x)) + (0.005837149918079376220703125 * float(_260.y)))) - 0.5) * 0.5;
    float _273 = 0.5 + _272;
    float _274 = _259.y;
    float _275 = _243 * 0.125;
    float _280 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _290 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_201.x) + _280)) + (0.005837149918079376220703125 * (float(_201.y) + _280))));
    bool _291 = _273 < 0.5;
    float _295;
    if (_291)
    {
        _295 = 0.5 - _272;
    }
    else
    {
        _295 = _273;
    }
    float _296 = _291 ? (-1.0) : 1.0;
    vec4 _298;
    float _301;
    float _303;
    float _305;
    _298 = vec4(0.0);
    _301 = 0.0;
    _303 = 0.0;
    _305 = 0.0;
    vec4 _299;
    float _302;
    float _310;
    float _312;
    uint _183[8];
    float _304;
    float _306;
    uint _307 = 0u;
    float _309 = 10000000000.0;
    float _311 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _307 < 2u; _298 = _299, _301 = _302, _303 = _304, _305 = _306, _307++, _309 = _310, _311 = _312)
    {
        float _318 = (float(_307) + _290) * 1.57079637050628662109375;
        vec2 _321 = vec2(cos(_318), sin(_318));
        vec2 _359;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _334 = vec4((_232 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _338 = _334.xy / vec2(_334.z);
            _338.y = _338.y * (-1.0);
            vec2 _342 = normalize(_338);
            mat2 _347 = mat2(_342, vec2(-_342.y, _342.x));
            float _352 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_338, _338)));
            _359 = ((_321 * _347) * vec2(_352, 1.0 / _352)) * transpose(_347);
        }
        else
        {
            _359 = _321;
        }
        vec2 _364 = _359 * (vec2(_296) / vec2(_Globals.color_tex_dims));
        float _373 = -_243;
        float _374 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-8.0) + _274) * _275)), 0.0).x, _373, _243);
        float _375 = abs(_374);
        float _378 = _259.x * 0.20000000298023223876953125;
        float _379 = mix(_301, spvNMax(_301, _375), _378);
        float _391 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-7.0) + _274) * _275)), 0.0).x, _373, _243);
        float _394 = mix(_379, spvNMax(_379, abs(_391)), _378);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_374, 0.0)) | (packHalf2x16(vec2(_391, 0.0)) << 16u);
        float _407 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-6.0) + _274) * _275)), 0.0).x, _373, _243);
        float _408 = abs(_407);
        float _410 = mix(_394, spvNMax(_394, _408), _378);
        float _422 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-5.0) + _274) * _275)), 0.0).x, _373, _243);
        float _425 = mix(_410, spvNMax(_410, abs(_422)), _378);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_407, 0.0)) | (packHalf2x16(vec2(_422, 0.0)) << 16u);
        float _438 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-4.0) + _274) * _275)), 0.0).x, _373, _243);
        float _439 = abs(_438);
        float _441 = mix(_425, spvNMax(_425, _439), _378);
        float _453 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-3.0) + _274) * _275)), 0.0).x, _373, _243);
        float _456 = mix(_441, spvNMax(_441, abs(_453)), _378);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_438, 0.0)) | (packHalf2x16(vec2(_453, 0.0)) << 16u);
        float _469 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-2.0) + _274) * _275)), 0.0).x, _373, _243);
        float _470 = abs(_469);
        float _472 = mix(_456, spvNMax(_456, _470), _378);
        float _484 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (((-1.0) + _274) * _275)), 0.0).x, _373, _243);
        float _487 = mix(_472, spvNMax(_472, abs(_484)), _378);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_469, 0.0)) | (packHalf2x16(vec2(_484, 0.0)) << 16u);
        float _499 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * (_274 * _275)), 0.0).x, _373, _243);
        float _500 = abs(_499);
        float _502 = mix(_487, spvNMax(_487, _500), _378);
        float _514 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((1.0 + _274) * _275)), 0.0).x, _373, _243);
        float _517 = mix(_502, spvNMax(_502, abs(_514)), _378);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_499, 0.0)) | (packHalf2x16(vec2(_514, 0.0)) << 16u);
        float _530 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((2.0 + _274) * _275)), 0.0).x, _373, _243);
        float _531 = abs(_530);
        float _533 = mix(_517, spvNMax(_517, _531), _378);
        float _545 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((3.0 + _274) * _275)), 0.0).x, _373, _243);
        float _548 = mix(_533, spvNMax(_533, abs(_545)), _378);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_530, 0.0)) | (packHalf2x16(vec2(_545, 0.0)) << 16u);
        float _561 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((4.0 + _274) * _275)), 0.0).x, _373, _243);
        float _562 = abs(_561);
        float _564 = mix(_548, spvNMax(_548, _562), _378);
        float _576 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((5.0 + _274) * _275)), 0.0).x, _373, _243);
        float _579 = mix(_564, spvNMax(_564, abs(_576)), _378);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_561, 0.0)) | (packHalf2x16(vec2(_576, 0.0)) << 16u);
        float _592 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((6.0 + _274) * _275)), 0.0).x, _373, _243);
        float _593 = abs(_592);
        float _595 = mix(_579, spvNMax(_579, _593), _378);
        float _607 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((7.0 + _274) * _275)), 0.0).x, _373, _243);
        float _610 = mix(_595, spvNMax(_595, abs(_607)), _378);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_592, 0.0)) | (packHalf2x16(vec2(_607, 0.0)) << 16u);
        float _623 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * ((8.0 + _274) * _275)), 0.0).x, _373, _243);
        float _624 = abs(_623);
        _302 = mix(_610, spvNMax(_610, _624), _378);
        _310 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_309, _375), _408), _439), _470), _500), _531), _562), _593), _624);
        _312 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_311, _375), _408), _439), _470), _500), _531), _562), _593), _624);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_623, 0.0));
        for (uint _630 = 0u; _630 < 8u; )
        {
            _183[_630] = 0u;
            _630++;
            continue;
        }
        for (uint _637 = 0u; _637 < 17u; _637++)
        {
            uint _645 = _637 & 1u;
            uint _646 = _637 >> 1u;
            vec2 _184 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_646] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_646] >> 16u) & 65535u).x);
            float _661 = (float(int(_637 - 8u)) + _274) * _275;
            for (int _663 = 0; _663 < 8; _663++)
            {
                if ((_661 / (((float(_663) + _295) * 0.25) - 1.0)) >= _184[_645])
                {
                    _183[_663] |= (1u << (_637 & 31u));
                }
            }
        }
        vec4 _686;
        _306 = _305;
        _304 = _303;
        _686 = vec4(0.0);
        float _685;
        float _683;
        vec4 _687;
        for (int _688 = 0; _688 < 8; _306 = _683, _304 = _685, _686 = _687, _688++)
        {
            float _696 = ((float(_688) + _295) * 0.25) - 1.0;
            float _697 = abs(_696);
            _685 = _304 + _697;
            if (_183[_688] == 0u)
            {
                _683 = _306;
                _687 = _686;
                continue;
            }
            bool _703 = _696 >= 0.0;
            float _771;
            float _772;
            uint _773;
            if (_703)
            {
                uint _738 = uint(findLSB(_183[_688]));
                uint _742 = uint(max(0, (int(_738) - 1)));
                uint _744 = _738 >> 1u;
                vec2 _185 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_744] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_744] >> 16u) & 65535u).x);
                uint _758 = _742 >> 1u;
                vec2 _186 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_758] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_758] >> 16u) & 65535u).x);
                _771 = _185[_738 & 1u];
                _772 = _186[_742 & 1u];
                _773 = _738;
            }
            else
            {
                uint _707 = uint(findMSB(_183[_688]));
                uint _709 = min(16u, (_707 + 1u));
                uint _711 = _707 >> 1u;
                vec2 _187 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_711] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_711] >> 16u) & 65535u).x);
                uint _725 = _709 >> 1u;
                vec2 _188 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_725] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_725] >> 16u) & 65535u).x);
                _771 = _187[_707 & 1u];
                _772 = _188[_709 & 1u];
                _773 = _707;
            }
            float _779 = (float(int(_773 - 8u)) + _274) * _275;
            float _780 = _779 / _696;
            float _781 = _275 * (_703 ? 1.0 : (-1.0));
            float _784 = _779 - _781;
            float _785 = _772 - (_780 - (_781 / _696));
            float _789 = _697 * float(_183[_688] != 0u);
            float _794 = mix(_784, _779, clamp(_785 / (_785 + (-(_771 - _780))), 0.0, 1.0));
            float _800 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _232 + (_364 * _794), 0.0).x, _373, _243);
            float _808 = log2(spvNMax(0.0, abs(_771) * 0.125)) * smoothstep(4.0, 0.0, _312 - _310);
            float _811 = subgroupShuffle(_808, gl_SubgroupInvocationID ^ 1u);
            float _812 = spvNMin(_808, _811);
            float _814 = subgroupShuffle(_812, gl_SubgroupInvocationID ^ 8u);
            if (_771 > _243)
            {
                _683 = _306;
                _687 = _686;
                continue;
            }
            float _820 = _785 * 0.100000001490116119384765625;
            bool _826;
            if (((_794 / _696) - _800) < _820)
            {
                _826 = (_800 - _771) < _820;
            }
            else
            {
                _826 = false;
            }
            float _850;
            float _851;
            if (_826)
            {
                float _844 = abs(_800);
                if (_844 >= 11.0)
                {
                    _683 = _306;
                    _687 = _686;
                    continue;
                }
                _850 = _306;
                _851 = _789 * smoothstep(11.0, 8.0, _844);
            }
            else
            {
                float _830 = abs(_772);
                if (_830 >= 11.0)
                {
                    _683 = _306;
                    _687 = _686;
                    continue;
                }
                float _835 = _789 * smoothstep(11.0, 8.0, _830);
                float _839 = smoothstep(0.0, 30.0, _772) * smoothstep(0.20000000298023223876953125 * _243, 0.0, _771);
                _850 = _306 + (_835 * _839);
                _851 = _835 * (1.0 - _839);
            }
            _683 = _850;
            _687 = _686 + (textureLod(sampler2D(color_tex, sampler_nnc), _232 + (_364 * (_826 ? _794 : _784)), spvNMin(_812, _814)) * _851);
        }
        _299 = _298 + _686;
    }
    float _865 = _305 * smoothstep(0.00999999977648258209228515625 * _298.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _298.w), _305);
    vec4 _926;
    SPIRV_CROSS_BRANCH
    if (int(_865 > 0.0) != 0)
    {
        float _872 = -_243;
        vec4 _874;
        _874 = vec4(0.0);
        vec4 _875;
        float _878;
        float _877 = _872;
        uint _879 = 0u;
        for (; _879 < 16u; _874 = _875, _877 = _878, _879++)
        {
            vec2 _890 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_879)) + vec2(0.5)) + _259.xy);
            float _892 = sqrt(_890.x);
            float _894 = _890.y * 6.283185482025146484375;
            vec2 _905 = _232 + (((vec2(cos(_894) * _892, sin(_894) * _892) * 1.0) / vec2(_Globals.color_tex_dims)) * _301);
            vec4 _908 = textureLod(sampler2D(coc_tex, sampler_nnc), _905, 0.0);
            float _909 = _908.x;
            if (_909 > (_877 - 10.0))
            {
                _875 = _874 + textureLod(sampler2D(color_tex, sampler_llc), _905, 0.0);
                _878 = spvNMax(_877, _909);
            }
            else
            {
                _875 = _874;
                _878 = _877;
            }
        }
        _926 = _298 + ((_874 / vec4(spvNMax(1.0, _874.w))) * _865);
    }
    else
    {
        _926 = _298;
    }
    vec4 _929 = (_208 * vec4(0.125)) + (_926 / vec4(_303));
    imageStore(img_output, ivec2(_201), vec4((_929 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _929.w))).xyz, 1.0));
}

