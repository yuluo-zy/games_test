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
    uvec2 _186 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[5] + gl_WorkGroupID.x]);
    vec2 _193 = (vec2(ivec2(_186)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _204 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _193, 0.0).x);
    vec4 _220 = ((texelFetch(blue_noise_texture, ivec2(((_186 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _221 = _220.z;
    float _222 = _220.y;
    float _223 = _204 * 0.125;
    float _224 = _220.x;
    bool _225 = _221 < 0.5;
    float _229;
    if (_225)
    {
        _229 = 1.0 - _221;
    }
    else
    {
        _229 = _221;
    }
    float _230 = _225 ? (-1.0) : 1.0;
    vec4 _232;
    float _235;
    float _237;
    float _239;
    _232 = vec4(0.0);
    _235 = 0.0;
    _237 = 0.0;
    _239 = 0.0;
    vec4 _233;
    float _236;
    float _244;
    float _246;
    uint _168[12];
    float _238;
    float _240;
    uint _241 = 0u;
    float _243 = 10000000000.0;
    float _245 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _241 < 4u; _232 = _233, _235 = _236, _237 = _238, _239 = _240, _241++, _243 = _244, _245 = _246)
    {
        float _252 = (float(_241) + _224) * 0.785398185253143310546875;
        vec2 _255 = vec2(cos(_252), sin(_252));
        vec2 _293;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _268 = vec4((_193 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _272 = _268.xy / vec2(_268.z);
            _272.y = _272.y * (-1.0);
            vec2 _276 = normalize(_272);
            mat2 _281 = mat2(_276, vec2(-_276.y, _276.x));
            float _286 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_272, _272)));
            _293 = ((_255 * _281) * vec2(_286, 1.0 / _286)) * transpose(_281);
        }
        else
        {
            _293 = _255;
        }
        vec2 _298 = _293 * (vec2(_230) / vec2(_Globals.color_tex_dims));
        float _307 = -_204;
        float _308 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-8.0) + _222) * _223)), 0.0).x, _307, _204);
        float _309 = abs(_308);
        float _311 = _224 * 0.20000000298023223876953125;
        float _312 = mix(_235, spvNMax(_235, _309), _311);
        float _324 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-7.0) + _222) * _223)), 0.0).x, _307, _204);
        float _327 = mix(_312, spvNMax(_312, abs(_324)), _311);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_308, 0.0)) | (packHalf2x16(vec2(_324, 0.0)) << 16u);
        float _340 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-6.0) + _222) * _223)), 0.0).x, _307, _204);
        float _341 = abs(_340);
        float _343 = mix(_327, spvNMax(_327, _341), _311);
        float _355 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-5.0) + _222) * _223)), 0.0).x, _307, _204);
        float _358 = mix(_343, spvNMax(_343, abs(_355)), _311);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_340, 0.0)) | (packHalf2x16(vec2(_355, 0.0)) << 16u);
        float _371 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-4.0) + _222) * _223)), 0.0).x, _307, _204);
        float _372 = abs(_371);
        float _374 = mix(_358, spvNMax(_358, _372), _311);
        float _386 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-3.0) + _222) * _223)), 0.0).x, _307, _204);
        float _389 = mix(_374, spvNMax(_374, abs(_386)), _311);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_371, 0.0)) | (packHalf2x16(vec2(_386, 0.0)) << 16u);
        float _402 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-2.0) + _222) * _223)), 0.0).x, _307, _204);
        float _403 = abs(_402);
        float _405 = mix(_389, spvNMax(_389, _403), _311);
        float _417 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (((-1.0) + _222) * _223)), 0.0).x, _307, _204);
        float _420 = mix(_405, spvNMax(_405, abs(_417)), _311);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_402, 0.0)) | (packHalf2x16(vec2(_417, 0.0)) << 16u);
        float _432 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * (_222 * _223)), 0.0).x, _307, _204);
        float _433 = abs(_432);
        float _435 = mix(_420, spvNMax(_420, _433), _311);
        float _447 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((1.0 + _222) * _223)), 0.0).x, _307, _204);
        float _450 = mix(_435, spvNMax(_435, abs(_447)), _311);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_432, 0.0)) | (packHalf2x16(vec2(_447, 0.0)) << 16u);
        float _463 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((2.0 + _222) * _223)), 0.0).x, _307, _204);
        float _464 = abs(_463);
        float _466 = mix(_450, spvNMax(_450, _464), _311);
        float _478 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((3.0 + _222) * _223)), 0.0).x, _307, _204);
        float _481 = mix(_466, spvNMax(_466, abs(_478)), _311);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_463, 0.0)) | (packHalf2x16(vec2(_478, 0.0)) << 16u);
        float _494 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((4.0 + _222) * _223)), 0.0).x, _307, _204);
        float _495 = abs(_494);
        float _497 = mix(_481, spvNMax(_481, _495), _311);
        float _509 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((5.0 + _222) * _223)), 0.0).x, _307, _204);
        float _512 = mix(_497, spvNMax(_497, abs(_509)), _311);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_494, 0.0)) | (packHalf2x16(vec2(_509, 0.0)) << 16u);
        float _525 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((6.0 + _222) * _223)), 0.0).x, _307, _204);
        float _526 = abs(_525);
        float _528 = mix(_512, spvNMax(_512, _526), _311);
        float _540 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((7.0 + _222) * _223)), 0.0).x, _307, _204);
        float _543 = mix(_528, spvNMax(_528, abs(_540)), _311);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_525, 0.0)) | (packHalf2x16(vec2(_540, 0.0)) << 16u);
        float _556 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * ((8.0 + _222) * _223)), 0.0).x, _307, _204);
        float _557 = abs(_556);
        _236 = mix(_543, spvNMax(_543, _557), _311);
        _244 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_243, _309), _341), _372), _403), _433), _464), _495), _526), _557);
        _246 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_245, _309), _341), _372), _403), _433), _464), _495), _526), _557);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_556, 0.0));
        for (uint _563 = 0u; _563 < 12u; )
        {
            _168[_563] = 0u;
            _563++;
            continue;
        }
        for (uint _570 = 0u; _570 < 17u; _570++)
        {
            uint _578 = _570 & 1u;
            uint _579 = _570 >> 1u;
            vec2 _169 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_579] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_579] >> 16u) & 65535u).x);
            float _594 = (float(int(_570 - 8u)) + _222) * _223;
            for (int _596 = 0; _596 < 12; _596++)
            {
                if ((_594 / (((float(_596) + _229) * 0.16666667163372039794921875) - 1.0)) >= _169[_578])
                {
                    _168[_596] |= (1u << (_570 & 31u));
                }
            }
        }
        vec4 _619;
        _240 = _239;
        _238 = _237;
        _619 = vec4(0.0);
        float _618;
        float _616;
        vec4 _620;
        for (int _621 = 0; _621 < 12; _240 = _616, _238 = _618, _619 = _620, _621++)
        {
            float _629 = ((float(_621) + _229) * 0.16666667163372039794921875) - 1.0;
            float _630 = abs(_629);
            _618 = _238 + _630;
            if (_168[_621] == 0u)
            {
                _616 = _240;
                _620 = _619;
                continue;
            }
            bool _636 = _629 >= 0.0;
            float _704;
            float _705;
            uint _706;
            if (_636)
            {
                uint _671 = uint(findLSB(_168[_621]));
                uint _675 = uint(max(0, (int(_671) - 1)));
                uint _677 = _671 >> 1u;
                vec2 _170 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_677] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_677] >> 16u) & 65535u).x);
                uint _691 = _675 >> 1u;
                vec2 _171 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_691] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_691] >> 16u) & 65535u).x);
                _704 = _170[_671 & 1u];
                _705 = _171[_675 & 1u];
                _706 = _671;
            }
            else
            {
                uint _640 = uint(findMSB(_168[_621]));
                uint _642 = min(16u, (_640 + 1u));
                uint _644 = _640 >> 1u;
                vec2 _172 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_644] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_644] >> 16u) & 65535u).x);
                uint _658 = _642 >> 1u;
                vec2 _173 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_658] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_658] >> 16u) & 65535u).x);
                _704 = _172[_640 & 1u];
                _705 = _173[_642 & 1u];
                _706 = _640;
            }
            float _712 = (float(int(_706 - 8u)) + _222) * _223;
            float _713 = _712 / _629;
            float _714 = _223 * (_636 ? 1.0 : (-1.0));
            float _717 = _712 - _714;
            float _718 = _705 - (_713 - (_714 / _629));
            float _722 = _630 * float(_168[_621] != 0u);
            float _727 = mix(_717, _712, clamp(_718 / (_718 + (-(_704 - _713))), 0.0, 1.0));
            float _733 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _193 + (_298 * _727), 0.0).x, _307, _204);
            float _741 = log2(spvNMax(0.0, abs(_704) * 0.125)) * smoothstep(4.0, 0.0, _246 - _244);
            float _744 = subgroupShuffle(_741, gl_SubgroupInvocationID ^ 1u);
            float _745 = spvNMin(_741, _744);
            float _747 = subgroupShuffle(_745, gl_SubgroupInvocationID ^ 8u);
            if (_704 > _204)
            {
                _616 = _240;
                _620 = _619;
                continue;
            }
            float _753 = _718 * 0.100000001490116119384765625;
            bool _759;
            if (((_727 / _629) - _733) < _753)
            {
                _759 = (_733 - _704) < _753;
            }
            else
            {
                _759 = false;
            }
            float _771;
            float _772;
            if (_759)
            {
                _771 = _240;
                _772 = _722;
            }
            else
            {
                float _766 = smoothstep(0.0, 30.0, _705) * smoothstep(0.20000000298023223876953125 * _204, 0.0, _704);
                _771 = _240 + (_722 * _766);
                _772 = _722 * (1.0 - _766);
            }
            _616 = _771;
            _620 = _619 + (textureLod(sampler2D(color_tex, sampler_nnc), _193 + (_298 * (_759 ? _727 : _717)), spvNMin(_745, _747)) * _772);
        }
        _233 = _232 + _619;
    }
    float _786 = _239 * smoothstep(0.00999999977648258209228515625 * _232.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _232.w), _239);
    vec4 _847;
    SPIRV_CROSS_BRANCH
    if (int(_786 > 0.0) != 0)
    {
        float _793 = -_204;
        vec4 _795;
        _795 = vec4(0.0);
        vec4 _796;
        float _799;
        float _798 = _793;
        uint _800 = 0u;
        for (; _800 < 16u; _795 = _796, _798 = _799, _800++)
        {
            vec2 _811 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_800)) + vec2(0.5)) + _220.xy);
            float _813 = sqrt(_811.x);
            float _815 = _811.y * 6.283185482025146484375;
            vec2 _826 = _193 + (((vec2(cos(_815) * _813, sin(_815) * _813) * 1.0) / vec2(_Globals.color_tex_dims)) * _235);
            vec4 _829 = textureLod(sampler2D(coc_tex, sampler_nnc), _826, 0.0);
            float _830 = _829.x;
            if (_830 > (_798 - 10.0))
            {
                _796 = _795 + textureLod(sampler2D(color_tex, sampler_llc), _826, 0.0);
                _799 = spvNMax(_798, _830);
            }
            else
            {
                _796 = _795;
                _799 = _798;
            }
        }
        _847 = _232 + ((_795 / vec4(spvNMax(1.0, _795.w))) * _786);
    }
    else
    {
        _847 = _232;
    }
    imageStore(img_output, ivec2(_186), spvNMax(vec4(0.0), _847 / vec4(_237)));
}

