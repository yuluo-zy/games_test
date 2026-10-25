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
    uvec2 _199 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[0] + gl_WorkGroupID.x]);
    uvec2 _200 = _199 & uvec2(1u);
    uint _204 = _200.x + (_200.y * 2u);
    vec4 _206;
    _206 = vec4(0.0);
    for (uint _209 = 0u; _209 < 8u; )
    {
        _206 += texelFetch(halfres_tex, ivec2((_199 / uvec2(2u)) + uvec2(spatial_resolve_offsets.spatial_resolve_offsets_array[(_204 * 16u) + _209].xy)), int(0u));
        _209++;
        continue;
    }
    vec2 _230 = (vec2(ivec2(_199)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _241 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _230, 0.0).x);
    vec4 _257 = ((texelFetch(blue_noise_texture, ivec2(((_199 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _258 = _257.z;
    float _259 = _257.y;
    float _260 = _241 * 0.125;
    float _261 = _257.x;
    bool _262 = _258 < 0.5;
    float _266;
    if (_262)
    {
        _266 = 1.0 - _258;
    }
    else
    {
        _266 = _258;
    }
    float _267 = _262 ? (-1.0) : 1.0;
    vec4 _269;
    float _272;
    float _274;
    float _276;
    _269 = vec4(0.0);
    _272 = 0.0;
    _274 = 0.0;
    _276 = 0.0;
    vec4 _270;
    float _273;
    float _281;
    float _283;
    uint _181[12];
    float _275;
    float _277;
    uint _278 = 0u;
    float _280 = 10000000000.0;
    float _282 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _278 < 4u; _269 = _270, _272 = _273, _274 = _275, _276 = _277, _278++, _280 = _281, _282 = _283)
    {
        float _289 = (float(_278) + _261) * 0.785398185253143310546875;
        vec2 _292 = vec2(cos(_289), sin(_289));
        vec2 _330;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _305 = vec4((_230 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _309 = _305.xy / vec2(_305.z);
            _309.y = _309.y * (-1.0);
            vec2 _313 = normalize(_309);
            mat2 _318 = mat2(_313, vec2(-_313.y, _313.x));
            float _323 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_309, _309)));
            _330 = ((_292 * _318) * vec2(_323, 1.0 / _323)) * transpose(_318);
        }
        else
        {
            _330 = _292;
        }
        vec2 _335 = _330 * (vec2(_267) / vec2(_Globals.color_tex_dims));
        float _344 = -_241;
        float _345 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-8.0) + _259) * _260)), 0.0).x, _344, _241);
        float _346 = abs(_345);
        float _348 = _261 * 0.20000000298023223876953125;
        float _349 = mix(_272, spvNMax(_272, _346), _348);
        float _361 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-7.0) + _259) * _260)), 0.0).x, _344, _241);
        float _364 = mix(_349, spvNMax(_349, abs(_361)), _348);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_345, 0.0)) | (packHalf2x16(vec2(_361, 0.0)) << 16u);
        float _377 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-6.0) + _259) * _260)), 0.0).x, _344, _241);
        float _378 = abs(_377);
        float _380 = mix(_364, spvNMax(_364, _378), _348);
        float _392 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-5.0) + _259) * _260)), 0.0).x, _344, _241);
        float _395 = mix(_380, spvNMax(_380, abs(_392)), _348);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_377, 0.0)) | (packHalf2x16(vec2(_392, 0.0)) << 16u);
        float _408 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-4.0) + _259) * _260)), 0.0).x, _344, _241);
        float _409 = abs(_408);
        float _411 = mix(_395, spvNMax(_395, _409), _348);
        float _423 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-3.0) + _259) * _260)), 0.0).x, _344, _241);
        float _426 = mix(_411, spvNMax(_411, abs(_423)), _348);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_408, 0.0)) | (packHalf2x16(vec2(_423, 0.0)) << 16u);
        float _439 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-2.0) + _259) * _260)), 0.0).x, _344, _241);
        float _440 = abs(_439);
        float _442 = mix(_426, spvNMax(_426, _440), _348);
        float _454 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (((-1.0) + _259) * _260)), 0.0).x, _344, _241);
        float _457 = mix(_442, spvNMax(_442, abs(_454)), _348);
        ring_samples[gl_LocalInvocationIndex][3u] = packHalf2x16(vec2(_439, 0.0)) | (packHalf2x16(vec2(_454, 0.0)) << 16u);
        float _469 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * (_259 * _260)), 0.0).x, _344, _241);
        float _470 = abs(_469);
        float _472 = mix(_457, spvNMax(_457, _470), _348);
        float _484 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((1.0 + _259) * _260)), 0.0).x, _344, _241);
        float _487 = mix(_472, spvNMax(_472, abs(_484)), _348);
        ring_samples[gl_LocalInvocationIndex][4u] = packHalf2x16(vec2(_469, 0.0)) | (packHalf2x16(vec2(_484, 0.0)) << 16u);
        float _500 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((2.0 + _259) * _260)), 0.0).x, _344, _241);
        float _501 = abs(_500);
        float _503 = mix(_487, spvNMax(_487, _501), _348);
        float _515 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((3.0 + _259) * _260)), 0.0).x, _344, _241);
        float _518 = mix(_503, spvNMax(_503, abs(_515)), _348);
        ring_samples[gl_LocalInvocationIndex][5u] = packHalf2x16(vec2(_500, 0.0)) | (packHalf2x16(vec2(_515, 0.0)) << 16u);
        float _531 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((4.0 + _259) * _260)), 0.0).x, _344, _241);
        float _532 = abs(_531);
        float _534 = mix(_518, spvNMax(_518, _532), _348);
        float _546 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((5.0 + _259) * _260)), 0.0).x, _344, _241);
        float _549 = mix(_534, spvNMax(_534, abs(_546)), _348);
        ring_samples[gl_LocalInvocationIndex][6u] = packHalf2x16(vec2(_531, 0.0)) | (packHalf2x16(vec2(_546, 0.0)) << 16u);
        float _562 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((6.0 + _259) * _260)), 0.0).x, _344, _241);
        float _563 = abs(_562);
        float _565 = mix(_549, spvNMax(_549, _563), _348);
        float _577 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((7.0 + _259) * _260)), 0.0).x, _344, _241);
        float _580 = mix(_565, spvNMax(_565, abs(_577)), _348);
        ring_samples[gl_LocalInvocationIndex][7u] = packHalf2x16(vec2(_562, 0.0)) | (packHalf2x16(vec2(_577, 0.0)) << 16u);
        float _593 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * ((8.0 + _259) * _260)), 0.0).x, _344, _241);
        float _594 = abs(_593);
        _273 = mix(_580, spvNMax(_580, _594), _348);
        _281 = spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(spvNMin(_280, _346), _378), _409), _440), _470), _501), _532), _563), _594);
        _283 = spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(spvNMax(_282, _346), _378), _409), _440), _470), _501), _532), _563), _594);
        ring_samples[gl_LocalInvocationIndex][8u] = packHalf2x16(vec2(_593, 0.0));
        for (uint _600 = 0u; _600 < 12u; )
        {
            _181[_600] = 0u;
            _600++;
            continue;
        }
        for (uint _607 = 0u; _607 < 17u; _607++)
        {
            uint _615 = _607 & 1u;
            uint _616 = _607 >> 1u;
            vec2 _182 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_616] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_616] >> 16u) & 65535u).x);
            float _631 = (float(int(_607 - 8u)) + _259) * _260;
            for (int _633 = 0; _633 < 12; _633++)
            {
                if ((_631 / (((float(_633) + _266) * 0.16666667163372039794921875) - 1.0)) >= _182[_615])
                {
                    _181[_633] |= (1u << (_607 & 31u));
                }
            }
        }
        vec4 _656;
        _277 = _276;
        _275 = _274;
        _656 = vec4(0.0);
        float _655;
        float _653;
        vec4 _657;
        for (int _658 = 0; _658 < 12; _277 = _653, _275 = _655, _656 = _657, _658++)
        {
            float _666 = ((float(_658) + _266) * 0.16666667163372039794921875) - 1.0;
            float _667 = abs(_666);
            _655 = _275 + _667;
            if (_181[_658] == 0u)
            {
                _653 = _277;
                _657 = _656;
                continue;
            }
            bool _673 = _666 >= 0.0;
            float _741;
            float _742;
            uint _743;
            if (_673)
            {
                uint _708 = uint(findLSB(_181[_658]));
                uint _712 = uint(max(0, (int(_708) - 1)));
                uint _714 = _708 >> 1u;
                vec2 _183 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_714] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_714] >> 16u) & 65535u).x);
                uint _728 = _712 >> 1u;
                vec2 _184 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_728] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_728] >> 16u) & 65535u).x);
                _741 = _183[_708 & 1u];
                _742 = _184[_712 & 1u];
                _743 = _708;
            }
            else
            {
                uint _677 = uint(findMSB(_181[_658]));
                uint _679 = min(16u, (_677 + 1u));
                uint _681 = _677 >> 1u;
                vec2 _185 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_681] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_681] >> 16u) & 65535u).x);
                uint _695 = _679 >> 1u;
                vec2 _186 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_695] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_695] >> 16u) & 65535u).x);
                _741 = _185[_677 & 1u];
                _742 = _186[_679 & 1u];
                _743 = _677;
            }
            float _749 = (float(int(_743 - 8u)) + _259) * _260;
            float _750 = _749 / _666;
            float _751 = _260 * (_673 ? 1.0 : (-1.0));
            float _754 = _749 - _751;
            float _755 = _742 - (_750 - (_751 / _666));
            float _759 = _667 * float(_181[_658] != 0u);
            float _764 = mix(_754, _749, clamp(_755 / (_755 + (-(_741 - _750))), 0.0, 1.0));
            float _770 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _230 + (_335 * _764), 0.0).x, _344, _241);
            float _778 = log2(spvNMax(0.0, abs(_741) * 0.125)) * smoothstep(4.0, 0.0, _283 - _281);
            float _781 = subgroupShuffle(_778, gl_SubgroupInvocationID ^ 1u);
            float _782 = spvNMin(_778, _781);
            float _784 = subgroupShuffle(_782, gl_SubgroupInvocationID ^ 8u);
            if (_741 > _241)
            {
                _653 = _277;
                _657 = _656;
                continue;
            }
            float _790 = _755 * 0.100000001490116119384765625;
            bool _796;
            if (((_764 / _666) - _770) < _790)
            {
                _796 = (_770 - _741) < _790;
            }
            else
            {
                _796 = false;
            }
            float _820;
            float _821;
            if (_796)
            {
                float _814 = abs(_770);
                if (_814 >= 11.0)
                {
                    _653 = _277;
                    _657 = _656;
                    continue;
                }
                _820 = _277;
                _821 = _759 * smoothstep(11.0, 8.0, _814);
            }
            else
            {
                float _800 = abs(_742);
                if (_800 >= 11.0)
                {
                    _653 = _277;
                    _657 = _656;
                    continue;
                }
                float _805 = _759 * smoothstep(11.0, 8.0, _800);
                float _809 = smoothstep(0.0, 30.0, _742) * smoothstep(0.20000000298023223876953125 * _241, 0.0, _741);
                _820 = _277 + (_805 * _809);
                _821 = _805 * (1.0 - _809);
            }
            _653 = _820;
            _657 = _656 + (textureLod(sampler2D(color_tex, sampler_nnc), _230 + (_335 * (_796 ? _764 : _754)), spvNMin(_782, _784)) * _821);
        }
        _270 = _269 + _656;
    }
    float _835 = _276 * smoothstep(0.00999999977648258209228515625 * _269.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _269.w), _276);
    vec4 _896;
    SPIRV_CROSS_BRANCH
    if (int(_835 > 0.0) != 0)
    {
        float _842 = -_241;
        vec4 _844;
        _844 = vec4(0.0);
        vec4 _845;
        float _848;
        float _847 = _842;
        uint _849 = 0u;
        for (; _849 < 16u; _844 = _845, _847 = _848, _849++)
        {
            vec2 _860 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_849)) + vec2(0.5)) + _257.xy);
            float _862 = sqrt(_860.x);
            float _864 = _860.y * 6.283185482025146484375;
            vec2 _875 = _230 + (((vec2(cos(_864) * _862, sin(_864) * _862) * 1.0) / vec2(_Globals.color_tex_dims)) * _272);
            vec4 _878 = textureLod(sampler2D(coc_tex, sampler_nnc), _875, 0.0);
            float _879 = _878.x;
            if (_879 > (_847 - 10.0))
            {
                _845 = _844 + textureLod(sampler2D(color_tex, sampler_llc), _875, 0.0);
                _848 = spvNMax(_847, _879);
            }
            else
            {
                _845 = _844;
                _848 = _847;
            }
        }
        _896 = _269 + ((_844 / vec4(spvNMax(1.0, _844.w))) * _835);
    }
    else
    {
        _896 = _269;
    }
    vec4 _899 = (_206 * vec4(0.125)) + (_896 / vec4(_274));
    imageStore(img_output, ivec2(_199), vec4((_899 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _899.w))).xyz, 1.0));
}

