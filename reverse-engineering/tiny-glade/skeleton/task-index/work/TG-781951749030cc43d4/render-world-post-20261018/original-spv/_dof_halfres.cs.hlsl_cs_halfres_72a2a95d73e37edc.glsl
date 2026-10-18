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
    uvec2 _187 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[4] + gl_WorkGroupID.x]);
    vec2 _194 = (vec2(ivec2(_187)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _205 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _194, 0.0).x);
    vec4 _221 = ((texelFetch(blue_noise_texture, ivec2(((_187 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _222 = _221.z;
    float _223 = _221.y;
    float _224 = _205 * 0.125;
    float _225 = _221.x;
    bool _226 = _222 < 0.5;
    float _230;
    if (_226)
    {
        _230 = 1.0 - _222;
    }
    else
    {
        _230 = _222;
    }
    float _231 = _226 ? (-1.0) : 1.0;
    vec4 _233;
    float _236;
    float _238;
    float _240;
    _233 = vec4(0.0);
    _236 = 0.0;
    _238 = 0.0;
    _240 = 0.0;
    vec4 _234;
    float _237;
    float _245;
    float _247;
    uint _169[12];
    float _239;
    float _241;
    uint _242 = 0u;
    float _244 = 10000000000.0;
    float _246 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _242 < 4u; _233 = _234, _236 = _237, _238 = _239, _240 = _241, _242++, _244 = _245, _246 = _247)
    {
        float _253 = (float(_242) + _225) * 0.785398185253143310546875;
        vec2 _256 = vec2(cos(_253), sin(_253));
        vec2 _294;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _269 = vec4((_194 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _273 = _269.xy / vec2(_269.z);
            _273.y = _273.y * (-1.0);
            vec2 _277 = normalize(_273);
            mat2 _282 = mat2(_277, vec2(-_277.y, _277.x));
            float _287 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_273, _273)));
            _294 = ((_256 * _282) * vec2(_287, 1.0 / _287)) * transpose(_282);
        }
        else
        {
            _294 = _256;
        }
        vec2 _299 = _294 * (vec2(_231) / vec2(_Globals.color_tex_dims));
        float _308 = -_205;
        float _309 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-8.0) + _223) * _224)), 0.0).x, _308, _205);
        float _310 = abs(_309);
        float _312 = _225 * 0.20000000298023223876953125;
        float _313 = mix(_236, spvNMax(_236, _310), _312);
        float _325 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-7.0) + _223) * _224)), 0.0).x, _308, _205);
        float _328 = mix(_313, spvNMax(_313, abs(_325)), _312);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_309, 0.0)) | (packHalf2x16(vec2(_325, 0.0)) << 16u);
        float _341 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-6.0) + _223) * _224)), 0.0).x, _308, _205);
        float _342 = abs(_341);
        float _344 = mix(_328, spvNMax(_328, _342), _312);
        float _356 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-5.0) + _223) * _224)), 0.0).x, _308, _205);
        float _359 = mix(_344, spvNMax(_344, abs(_356)), _312);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_341, 0.0)) | (packHalf2x16(vec2(_356, 0.0)) << 16u);
        float _372 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-4.0) + _223) * _224)), 0.0).x, _308, _205);
        float _373 = abs(_372);
        float _375 = mix(_359, spvNMax(_359, _373), _312);
        float _387 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-3.0) + _223) * _224)), 0.0).x, _308, _205);
        float _390 = mix(_375, spvNMax(_375, abs(_387)), _312);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_372, 0.0)) | (packHalf2x16(vec2(_387, 0.0)) << 16u);
        float _403 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-2.0) + _223) * _224)), 0.0).x, _308, _205);
        float _404 = abs(_403);
        float _406 = mix(_390, spvNMax(_390, _404), _312);
        float _418 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (((-1.0) + _223) * _224)), 0.0).x, _308, _205);
        float _421 = mix(_406, spvNMax(_406, abs(_418)), _312);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_403, 0.0)) | (packHalf2x16(vec2(_418, 0.0)) << 16u);
        float _433 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * (_223 * _224)), 0.0).x, _308, _205);
        float _434 = abs(_433);
        float _436 = mix(_421, spvNMax(_421, _434), _312);
        float _448 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((1.0 + _223) * _224)), 0.0).x, _308, _205);
        float _451 = mix(_436, spvNMax(_436, abs(_448)), _312);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_433, 0.0)) | (packHalf2x16(vec2(_448, 0.0)) << 16u);
        float _464 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((2.0 + _223) * _224)), 0.0).x, _308, _205);
        float _465 = abs(_464);
        float _467 = mix(_451, spvNMax(_451, _465), _312);
        float _479 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((3.0 + _223) * _224)), 0.0).x, _308, _205);
        float _482 = mix(_467, spvNMax(_467, abs(_479)), _312);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_464, 0.0)) | (packHalf2x16(vec2(_479, 0.0)) << 16u);
        float _495 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((4.0 + _223) * _224)), 0.0).x, _308, _205);
        float _496 = abs(_495);
        float _498 = mix(_482, spvNMax(_482, _496), _312);
        float _510 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((5.0 + _223) * _224)), 0.0).x, _308, _205);
        float _513 = mix(_498, spvNMax(_498, abs(_510)), _312);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_495, 0.0)) | (packHalf2x16(vec2(_510, 0.0)) << 16u);
        float _526 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((6.0 + _223) * _224)), 0.0).x, _308, _205);
        float _527 = abs(_526);
        float _529 = mix(_513, spvNMax(_513, _527), _312);
        float _541 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((7.0 + _223) * _224)), 0.0).x, _308, _205);
        float _544 = mix(_529, spvNMax(_529, abs(_541)), _312);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_526, 0.0)) | (packHalf2x16(vec2(_541, 0.0)) << 16u);
        float _557 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * ((8.0 + _223) * _224)), 0.0).x, _308, _205);
        float _558 = abs(_557);
        _237 = mix(_544, spvNMax(_544, _558), _312);
        _245 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_244, _310), _342), _373), _404), _434), _465), _496), _527), _558);
        _247 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_246, _310), _342), _373), _404), _434), _465), _496), _527), _558);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_557, 0.0));
        for (uint _564 = 0u; _564 < 12u; )
        {
            _169[_564] = 0u;
            _564++;
            continue;
        }
        for (uint _571 = 0u; _571 < 17u; _571++)
        {
            uint _579 = _571 & 1u;
            uint _580 = _571 >> 1u;
            vec2 _170 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_580] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_580] >> 16u) & 65535u).x);
            float _595 = (float(int(_571 - 8u)) + _223) * _224;
            for (int _597 = 0; _597 < 12; _597++)
            {
                if ((_595 / (((float(_597) + _230) * 0.16666667163372039794921875) - 1.0)) >= _170[_579])
                {
                    _169[_597] |= (1u << (_571 & 31u));
                }
            }
        }
        vec4 _620;
        _241 = _240;
        _239 = _238;
        _620 = vec4(0.0);
        float _619;
        float _617;
        vec4 _621;
        for (int _622 = 0; _622 < 12; _241 = _617, _239 = _619, _620 = _621, _622++)
        {
            float _630 = ((float(_622) + _230) * 0.16666667163372039794921875) - 1.0;
            float _631 = abs(_630);
            _619 = _239 + _631;
            if (_169[_622] == 0u)
            {
                _617 = _241;
                _621 = _620;
                continue;
            }
            bool _637 = _630 >= 0.0;
            float _705;
            float _706;
            uint _707;
            if (_637)
            {
                uint _672 = uint(findLSB(_169[_622]));
                uint _676 = uint(max(0, (int(_672) - 1)));
                uint _678 = _672 >> 1u;
                vec2 _171 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_678] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_678] >> 16u) & 65535u).x);
                uint _692 = _676 >> 1u;
                vec2 _172 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_692] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_692] >> 16u) & 65535u).x);
                _705 = _171[_672 & 1u];
                _706 = _172[_676 & 1u];
                _707 = _672;
            }
            else
            {
                uint _641 = uint(findMSB(_169[_622]));
                uint _643 = min(16u, (_641 + 1u));
                uint _645 = _641 >> 1u;
                vec2 _173 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_645] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_645] >> 16u) & 65535u).x);
                uint _659 = _643 >> 1u;
                vec2 _174 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_659] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_659] >> 16u) & 65535u).x);
                _705 = _173[_641 & 1u];
                _706 = _174[_643 & 1u];
                _707 = _641;
            }
            float _713 = (float(int(_707 - 8u)) + _223) * _224;
            float _714 = _713 / _630;
            float _715 = _224 * (_637 ? 1.0 : (-1.0));
            float _718 = _713 - _715;
            float _719 = _706 - (_714 - (_715 / _630));
            float _723 = _631 * float(_169[_622] != 0u);
            float _728 = mix(_718, _713, clamp(_719 / (_719 + (-(_705 - _714))), 0.0, 1.0));
            float _734 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _194 + (_299 * _728), 0.0).x, _308, _205);
            float _742 = log2(spvNMax(0.0, abs(_705) * 0.125)) * smoothstep(4.0, 0.0, _247 - _245);
            float _745 = subgroupShuffle(_742, gl_SubgroupInvocationID ^ 1u);
            float _746 = spvNMin(_742, _745);
            float _748 = subgroupShuffle(_746, gl_SubgroupInvocationID ^ 8u);
            if (_705 > _205)
            {
                _617 = _241;
                _621 = _620;
                continue;
            }
            float _754 = _719 * 0.100000001490116119384765625;
            bool _760;
            if (((_728 / _630) - _734) < _754)
            {
                _760 = (_734 - _705) < _754;
            }
            else
            {
                _760 = false;
            }
            float _784;
            float _785;
            if (_760)
            {
                float _778 = abs(_734);
                if (_778 < 8.0)
                {
                    _617 = _241;
                    _621 = _620;
                    continue;
                }
                _784 = _241;
                _785 = _723 * smoothstep(8.0, 11.0, _778);
            }
            else
            {
                float _764 = abs(_706);
                if (_764 < 8.0)
                {
                    _617 = _241;
                    _621 = _620;
                    continue;
                }
                float _769 = _723 * smoothstep(8.0, 11.0, _764);
                float _773 = smoothstep(0.0, 30.0, _706) * smoothstep(0.20000000298023223876953125 * _205, 0.0, _705);
                _784 = _241 + (_769 * _773);
                _785 = _769 * (1.0 - _773);
            }
            _617 = _784;
            _621 = _620 + (textureLod(sampler2D(color_tex, sampler_nnc), _194 + (_299 * (_760 ? _728 : _718)), spvNMin(_746, _748)) * _785);
        }
        _234 = _233 + _620;
    }
    float _799 = _240 * smoothstep(0.00999999977648258209228515625 * _233.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _233.w), _240);
    vec4 _860;
    SPIRV_CROSS_BRANCH
    if (int(_799 > 0.0) != 0)
    {
        float _806 = -_205;
        vec4 _808;
        _808 = vec4(0.0);
        vec4 _809;
        float _812;
        float _811 = _806;
        uint _813 = 0u;
        for (; _813 < 16u; _808 = _809, _811 = _812, _813++)
        {
            vec2 _824 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_813)) + vec2(0.5)) + _221.xy);
            float _826 = sqrt(_824.x);
            float _828 = _824.y * 6.283185482025146484375;
            vec2 _839 = _194 + (((vec2(cos(_828) * _826, sin(_828) * _826) * 1.0) / vec2(_Globals.color_tex_dims)) * _236);
            vec4 _842 = textureLod(sampler2D(coc_tex, sampler_nnc), _839, 0.0);
            float _843 = _842.x;
            if (_843 > (_811 - 10.0))
            {
                _809 = _808 + textureLod(sampler2D(color_tex, sampler_llc), _839, 0.0);
                _812 = spvNMax(_811, _843);
            }
            else
            {
                _809 = _808;
                _812 = _811;
            }
        }
        _860 = _233 + ((_808 / vec4(spvNMax(1.0, _808.w))) * _799);
    }
    else
    {
        _860 = _233;
    }
    imageStore(img_output, ivec2(_187), spvNMax(vec4(0.0), _860 / vec4(_238)));
}

