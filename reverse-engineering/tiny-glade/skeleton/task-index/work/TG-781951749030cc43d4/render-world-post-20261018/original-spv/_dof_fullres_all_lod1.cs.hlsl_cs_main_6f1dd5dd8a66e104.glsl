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
    uvec2 _174 = (gl_GlobalInvocationID.xy & uvec2(7u)) + (uvec2(8u) * tile_coords_buffer._m0[tile_offsets_buffer._m0[0u].offsets[2] + gl_WorkGroupID.x]);
    vec2 _181 = (vec2(ivec2(_174)) + vec2(0.5)) / vec2(_Globals.img_output_dims);
    float _192 = spvNMin(float(_Globals.color_tex_dims.y) * 0.0416666679084300994873046875, textureLod(sampler2D(coc_tiles_dilated_tex, sampler_nnc), _181, 0.0).x);
    vec4 _208 = ((texelFetch(blue_noise_texture, ivec2(((_174 + uvec2(13u)) + uvec2(ivec2(int(frame_constants.frame_index * 59u), int(frame_constants.frame_index * 37u)))) & uvec2(255u)), int(0u)) * 255.0) * vec4(0.00390625)) + vec4(0.001953125);
    uvec2 _209 = uvec2(7u) + _174;
    float _221 = (fract(52.98291778564453125 * fract((0.067110560834407806396484375 * float(_209.x)) + (0.005837149918079376220703125 * float(_209.y)))) - 0.5) * 0.5;
    float _222 = 0.5 + _221;
    float _223 = _208.y;
    float _224 = _192 * 0.5;
    float _229 = 5.588237762451171875 * float(frame_constants.frame_index % 64u);
    float _239 = fract(52.98291778564453125 * fract((0.067110560834407806396484375 * (float(_174.x) + _229)) + (0.005837149918079376220703125 * (float(_174.y) + _229))));
    bool _240 = _222 < 0.5;
    float _244;
    if (_240)
    {
        _244 = 0.5 - _221;
    }
    else
    {
        _244 = _222;
    }
    float _245 = _240 ? (-1.0) : 1.0;
    vec4 _247;
    float _250;
    float _252;
    float _254;
    _247 = vec4(0.0);
    _250 = 0.0;
    _252 = 0.0;
    _254 = 0.0;
    vec4 _248;
    float _251;
    float _259;
    float _261;
    uint _156[2];
    float _253;
    float _255;
    uint _256 = 0u;
    float _258 = 10000000000.0;
    float _260 = -10000000000.0;
    SPIRV_CROSS_LOOP
    for (; _256 < 2u; _247 = _248, _250 = _251, _252 = _253, _254 = _255, _256++, _258 = _259, _260 = _261)
    {
        float _267 = (float(_256) + _239) * 1.57079637050628662109375;
        vec2 _270 = vec2(cos(_267), sin(_267));
        vec2 _308;
        if (_Globals.bokeh_anisotropy > 0.0)
        {
            vec4 _283 = vec4((_181 - vec2(0.5)) * vec2(2.0, -2.0), 0.0, 1.0) * frame_constants.view_constants[0u].clip_to_view;
            vec2 _287 = _283.xy / vec2(_283.z);
            _287.y = _287.y * (-1.0);
            vec2 _291 = normalize(_287);
            mat2 _296 = mat2(_291, vec2(-_291.y, _291.x));
            float _301 = mix(0.699999988079071044921875, 1.0, exp(((-4.0) * _Globals.bokeh_anisotropy) * dot(_287, _287)));
            _308 = ((_270 * _296) * vec2(_301, 1.0 / _301)) * transpose(_296);
        }
        else
        {
            _308 = _270;
        }
        vec2 _313 = _308 * (vec2(_245) / vec2(_Globals.color_tex_dims));
        float _322 = -_192;
        float _323 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * (((-2.0) + _223) * _224)), 0.0).x, _322, _192);
        float _324 = abs(_323);
        float _327 = _208.x * 0.20000000298023223876953125;
        float _328 = mix(_250, spvNMax(_250, _324), _327);
        float _340 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * (((-1.0) + _223) * _224)), 0.0).x, _322, _192);
        float _343 = mix(_328, spvNMax(_328, abs(_340)), _327);
        ring_samples[gl_LocalInvocationIndex][0u] = packHalf2x16(vec2(_323, 0.0)) | (packHalf2x16(vec2(_340, 0.0)) << 16u);
        float _355 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * (_223 * _224)), 0.0).x, _322, _192);
        float _356 = abs(_355);
        float _358 = mix(_343, spvNMax(_343, _356), _327);
        float _370 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * ((1.0 + _223) * _224)), 0.0).x, _322, _192);
        float _373 = mix(_358, spvNMax(_358, abs(_370)), _327);
        ring_samples[gl_LocalInvocationIndex][1u] = packHalf2x16(vec2(_355, 0.0)) | (packHalf2x16(vec2(_370, 0.0)) << 16u);
        float _386 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * ((2.0 + _223) * _224)), 0.0).x, _322, _192);
        float _387 = abs(_386);
        _251 = mix(_373, spvNMax(_373, _387), _327);
        _259 = spvNMin(spvNMin(spvNMin(_258, _324), _356), _387);
        _261 = spvNMax(spvNMax(spvNMax(_260, _324), _356), _387);
        ring_samples[gl_LocalInvocationIndex][2u] = packHalf2x16(vec2(_386, 0.0));
        for (uint _393 = 0u; _393 < 2u; )
        {
            _156[_393] = 0u;
            _393++;
            continue;
        }
        for (uint _400 = 0u; _400 < 5u; _400++)
        {
            uint _408 = _400 & 1u;
            uint _409 = _400 >> 1u;
            vec2 _157 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_409] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_409] >> 16u) & 65535u).x);
            float _424 = (float(int(_400 - 2u)) + _223) * _224;
            for (int _426 = 0; _426 < 2; _426++)
            {
                if ((_424 / ((float(_426) + _244) - 1.0)) >= _157[_408])
                {
                    _156[_426] |= (1u << (_400 & 31u));
                }
            }
        }
        vec4 _448;
        _255 = _254;
        _253 = _252;
        _448 = vec4(0.0);
        float _447;
        float _445;
        vec4 _449;
        for (int _450 = 0; _450 < 2; _255 = _445, _253 = _447, _448 = _449, _450++)
        {
            float _457 = (float(_450) + _244) - 1.0;
            float _458 = abs(_457);
            _447 = _253 + _458;
            if (_156[_450] == 0u)
            {
                _445 = _255;
                _449 = _448;
                continue;
            }
            bool _464 = _457 >= 0.0;
            float _532;
            float _533;
            uint _534;
            if (_464)
            {
                uint _499 = uint(findLSB(_156[_450]));
                uint _503 = uint(max(0, (int(_499) - 1)));
                uint _505 = _499 >> 1u;
                vec2 _158 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_505] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_505] >> 16u) & 65535u).x);
                uint _519 = _503 >> 1u;
                vec2 _159 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_519] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_519] >> 16u) & 65535u).x);
                _532 = _158[_499 & 1u];
                _533 = _159[_503 & 1u];
                _534 = _499;
            }
            else
            {
                uint _468 = uint(findMSB(_156[_450]));
                uint _470 = min(4u, (_468 + 1u));
                uint _472 = _468 >> 1u;
                vec2 _160 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_472] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_472] >> 16u) & 65535u).x);
                uint _486 = _470 >> 1u;
                vec2 _161 = vec2(unpackHalf2x16(ring_samples[gl_LocalInvocationIndex][_486] & 65535u).x, unpackHalf2x16((ring_samples[gl_LocalInvocationIndex][_486] >> 16u) & 65535u).x);
                _532 = _160[_468 & 1u];
                _533 = _161[_470 & 1u];
                _534 = _468;
            }
            float _540 = (float(int(_534 - 2u)) + _223) * _224;
            float _541 = _540 / _457;
            float _542 = _224 * (_464 ? 1.0 : (-1.0));
            float _545 = _540 - _542;
            float _546 = _533 - (_541 - (_542 / _457));
            float _550 = _458 * float(_156[_450] != 0u);
            float _555 = mix(_545, _540, clamp(_546 / (_546 + (-(_532 - _541))), 0.0, 1.0));
            float _561 = clamp(textureLod(sampler2D(coc_tex, sampler_nnc), _181 + (_313 * _555), 0.0).x, _322, _192);
            float _569 = log2(spvNMax(0.0, abs(_532) * 0.125)) * smoothstep(4.0, 0.0, _261 - _259);
            float _572 = subgroupShuffle(_569, gl_SubgroupInvocationID ^ 1u);
            float _573 = spvNMin(_569, _572);
            float _575 = subgroupShuffle(_573, gl_SubgroupInvocationID ^ 8u);
            if (_532 > _192)
            {
                _445 = _255;
                _449 = _448;
                continue;
            }
            float _581 = _546 * 0.100000001490116119384765625;
            bool _587;
            if (((_555 / _457) - _561) < _581)
            {
                _587 = (_561 - _532) < _581;
            }
            else
            {
                _587 = false;
            }
            float _599;
            float _600;
            if (_587)
            {
                _599 = _255;
                _600 = _550;
            }
            else
            {
                float _594 = smoothstep(0.0, 30.0, _533) * smoothstep(0.20000000298023223876953125 * _192, 0.0, _532);
                _599 = _255 + (_550 * _594);
                _600 = _550 * (1.0 - _594);
            }
            _445 = _599;
            _449 = _448 + (textureLod(sampler2D(color_tex, sampler_nnc), _181 + (_313 * (_587 ? _555 : _545)), spvNMin(_573, _575)) * _600);
        }
        _248 = _247 + _448;
    }
    float _614 = _254 * smoothstep(0.00999999977648258209228515625 * _247.w, spvNMax(9.9999997473787516355514526367188e-06, 0.20000000298023223876953125 * _247.w), _254);
    vec4 _675;
    SPIRV_CROSS_BRANCH
    if (int(_614 > 0.0) != 0)
    {
        float _621 = -_192;
        vec4 _623;
        _623 = vec4(0.0);
        vec4 _624;
        float _627;
        float _626 = _621;
        uint _628 = 0u;
        for (; _628 < 16u; _623 = _624, _626 = _627, _628++)
        {
            vec2 _639 = fract(fract((vec2(0.75487768650054931640625, 0.56984031200408935546875) * float(_628)) + vec2(0.5)) + _208.xy);
            float _641 = sqrt(_639.x);
            float _643 = _639.y * 6.283185482025146484375;
            vec2 _654 = _181 + (((vec2(cos(_643) * _641, sin(_643) * _641) * 1.0) / vec2(_Globals.color_tex_dims)) * _250);
            vec4 _657 = textureLod(sampler2D(coc_tex, sampler_nnc), _654, 0.0);
            float _658 = _657.x;
            if (_658 > (_626 - 10.0))
            {
                _624 = _623 + textureLod(sampler2D(color_tex, sampler_llc), _654, 0.0);
                _627 = spvNMax(_626, _658);
            }
            else
            {
                _624 = _623;
                _627 = _626;
            }
        }
        _675 = _247 + ((_623 / vec4(spvNMax(1.0, _623.w))) * _614);
    }
    else
    {
        _675 = _247;
    }
    vec4 _677 = _675 / vec4(_252);
    imageStore(img_output, ivec2(_174), vec4((_677 / vec4(spvNMax(1.0000000133514319600180897396058e-10, _677.w))).xyz, 1.0));
}

