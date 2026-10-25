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

shared uint ring_samples[64][3];

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
    uvec2 _170 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[2] + gl_WorkGroupID.x]);
    vec2 _177 = (vec2(ivec2(_170)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _188 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _177, 0.0).x);
    vec4 _204 = ((texelFetch(blue_noise_texture, ivec2(((_170 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    float _205 = _204.z;
    float _206 = _204.y;
    float _207 = _188 * 0.5;
    float _208 = _204.x;
    bool _209 = _205 < 0.5;
    float _213;
    if (_209)
    {
        _213 = 1.0 - _205;
    }
    else
    {
        _213 = _205;
    }
    float _214 = _209 ? (-1.0) : 1.0;
    vec4 _216;
    float _219;
    float _221;
    float _223;
    _216 = vec4(0.0);
    _219 = 0.0;
    _221 = 0.0;
    _223 = 0.0;
    vec4 _217;
    float _220;
    float _228;
    float _230;
    uint _152[2];
    float _222;
    float _224;
    uint _225 = 0u;
    float _227 = 10000000000.0;
    float _229 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _225 < 4u; _216 = _217, _219 = _220, _221 = _222, _223 = _224, _225++, _227 = _228, _229 = _230)
    {
        float _236 = (float(_225) + _208) * 0.785398185253143310546875;
        vec2 _239 = vec2(cos(_236), sin(_236));
        vec2 _277;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _252 = vec4((_177 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _256 = _252.xy / vec2(_252.z);
            _256.y = _256.y * (-1.0);
            vec2 _260 = normalize(_256);
            mat2 _265 = mat2(_260, vec2(-_260.y, _260.x));
            float _270 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_256, _256)));
            _277 = ((_239 * _265) * vec2(_270, 1.0 / _270)) * transpose(_265);
        }
        else
        {
            _277 = _239;
        }
        vec2 _282 = _277 * (vec2(_214) / vec2(_Globals.color_tex_dims));
        float _291 = -_188;
        float _292 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * (((-2.0) + _206) * _207)), 0.0).x, _291, _188);
        float _293 = abs(_292);
        float _295 = _208 * 0.20000000298023223876953125;
        float _296 = mix(_219, spvNMax(_219, _293), _295);
        float _308 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * (((-1.0) + _206) * _207)), 0.0).x, _291, _188);
        float _311 = mix(_296, spvNMax(_296, abs(_308)), _295);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_292, 0.0)) | (packHalf2x16(vec2(_308, 0.0)) << 16u);
        float _323 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * (_206 * _207)), 0.0).x, _291, _188);
        float _324 = abs(_323);
        float _326 = mix(_311, spvNMax(_311, _324), _295);
        float _338 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * ((1.0 + _206) * _207)), 0.0).x, _291, _188);
        float _341 = mix(_326, spvNMax(_326, abs(_338)), _295);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_323, 0.0)) | (packHalf2x16(vec2(_338, 0.0)) << 16u);
        float _354 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * ((2.0 + _206) * _207)), 0.0).x, _291, _188);
        float _355 = abs(_354);
        _220 = mix(_341, spvNMax(_341, _355), _295);
        _228 = spvNMin(spvNMin(spvNMin(_227, _293), _324), _355);
        _230 = spvNMax(spvNMax(spvNMax(_229, _293), _324), _355);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_354, 0.0));
        for (uint _361 = 0u; _361 < 2u; )
        {
            _152[_361] = 0u;
            _361++;
            continue;
        }
        for (uint _368 = 0u; _368 < 5u; _368++)
        {
            uint _376 = _368 & 1u;
            uint _377 = _368 >> 1u;
            vec2 _153 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_377] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_377] >> 16u) & 65535u).x);
            float _392 = (float(int(_368 - 2u)) + _206) * _207;
            for (int _394 = 0; _394 < 2; _394++)
            {
                if ((_392 / ((float(_394) + _213) - 1.0)) >= _153[_376])
                {
                    _152[_394] |= (1u << (_368 & 31u));
                }
            }
        }
        vec4 _416;
        _224 = _223;
        _222 = _221;
        _416 = vec4(0.0);
        float _415;
        float _413;
        vec4 _417;
        for (int _418 = 0; _418 < 2; _224 = _413, _222 = _415, _416 = _417, _418++)
        {
            float _425 = (float(_418) + _213) - 1.0;
            float _426 = abs(_425);
            _415 = _222 + _426;
            if (_152[_418] == 0u)
            {
                _413 = _224;
                _417 = _416;
                continue;
            }
            bool _432 = _425 >= 0.0;
            float _500;
            float _501;
            uint _502;
            if (_432)
            {
                uint _467 = uint(findLSB(_152[_418]));
                uint _471 = uint(max(0, (int(_467) - 1)));
                uint _473 = _467 >> 1u;
                vec2 _154 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_473] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_473] >> 16u) & 65535u).x);
                uint _487 = _471 >> 1u;
                vec2 _155 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_487] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_487] >> 16u) & 65535u).x);
                _500 = _154[_467 & 1u];
                _501 = _155[_471 & 1u];
                _502 = _467;
            }
            else
            {
                uint _436 = uint(findMSB(_152[_418]));
                uint _438 = min(4u, (_436 + 1u));
                uint _440 = _436 >> 1u;
                vec2 _156 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_440] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_440] >> 16u) & 65535u).x);
                uint _454 = _438 >> 1u;
                vec2 _157 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_454] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_454] >> 16u) & 65535u).x);
                _500 = _156[_436 & 1u];
                _501 = _157[_438 & 1u];
                _502 = _436;
            }
            float _508 = (float(int(_502 - 2u)) + _206) * _207;
            float _509 = _508 / _425;
            float _510 = _207 * (_432 ? 1.0 : (-1.0));
            float _513 = _508 - _510;
            float _514 = _501 - (_509 - (_510 / _425));
            float _518 = _426 * float(_152[_418] != 0u);
            float _523 = mix(_513, _508, clamp(_514 / (_514 + (-(_500 - _509))), 0.0, 1.0));
            float _529 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _177 + (_282 * _523), 0.0).x, _291, _188);
            float _537 = log2(spvNMax(0.0, abs(_500) * 0.125)) * smoothstep(4.0, 0.0, _230 - _228);
            float _540 = subgroupShuffle(_537, gl_SubgroupInvocationID ^ 1u);
            float _541 = spvNMin(_537, _540);
            float _543 = subgroupShuffle(_541, gl_SubgroupInvocationID ^ 8u);
            if (_500 > _188)
            {
                _413 = _224;
                _417 = _416;
                continue;
            }
            float _549 = _514 * 0.100000001490116119384765625;
            bool _555;
            if (((_523 / _425) - _529) < _549)
            {
                _555 = (_529 - _500) < _549;
            }
            else
            {
                _555 = false;
            }
            float _567;
            float _568;
            if (_555)
            {
                _567 = _224;
                _568 = _518;
            }
            else
            {
                float _562 = smoothstep(0.0, 30.0, _501) * smoothstep(0.20000000298023223876953125 * _188, 0.0, _500);
                _567 = _224 + (_518 * _562);
                _568 = _518 * (1.0 - _562);
            }
            _413 = _567;
            _417 = _416 + (textureLod(sampler2D(color_tex, sampler_nnc), _177 + (_282 * (_555 ? _523 : _513)), spvNMin(_541, _543)) * _568);
        }
        _217 = _216 + _416;
    }
    float _582 = _223 * smoothstep(0.00999999977648258209228515625 * _216.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _216.w), _223);
    vec4 _643;
    SPIRV_CROSS_BRANCH
    if (int(_582 > 0.0) != 0)
    {
        float _589 = -_188;
        vec4 _591;
        _591 = vec4(0.0);
        vec4 _592;
        float _595;
        float _594 = _589;
        uint _596 = 0u;
        for (; _596 < 16u; _591 = _592, _594 = _595, _596++)
        {
            vec2 _607 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_596)) + vec2(0.5)) + _204.xy);
            float _609 = sqrt(_607.x);
            float _611 = _607.y * 6.283185482025146484375;
            vec2 _622 = _177 + (((vec2(cos(_611) * _609, sin(_611) * _609) * 1.0) / vec2(_Globals.color_tex_dims)) * _219);
            vec4 _625 = textureLod(sampler2D(coc_tex, sampler_nnc), _622, 0.0);
            float _626 = _625.x;
            if (_626 > (_594 - 10.0))
            {
                _592 = _591 + textureLod(sampler2D(color_tex, sampler_llc), _622, 0.0);
                _595 = spvNMax(_594, _626);
            }
            else
            {
                _592 = _591;
                _595 = _594;
            }
        }
        _643 = _216 + ((_591 / vec4(spvNMax(1.0, _591.w))) * _582);
    }
    else
    {
        _643 = _216;
    }
    vec4 _645 = _643 / vec4(_221);
    imageStore(img_output, ivec2(_170), vec4((_645 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _645.w))).xyz, 1.0));
}

