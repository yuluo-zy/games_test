#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
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
    layout(offset = 24) uvec2 depth_tex_dims;
    layout(offset = 32) uvec2 coc_tex_dims;
    layout(offset = 40) uvec2 img_output_dims;
} _Globals;

layout(set = 0, binding = 2) uniform sampler sampler_nnc;
layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 14) uniform texture2D blue_noise_texture;
layout(set = 0, binding = 15) uniform texture2D color_tex;
layout(set = 0, binding = 17) uniform texture2D coc_tex;
layout(set = 0, binding = 18, r11f_g11f_b10f) uniform writeonly image2D img_output;

shared uint ring_samples[64][11];

spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    vec2 _168 = vec2(gl_GlobalInvocationID.xy) + vec2(0.5);
    vec2 _171 = vec2(_Globals.color_tex_dims);
    vec2 _172 = _168 / _171;
    float _175 = float(_Globals.color_tex_dims.y);
    float _176 = _175 * 0.0416666679084300994873046875;
    uvec2 _177 = uvec2(_168);
    vec4 _193 = ((texelFetch(blue_noise_texture, ivec2(((_177 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _194 = _193.x;
    float _195 = _193.y;
    float _196 = _175 * 0.0041666668839752674102783203125;
    float _197 = _193.z;
    bool _198 = _197 < 0.5;
    float _202;
    if (_198)
    {
        _202 = 1.0 - _197;
    }
    else
    {
        _202 = _197;
    }
    float _203 = _198 ? (-1.0) : 1.0;
    vec4 _205;
    float _208;
    float _210;
    _205 = vec4(0.0);
    _208 = 0.0;
    _210 = 0.0;
    vec4 _206;
    float _209;
    uint _158[6];
    float _211;
    SPIRV_CROSS_LOOP
    for (uint _212 = 0u; _212 < 2u; _205 = _206, _208 = _209, _210 = _211, _212++)
    {
        float _219 = (float(_212) + _194) * 1.57079637050628662109375;
        vec4 _230 = vec4((_172 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
        vec2 _234 = _230.xy / vec2(_230.z);
        _234.y = _234.y * (-1.0);
        vec2 _238 = normalize(_234);
        mat2 _243 = mat2(_238, vec2(-_238.y, _238.x));
        float _253 = mix(0.699999988079071044921875, 1.0, exp(((-frame_constants.debug_tweakers.z) * _Globals.bokeh_anisotropy) * dot(_234, _234)));
        vec2 _260 = (((vec2(cos(_219), sin(_219)) * _243) * vec2(_253, 1.0 / _253)) * transpose(_243)) * _203;
        vec2 _263 = vec2(_177);
        float _272 = _175 * (-0.0416666679084300994873046875);
        float _273 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-10.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _276 = _194 * 0.20000000298023223876953125;
        float _277 = mix(_208, spvNMax(_208, abs(_273)), _276);
        float _288 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-9.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _291 = mix(_277, spvNMax(_277, abs(_288)), _276);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_273, 0.0)) | (packHalf2x16(vec2(_288, 0.0)) << 16u);
        float _305 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-8.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _308 = mix(_291, spvNMax(_291, abs(_305)), _276);
        float _319 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-7.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _322 = mix(_308, spvNMax(_308, abs(_319)), _276);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_305, 0.0)) | (packHalf2x16(vec2(_319, 0.0)) << 16u);
        float _336 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-6.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _339 = mix(_322, spvNMax(_322, abs(_336)), _276);
        float _350 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-5.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _353 = mix(_339, spvNMax(_339, abs(_350)), _276);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_336, 0.0)) | (packHalf2x16(vec2(_350, 0.0)) << 16u);
        float _367 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-4.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _370 = mix(_353, spvNMax(_353, abs(_367)), _276);
        float _381 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-3.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _384 = mix(_370, spvNMax(_370, abs(_381)), _276);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_367, 0.0)) | (packHalf2x16(vec2(_381, 0.0)) << 16u);
        float _398 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-2.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _401 = mix(_384, spvNMax(_384, abs(_398)), _276);
        float _412 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (((-1.0) + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _415 = mix(_401, spvNMax(_401, abs(_412)), _276);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_398, 0.0)) | (packHalf2x16(vec2(_412, 0.0)) << 16u);
        float _428 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * (_195 * _196))) / _171, 0.0).x, _272, _176);
        float _431 = mix(_415, spvNMax(_415, abs(_428)), _276);
        float _442 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((1.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _445 = mix(_431, spvNMax(_431, abs(_442)), _276);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_428, 0.0)) | (packHalf2x16(vec2(_442, 0.0)) << 16u);
        float _459 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((2.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _462 = mix(_445, spvNMax(_445, abs(_459)), _276);
        float _473 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((3.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _476 = mix(_462, spvNMax(_462, abs(_473)), _276);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_459, 0.0)) | (packHalf2x16(vec2(_473, 0.0)) << 16u);
        float _490 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((4.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _493 = mix(_476, spvNMax(_476, abs(_490)), _276);
        float _504 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((5.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _507 = mix(_493, spvNMax(_493, abs(_504)), _276);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_490, 0.0)) | (packHalf2x16(vec2(_504, 0.0)) << 16u);
        float _521 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((6.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _524 = mix(_507, spvNMax(_507, abs(_521)), _276);
        float _535 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((7.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _538 = mix(_524, spvNMax(_524, abs(_535)), _276);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_521, 0.0)) | (packHalf2x16(vec2(_535, 0.0)) << 16u);
        float _552 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((8.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _555 = mix(_538, spvNMax(_538, abs(_552)), _276);
        float _566 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((9.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        float _569 = mix(_555, spvNMax(_555, abs(_566)), _276);
        ring_samples[gl_LocalInvocationIndex][9u] = packHalf2x16(vec2(_552, 0.0)) | (packHalf2x16(vec2(_566, 0.0)) << 16u);
        float _583 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), (_263 + (_260 * ((10.0 + _195) * _196))) / _171, 0.0).x, _272, _176);
        _209 = mix(_569, spvNMax(_569, abs(_583)), _276);
        ring_samples[gl_LocalInvocationIndex][10u] = packHalf2x16(vec2(_583, 0.0));
        for (uint _590 = 0u; _590 < 6u; )
        {
            _158[_590] = 0u;
            _590++;
            continue;
        }
        for (uint _597 = 0u; _597 < 21u; _597++)
        {
            uint _605 = _597 & 1u;
            uint _606 = _597 >> 1u;
            vec2 _159 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_606] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_606] >> 16u) & 65535u).x);
            float _621 = (float(int(_597 - 10u)) + _195) * _196;
            for (int _623 = 0; _623 < 6; _623++)
            {
                if ((_621 / (((float(_623) + _202) * 0.3333333432674407958984375) - 1.0)) >= _159[_605])
                {
                    _158[_623] |= (1u << (_597 & 31u));
                }
            }
        }
        vec4 _645;
        _211 = _210;
        _645 = vec4(0.0);
        vec4 _646;
        float _643;
        for (int _647 = 0; _647 < 6; _211 = _643, _645 = _646, _647++)
        {
            float _655 = ((float(_647) + _202) * 0.3333333432674407958984375) - 1.0;
            bool _658 = _655 >= 0.0;
            float _726;
            float _727;
            uint _728;
            if (_658)
            {
                uint _693 = uint(findLSB(_158[_647]));
                uint _697 = uint(max(0, (int(_693) - 1)));
                uint _699 = _693 >> 1u;
                vec2 _160 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_699] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_699] >> 16u) & 65535u).x);
                uint _713 = _697 >> 1u;
                vec2 _161 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_713] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_713] >> 16u) & 65535u).x);
                _726 = _160[_693 & 1u];
                _727 = _161[_697 & 1u];
                _728 = _693;
            }
            else
            {
                uint _662 = uint(findMSB(_158[_647]));
                uint _664 = min(20u, (_662 + 1u));
                uint _666 = _662 >> 1u;
                vec2 _162 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_666] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_666] >> 16u) & 65535u).x);
                uint _680 = _664 >> 1u;
                vec2 _163 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_680] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_680] >> 16u) & 65535u).x);
                _726 = _162[_662 & 1u];
                _727 = _163[_664 & 1u];
                _728 = _662;
            }
            float _734 = (float(int(_728 - 10u)) + _195) * _196;
            float _735 = _734 / _655;
            float _736 = _196 * (_658 ? 1.0 : (-1.0));
            float _739 = _734 - _736;
            float _740 = _727 - (_735 - (_736 / _655));
            float _745 = abs(_655) * float(_158[_647] != 0u);
            float _747 = _740 + (-(_726 - _735));
            float _750 = mix(_739, _734, clamp(_740 / _747, 0.0, 1.0));
            float _757 = clamp(texelFetch(coc_tex, ivec2(uvec2(ivec2(_263 + (_260 * _750)))), int(0u)).x, _272, _176);
            float _760 = _740 * 0.100000001490116119384765625;
            bool _766;
            if (((_750 / _655) - _757) < _760)
            {
                _766 = (_757 - _726) < _760;
            }
            else
            {
                _766 = false;
            }
            float _778;
            if (_766)
            {
                _643 = _211;
                _778 = _745;
            }
            else
            {
                float _773 = smoothstep(0.0, 30.0, _727) * smoothstep(_175 * 0.008333333767950534820556640625, 0.0, _726);
                _643 = _211 + (_745 * _773);
                _778 = _745 * (1.0 - _773);
            }
            _646 = _645 + (textureLod(sampler2D(color_tex, sampler_llc), (floor(_263 + (_260 * (_766 ? _750 : _739))) + vec2(0.5)) / _171, log2(spvNMax(0.0, (abs(_726) - _747) * 0.25))) * _778);
        }
        _206 = _205 + _645;
    }
    float _800 = _210 * smoothstep(0.00999999977648258209228515625 * _205.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _205.w), _210);
    vec4 _858;
    SPIRV_CROSS_BRANCH
    if (int(_800 > 0.0) != 0)
    {
        float _807 = _175 * (-0.0416666679084300994873046875);
        vec4 _809;
        _809 = vec4(0.0);
        vec4 _810;
        float _813;
        float _812 = _807;
        uint _814 = 0u;
        for (; _814 < 6u; _809 = _810, _812 = _813, _814++)
        {
            vec2 _825 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_814)) + vec2(0.5)) + _193.xy);
            float _827 = sqrt(_825.x);
            float _829 = _825.y * 6.283185482025146484375;
            uvec2 _840 = uvec2(ivec2(vec2(_177) + ((vec2(cos(_829) * _827, sin(_829) * _827) * 1.0) * _208)));
            vec4 _842 = texelFetch(coc_tex, ivec2(_840), int(0u));
            float _843 = _842.x;
            if (_843 > (_812 - 10.0))
            {
                _810 = _809 + texelFetch(color_tex, ivec2(_840), int(0u));
                _813 = spvNMax(_812, _843);
            }
            else
            {
                _810 = _809;
                _813 = _812;
            }
        }
        _858 = _205 + ((_809 / vec4(spvNMax(1.0, _809.w))) * _800);
    }
    else
    {
        _858 = _205;
    }
    imageStore(img_output, ivec2(gl_GlobalInvocationID.xy), spvNMax(vec3(0.0), (_858.xyz / vec3(spvNMax(9.9999997473787516355514526367188e-06, _858.w))).xyz).xyzz);
}

