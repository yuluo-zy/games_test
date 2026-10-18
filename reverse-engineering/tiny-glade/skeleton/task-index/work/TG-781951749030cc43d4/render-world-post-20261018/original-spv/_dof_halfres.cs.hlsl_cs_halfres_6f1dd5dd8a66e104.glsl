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
    uvec2 _189 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[4] + gl_WorkGroupID.x]);
    vec2 _196 = (vec2(ivec2(_189)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _207 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _196, 0.0).x);
    vec4 _223 = ((texelFetch(blue_noise_texture, ivec2(((_189 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    uvec2 _224 = uvec2(7u) + _189;
    float _236 = (fract(52.98291778564453125 * fract((0.067110560834407806396484375 * float(_224.x)) + (0.005837149918079376220703125 * float(_224.y)))) - 0.5) * 0.5;
    float _237 = 0.5 + _236;
    float _238 = _223.y;
    float _239 = _207 * 0.125;
    float _244 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _254 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_189.x) + _244)) + (0.005837149918079376220703125 * (float(_189.y) + _244))));
    bool _255 = _237 < 0.5;
    float _259;
    if (_255)
    {
        _259 = 0.5 - _236;
    }
    else
    {
        _259 = _237;
    }
    float _260 = _255 ? (-1.0) : 1.0;
    vec4 _262;
    float _265;
    float _267;
    float _269;
    _262 = vec4(0.0);
    _265 = 0.0;
    _267 = 0.0;
    _269 = 0.0;
    vec4 _263;
    float _266;
    float _274;
    float _276;
    uint _171[8];
    float _268;
    float _270;
    uint _271 = 0u;
    float _273 = 10000000000.0;
    float _275 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _271 < 2u; _262 = _263, _265 = _266, _267 = _268, _269 = _270, _271++, _273 = _274, _275 = _276)
    {
        float _282 = (float(_271) + _254) * 1.57079637050628662109375;
        vec2 _285 = vec2(cos(_282), sin(_282));
        vec2 _323;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _298 = vec4((_196 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _302 = _298.xy / vec2(_298.z);
            _302.y = _302.y * (-1.0);
            vec2 _306 = normalize(_302);
            mat2 _311 = mat2(_306, vec2(-_306.y, _306.x));
            float _316 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_302, _302)));
            _323 = ((_285 * _311) * vec2(_316, 1.0 / _316)) * transpose(_311);
        }
        else
        {
            _323 = _285;
        }
        vec2 _328 = _323 * (vec2(_260) / vec2(_Globals.color_tex_dims));
        float _337 = -_207;
        float _338 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-8.0) + _238) * _239)), 0.0).x, _337, _207);
        float _339 = abs(_338);
        float _342 = _223.x * 0.20000000298023223876953125;
        float _343 = mix(_265, spvNMax(_265, _339), _342);
        float _355 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-7.0) + _238) * _239)), 0.0).x, _337, _207);
        float _358 = mix(_343, spvNMax(_343, abs(_355)), _342);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_338, 0.0)) | (packHalf2x16(vec2(_355, 0.0)) << 16u);
        float _371 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-6.0) + _238) * _239)), 0.0).x, _337, _207);
        float _372 = abs(_371);
        float _374 = mix(_358, spvNMax(_358, _372), _342);
        float _386 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-5.0) + _238) * _239)), 0.0).x, _337, _207);
        float _389 = mix(_374, spvNMax(_374, abs(_386)), _342);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_371, 0.0)) | (packHalf2x16(vec2(_386, 0.0)) << 16u);
        float _402 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-4.0) + _238) * _239)), 0.0).x, _337, _207);
        float _403 = abs(_402);
        float _405 = mix(_389, spvNMax(_389, _403), _342);
        float _417 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-3.0) + _238) * _239)), 0.0).x, _337, _207);
        float _420 = mix(_405, spvNMax(_405, abs(_417)), _342);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_402, 0.0)) | (packHalf2x16(vec2(_417, 0.0)) << 16u);
        float _433 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-2.0) + _238) * _239)), 0.0).x, _337, _207);
        float _434 = abs(_433);
        float _436 = mix(_420, spvNMax(_420, _434), _342);
        float _448 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (((-1.0) + _238) * _239)), 0.0).x, _337, _207);
        float _451 = mix(_436, spvNMax(_436, abs(_448)), _342);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_433, 0.0)) | (packHalf2x16(vec2(_448, 0.0)) << 16u);
        float _463 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * (_238 * _239)), 0.0).x, _337, _207);
        float _464 = abs(_463);
        float _466 = mix(_451, spvNMax(_451, _464), _342);
        float _478 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((1.0 + _238) * _239)), 0.0).x, _337, _207);
        float _481 = mix(_466, spvNMax(_466, abs(_478)), _342);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_463, 0.0)) | (packHalf2x16(vec2(_478, 0.0)) << 16u);
        float _494 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((2.0 + _238) * _239)), 0.0).x, _337, _207);
        float _495 = abs(_494);
        float _497 = mix(_481, spvNMax(_481, _495), _342);
        float _509 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((3.0 + _238) * _239)), 0.0).x, _337, _207);
        float _512 = mix(_497, spvNMax(_497, abs(_509)), _342);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_494, 0.0)) | (packHalf2x16(vec2(_509, 0.0)) << 16u);
        float _525 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((4.0 + _238) * _239)), 0.0).x, _337, _207);
        float _526 = abs(_525);
        float _528 = mix(_512, spvNMax(_512, _526), _342);
        float _540 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((5.0 + _238) * _239)), 0.0).x, _337, _207);
        float _543 = mix(_528, spvNMax(_528, abs(_540)), _342);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_525, 0.0)) | (packHalf2x16(vec2(_540, 0.0)) << 16u);
        float _556 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((6.0 + _238) * _239)), 0.0).x, _337, _207);
        float _557 = abs(_556);
        float _559 = mix(_543, spvNMax(_543, _557), _342);
        float _571 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((7.0 + _238) * _239)), 0.0).x, _337, _207);
        float _574 = mix(_559, spvNMax(_559, abs(_571)), _342);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_556, 0.0)) | (packHalf2x16(vec2(_571, 0.0)) << 16u);
        float _587 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * ((8.0 + _238) * _239)), 0.0).x, _337, _207);
        float _588 = abs(_587);
        _266 = mix(_574, spvNMax(_574, _588), _342);
        _274 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_273, _339), _372), _403), _434), _464), _495), _526), _557), _588);
        _276 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_275, _339), _372), _403), _434), _464), _495), _526), _557), _588);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_587, 0.0));
        for (uint _594 = 0u; _594 < 8u; )
        {
            _171[_594] = 0u;
            _594++;
            continue;
        }
        for (uint _601 = 0u; _601 < 17u; _601++)
        {
            uint _609 = _601 & 1u;
            uint _610 = _601 >> 1u;
            vec2 _172 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_610] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_610] >> 16u) & 65535u).x);
            float _625 = (float(int(_601 - 8u)) + _238) * _239;
            for (int _627 = 0; _627 < 8; _627++)
            {
                if ((_625 / (((float(_627) + _259) * 0.25) - 1.0)) >= _172[_609])
                {
                    _171[_627] |= (1u << (_601 & 31u));
                }
            }
        }
        vec4 _650;
        _270 = _269;
        _268 = _267;
        _650 = vec4(0.0);
        float _649;
        float _647;
        vec4 _651;
        for (int _652 = 0; _652 < 8; _270 = _647, _268 = _649, _650 = _651, _652++)
        {
            float _660 = ((float(_652) + _259) * 0.25) - 1.0;
            float _661 = abs(_660);
            _649 = _268 + _661;
            if (_171[_652] == 0u)
            {
                _647 = _270;
                _651 = _650;
                continue;
            }
            bool _667 = _660 >= 0.0;
            float _735;
            float _736;
            uint _737;
            if (_667)
            {
                uint _702 = uint(findLSB(_171[_652]));
                uint _706 = uint(max(0, (int(_702) - 1)));
                uint _708 = _702 >> 1u;
                vec2 _173 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_708] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_708] >> 16u) & 65535u).x);
                uint _722 = _706 >> 1u;
                vec2 _174 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_722] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_722] >> 16u) & 65535u).x);
                _735 = _173[_702 & 1u];
                _736 = _174[_706 & 1u];
                _737 = _702;
            }
            else
            {
                uint _671 = uint(findMSB(_171[_652]));
                uint _673 = min(16u, (_671 + 1u));
                uint _675 = _671 >> 1u;
                vec2 _175 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_675] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_675] >> 16u) & 65535u).x);
                uint _689 = _673 >> 1u;
                vec2 _176 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_689] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_689] >> 16u) & 65535u).x);
                _735 = _175[_671 & 1u];
                _736 = _176[_673 & 1u];
                _737 = _671;
            }
            float _743 = (float(int(_737 - 8u)) + _238) * _239;
            float _744 = _743 / _660;
            float _745 = _239 * (_667 ? 1.0 : (-1.0));
            float _748 = _743 - _745;
            float _749 = _736 - (_744 - (_745 / _660));
            float _753 = _661 * float(_171[_652] != 0u);
            float _758 = mix(_748, _743, clamp(_749 / (_749 + (-(_735 - _744))), 0.0, 1.0));
            float _764 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _196 + (_328 * _758), 0.0).x, _337, _207);
            float _772 = log2(spvNMax(0.0, abs(_735) * 0.125)) * smoothstep(4.0, 0.0, _276 - _274);
            float _775 = subgroupShuffle(_772, gl_SubgroupInvocationID ^ 1u);
            float _776 = spvNMin(_772, _775);
            float _778 = subgroupShuffle(_776, gl_SubgroupInvocationID ^ 8u);
            if (_735 > _207)
            {
                _647 = _270;
                _651 = _650;
                continue;
            }
            float _784 = _749 * 0.100000001490116119384765625;
            bool _790;
            if (((_758 / _660) - _764) < _784)
            {
                _790 = (_764 - _735) < _784;
            }
            else
            {
                _790 = false;
            }
            float _814;
            float _815;
            if (_790)
            {
                float _808 = abs(_764);
                if (_808 < 8.0)
                {
                    _647 = _270;
                    _651 = _650;
                    continue;
                }
                _814 = _270;
                _815 = _753 * smoothstep(8.0, 11.0, _808);
            }
            else
            {
                float _794 = abs(_736);
                if (_794 < 8.0)
                {
                    _647 = _270;
                    _651 = _650;
                    continue;
                }
                float _799 = _753 * smoothstep(8.0, 11.0, _794);
                float _803 = smoothstep(0.0, 30.0, _736) * smoothstep(0.20000000298023223876953125 * _207, 0.0, _735);
                _814 = _270 + (_799 * _803);
                _815 = _799 * (1.0 - _803);
            }
            _647 = _814;
            _651 = _650 + (textureLod(sampler2D(color_tex, sampler_nnc), _196 + (_328 * (_790 ? _758 : _748)), spvNMin(_776, _778)) * _815);
        }
        _263 = _262 + _650;
    }
    float _829 = _269 * smoothstep(0.00999999977648258209228515625 * _262.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _262.w), _269);
    vec4 _890;
    SPIRV_CROSS_BRANCH
    if (int(_829 > 0.0) != 0)
    {
        float _836 = -_207;
        vec4 _838;
        _838 = vec4(0.0);
        vec4 _839;
        float _842;
        float _841 = _836;
        uint _843 = 0u;
        for (; _843 < 16u; _838 = _839, _841 = _842, _843++)
        {
            vec2 _854 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_843)) + vec2(0.5)) + _223.xy);
            float _856 = sqrt(_854.x);
            float _858 = _854.y * 6.283185482025146484375;
            vec2 _869 = _196 + (((vec2(cos(_858) * _856, sin(_858) * _856) * 1.0) / vec2(_Globals.color_tex_dims)) * _265);
            vec4 _872 = textureLod(sampler2D(coc_tex, sampler_nnc), _869, 0.0);
            float _873 = _872.x;
            if (_873 > (_841 - 10.0))
            {
                _839 = _838 + textureLod(sampler2D(color_tex, sampler_llc), _869, 0.0);
                _842 = spvNMax(_841, _873);
            }
            else
            {
                _839 = _838;
                _842 = _841;
            }
        }
        _890 = _262 + ((_838 / vec4(spvNMax(1.0, _838.w))) * _829);
    }
    else
    {
        _890 = _262;
    }
    imageStore(img_output, ivec2(_189), spvNMax(vec4(0.0), _890 / vec4(_267)));
}

